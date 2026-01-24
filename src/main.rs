mod builtins;
mod db;
mod logger;
mod symbols;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use texter::core::text::Text;
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::*;
use tower_lsp_server::{Client, LanguageServer, LspService, Server};
use tree_sitter::{Parser, Tree};

use builtins::{
    AUTOCMD_EVENTS, BUILTIN_COMMANDS, BUILTIN_FUNCTIONS, BUILTIN_OPTIONS, BUILTIN_VARIABLES,
    EditorMode, HAS_FEATURES, MAP_OPTIONS,
};
use db::{HjklsDatabase, SourceFile};
use salsa::Setter;
use symbols::{
    SymbolKind, find_call_at_position, find_identifier_at_position, find_references,
    find_references_with_kind,
};

/// Completion context based on cursor position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompletionContext {
    /// Line start or command position -> Ex commands
    Command,
    /// After autocmd -> event names
    AutocmdEvent,
    /// After set/setlocal -> option names
    Option,
    /// After map command, typing <... -> map options
    MapOption,
    /// Inside has('...') -> feature names
    HasFeature,
    /// Expression/function call context -> functions and variables
    Function,
}

/// Document state holding text and syntax tree
struct Document {
    text: Text,
    tree: Tree,
}

/// LSP backend for Vim script
struct Backend {
    client: Client,
    parser: Mutex<Parser>,
    documents: Mutex<HashMap<Uri, Document>>,
    /// Workspace root directories
    workspace_roots: Arc<Mutex<Vec<PathBuf>>>,
    /// Salsa database for incremental computation
    salsa_db: Arc<Mutex<HjklsDatabase>>,
    /// Mapping from URI to salsa SourceFile
    source_files: Arc<Mutex<HashMap<String, SourceFile>>>,
    /// Whether workspace indexing is complete
    indexing_complete: Arc<AtomicBool>,
    /// Editor mode for filtering completions
    editor_mode: EditorMode,
    /// Vim runtime path for autoload resolution
    vimruntime: Option<PathBuf>,
}

impl Backend {
    fn new(client: Client, editor_mode: EditorMode, vimruntime: Option<PathBuf>) -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_vim::language())
            .expect("Error loading vim grammar");

        Self {
            client,
            parser: Mutex::new(parser),
            documents: Mutex::new(HashMap::new()),
            workspace_roots: Arc::new(Mutex::new(Vec::new())),
            salsa_db: Arc::new(Mutex::new(HjklsDatabase::default())),
            source_files: Arc::new(Mutex::new(HashMap::new())),
            indexing_complete: Arc::new(AtomicBool::new(false)),
            editor_mode,
            vimruntime,
        }
    }

    /// Get symbols for a document using salsa memoization
    fn get_symbols(&self, uri: &str, content: &str) -> Vec<symbols::Symbol> {
        let mut db = self.salsa_db.lock().unwrap();
        let mut source_files = self.source_files.lock().unwrap();

        let source_file = if let Some(sf) = source_files.get(uri) {
            // Update existing SourceFile if content changed
            if sf.content(&*db) != content {
                sf.set_content(&mut *db).to(content.to_string());
            }
            *sf
        } else {
            // Create new SourceFile
            let sf = SourceFile::new(&*db, uri.to_string(), content.to_string());
            source_files.insert(uri.to_string(), sf);
            sf
        };

        db::parse_symbols(&*db, source_file)
    }

    /// Set workspace roots from initialize params
    fn set_workspace_roots(&self, params: &InitializeParams) {
        let mut roots = self.workspace_roots.lock().unwrap();
        roots.clear();

        // Try workspace folders first (LSP 3.6+)
        if let Some(folders) = &params.workspace_folders {
            for folder in folders {
                if let Some(path) = folder.uri.to_file_path() {
                    roots.push(path.into_owned());
                }
            }
        }

        // Fall back to root_uri (deprecated but still used)
        #[allow(deprecated)]
        if roots.is_empty() {
            if let Some(uri) = &params.root_uri {
                if let Some(path) = uri.to_file_path() {
                    roots.push(path.into_owned());
                }
            }
        }
    }

    /// Collect warnings for autoload function calls that reference non-existent files
    fn collect_autoload_warnings(
        &self,
        tree: &Tree,
        source: &str,
        current_doc_uri: Option<&Uri>,
    ) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let mut cursor = tree.walk();

        self.collect_autoload_warnings_recursive(
            &mut cursor,
            source,
            current_doc_uri,
            &mut diagnostics,
        );

        diagnostics
    }

    fn collect_autoload_warnings_recursive(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        source: &str,
        current_doc_uri: Option<&Uri>,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        loop {
            let node = cursor.node();

            // Check if this is a call_expression
            if node.kind() == "call_expression" {
                // Get the function name (first child is usually the function reference)
                if let Some(func_node) = node.child(0) {
                    let func_name = func_node.utf8_text(source.as_bytes()).unwrap_or("");

                    // Check if it's an autoload function call (contains #)
                    if let Some(autoload_ref) = symbols::AutoloadRef::parse(func_name) {
                        // Check if the autoload file exists
                        if self
                            .find_autoload_file(&autoload_ref, current_doc_uri)
                            .is_none()
                        {
                            let start = func_node.start_position();
                            let end = func_node.end_position();
                            let expected_path = autoload_ref.to_file_path();

                            diagnostics.push(Diagnostic {
                                range: Range {
                                    start: Position {
                                        line: start.row as u32,
                                        character: start.column as u32,
                                    },
                                    end: Position {
                                        line: end.row as u32,
                                        character: end.column as u32,
                                    },
                                },
                                severity: Some(DiagnosticSeverity::WARNING),
                                source: Some("hjkls".to_string()),
                                message: format!("Autoload file not found: {}", expected_path),
                                ..Default::default()
                            });
                        }
                    }
                }
            }

            // Recurse into children
            if cursor.goto_first_child() {
                self.collect_autoload_warnings_recursive(
                    cursor,
                    source,
                    current_doc_uri,
                    diagnostics,
                );
                cursor.goto_parent();
            }

            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    /// Collect warnings for function calls with wrong number of arguments
    fn collect_arity_warnings(&self, tree: &Tree, source: &str, uri: &Uri) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let mut cursor = tree.walk();

        // Get user-defined symbols for this document
        let uri_str = uri.to_string();
        let symbols = self.get_symbols(&uri_str, source);

        Self::collect_arity_warnings_recursive(&mut cursor, source, &symbols, &mut diagnostics);

        diagnostics
    }

    fn collect_arity_warnings_recursive(
        cursor: &mut tree_sitter::TreeCursor,
        source: &str,
        symbols: &[symbols::Symbol],
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        loop {
            let node = cursor.node();

            // Check if this is a call_expression
            if node.kind() == "call_expression" {
                if let Some(func_node) = node.child(0) {
                    let func_name = func_node.utf8_text(source.as_bytes()).unwrap_or("");

                    // Skip autoload functions (handled separately) and empty names
                    if func_name.is_empty() || func_name.contains('#') {
                        // Continue to recurse but skip arity check
                    } else {
                        // Try to find signature - first check built-in functions
                        let signature = BUILTIN_FUNCTIONS
                            .iter()
                            .find(|f| f.name == func_name)
                            .map(|f| f.signature.to_string())
                            .or_else(|| {
                                // Then check user-defined functions
                                symbols
                                    .iter()
                                    .find(|s| {
                                        s.kind == symbols::SymbolKind::Function
                                            && s.full_name() == func_name
                                    })
                                    .and_then(|s| s.signature.clone())
                            });

                        if let Some(sig) = signature {
                            let (min_args, max_args) = get_param_count_range(&sig);
                            let actual_args = count_call_arguments(node, source);

                            let is_error = if actual_args < min_args {
                                Some(format!(
                                    "Too few arguments: {} requires at least {} argument(s), got {}",
                                    func_name, min_args, actual_args
                                ))
                            } else if let Some(max) = max_args {
                                if actual_args > max {
                                    Some(format!(
                                        "Too many arguments: {} accepts at most {} argument(s), got {}",
                                        func_name, max, actual_args
                                    ))
                                } else {
                                    None
                                }
                            } else {
                                None
                            };

                            if let Some(message) = is_error {
                                let start = func_node.start_position();
                                let end = node.end_position(); // Use whole call expression

                                diagnostics.push(Diagnostic {
                                    range: Range {
                                        start: Position {
                                            line: start.row as u32,
                                            character: start.column as u32,
                                        },
                                        end: Position {
                                            line: end.row as u32,
                                            character: end.column as u32,
                                        },
                                    },
                                    severity: Some(DiagnosticSeverity::WARNING),
                                    source: Some("hjkls".to_string()),
                                    message,
                                    ..Default::default()
                                });
                            }
                        }
                    }
                }
            }

            // Recurse into children
            if cursor.goto_first_child() {
                Self::collect_arity_warnings_recursive(cursor, source, symbols, diagnostics);
                cursor.goto_parent();
            }

            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    /// Collect warnings for scope violations (l: or a: used outside functions)
    fn collect_scope_violations(&self, tree: &Tree, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let root = tree.root_node();
        Self::collect_scope_violations_recursive(&root, source, false, &mut diagnostics);
        diagnostics
    }

    fn collect_scope_violations_recursive(
        node: &tree_sitter::Node,
        source: &str,
        inside_function: bool,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        // Check if we're entering a function definition
        let is_function = node.kind() == "function_definition";
        let in_func = inside_function || is_function;

        // Check for scoped identifiers with l: scope (e.g., let l:var = 1)
        if node.kind() == "scoped_identifier" {
            let mut cursor = node.walk();
            let children: Vec<_> = node.children(&mut cursor).collect();

            if let Some(scope_node) = children.iter().find(|c| c.kind() == "scope") {
                if let Ok(scope_text) = scope_node.utf8_text(source.as_bytes()) {
                    // l: is only valid inside functions
                    if scope_text == "l:" && !in_func {
                        let start = node.start_position();
                        let end = node.end_position();
                        let var_name = node.utf8_text(source.as_bytes()).unwrap_or("?");

                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position {
                                    line: start.row as u32,
                                    character: start.column as u32,
                                },
                                end: Position {
                                    line: end.row as u32,
                                    character: end.column as u32,
                                },
                            },
                            severity: Some(DiagnosticSeverity::WARNING),
                            source: Some("hjkls".to_string()),
                            message: format!(
                                "Scope violation: '{}' uses local scope (l:) outside of a function",
                                var_name
                            ),
                            ..Default::default()
                        });
                    }
                }
            }
        }

        // Check for a: scope usage outside functions
        // tree-sitter parses a:var as [argument] -> [a:] + [identifier]
        // or in some contexts as a standalone reference
        if node.kind() == "a:" && !in_func {
            // Find the full variable name by looking at the parent and siblings
            let parent = node.parent();
            let (start, end, var_name) = if let Some(parent) = parent {
                let text = parent.utf8_text(source.as_bytes()).unwrap_or("a:?");
                (parent.start_position(), parent.end_position(), text)
            } else {
                (node.start_position(), node.end_position(), "a:?")
            };

            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position {
                        line: start.row as u32,
                        character: start.column as u32,
                    },
                    end: Position {
                        line: end.row as u32,
                        character: end.column as u32,
                    },
                },
                severity: Some(DiagnosticSeverity::WARNING),
                source: Some("hjkls".to_string()),
                message: format!(
                    "Scope violation: '{}' uses argument scope (a:) outside of a function",
                    var_name
                ),
                ..Default::default()
            });
        }

        // Recurse into children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::collect_scope_violations_recursive(&child, source, in_func, diagnostics);
        }
    }

    /// Collect suspicious lint warnings (code that may behave unexpectedly)
    fn collect_suspicious_warnings(&self, tree: &Tree, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let root = tree.root_node();

        // normal_bang: prefer `normal!` over `normal`
        Self::collect_normal_bang_warnings_recursive(&root, source, &mut diagnostics);

        // match_case: prefer `=~#` or `=~?` over `=~`
        Self::collect_match_case_warnings_recursive(&root, source, &mut diagnostics);

        // autocmd_group: autocmd outside augroup is risky
        let _ =
            Self::collect_autocmd_group_warnings_recursive(&root, source, false, &mut diagnostics);

        // set_compatible: `set compatible` enables Vi-compatible mode (rarely intended)
        Self::collect_set_compatible_warnings_recursive(&root, source, &mut diagnostics);

        // vim9script_position: `vim9script` must be at the start of the file
        Self::collect_vim9script_position_warnings(&root, source, &mut diagnostics);

        diagnostics
    }

    /// Collect warnings for `normal` without `!` (should use `normal!`)
    fn collect_normal_bang_warnings_recursive(
        node: &tree_sitter::Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if node.kind() == "normal_statement" {
            // Check if there's a bang child node
            let mut cursor = node.walk();
            let has_bang = node.children(&mut cursor).any(|c| c.kind() == "bang");

            if !has_bang {
                let start = node.start_position();
                let end = node.end_position();
                let text = node.utf8_text(source.as_bytes()).unwrap_or("normal");

                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position {
                            line: start.row as u32,
                            character: start.column as u32,
                        },
                        end: Position {
                            line: end.row as u32,
                            character: end.column as u32,
                        },
                    },
                    severity: Some(DiagnosticSeverity::WARNING),
                    source: Some("hjkls".to_string()),
                    message: format!(
                        "Suspicious: '{}' uses `normal` without `!`. User mappings may interfere. Use `normal!` instead.",
                        text.trim()
                    ),
                    ..Default::default()
                });
            }
        }

        // Recurse into children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::collect_normal_bang_warnings_recursive(&child, source, diagnostics);
        }
    }

    /// Collect warnings for `=~` without case modifier (should use `=~#` or `=~?`)
    fn collect_match_case_warnings_recursive(
        node: &tree_sitter::Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if node.kind() == "binary_operation" {
            let mut cursor = node.walk();
            let children: Vec<_> = node.children(&mut cursor).collect();

            // Check if this is a =~ operation
            let has_match_op = children.iter().any(|c| c.kind() == "=~");

            if has_match_op {
                // Check if there's a match_case modifier
                let has_case_modifier = children.iter().any(|c| c.kind() == "match_case");

                if !has_case_modifier {
                    let start = node.start_position();
                    let end = node.end_position();
                    let text = node.utf8_text(source.as_bytes()).unwrap_or("=~");

                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: start.row as u32,
                                character: start.column as u32,
                            },
                            end: Position {
                                line: end.row as u32,
                                character: end.column as u32,
                            },
                        },
                        severity: Some(DiagnosticSeverity::WARNING),
                        source: Some("hjkls".to_string()),
                        message: format!(
                            "Suspicious: '{}' uses `=~` without case modifier. Behavior depends on 'ignorecase' option. Use `=~#` (case-sensitive) or `=~?` (case-insensitive) instead.",
                            text.trim()
                        ),
                        ..Default::default()
                    });
                }
            }
        }

        // Recurse into children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::collect_match_case_warnings_recursive(&child, source, diagnostics);
        }
    }

    /// Collect warnings for `autocmd` outside of `augroup`
    ///
    /// In tree-sitter-vim, augroup_statement and autocmd_statement are siblings,
    /// not parent-child. So we need to track state across siblings.
    fn collect_autocmd_group_warnings_recursive(
        node: &tree_sitter::Node,
        source: &str,
        inside_augroup: bool,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> bool {
        // If this is an augroup statement, update the state
        if node.kind() == "augroup_statement" {
            let mut cursor = node.walk();
            let children: Vec<_> = node.children(&mut cursor).collect();

            if let Some(name_node) = children.iter().find(|c| c.kind() == "augroup_name") {
                if let Ok(name) = name_node.utf8_text(source.as_bytes()) {
                    // Return new state: true if entering augroup, false if exiting (END)
                    return !name.eq_ignore_ascii_case("END");
                }
            }
            return inside_augroup;
        }

        // Check autocmd statements
        if node.kind() == "autocmd_statement" && !inside_augroup {
            let mut cursor = node.walk();
            let children: Vec<_> = node.children(&mut cursor).collect();

            // Check if this autocmd has an event list (actual autocmd registration)
            // autocmd! (with bang and no events) is just clearing, which is OK
            let has_events = children.iter().any(|c| c.kind() == "au_event_list");

            // Check if this autocmd has an inline group name (e.g., autocmd MyGroup BufRead ...)
            // This is valid even outside augroup block
            let has_inline_group = children.iter().any(|c| c.kind() == "augroup_name");

            if has_events && !has_inline_group {
                let start = node.start_position();
                let end = node.end_position();
                let text = node.utf8_text(source.as_bytes()).unwrap_or("autocmd");

                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position {
                            line: start.row as u32,
                            character: start.column as u32,
                        },
                        end: Position {
                            line: end.row as u32,
                            character: end.column as u32,
                        },
                    },
                    severity: Some(DiagnosticSeverity::WARNING),
                    source: Some("hjkls".to_string()),
                    message: format!(
                        "Suspicious: '{}' is defined outside of an augroup. This may cause duplicate autocmds on reload. Wrap in `augroup` with `autocmd!` to clear.",
                        text.lines().next().unwrap_or(text).trim()
                    ),
                    ..Default::default()
                });
            }
        }

        // Recurse into children, tracking augroup state across siblings
        let mut cursor = node.walk();
        let mut current_in_augroup = inside_augroup;
        for child in node.children(&mut cursor) {
            current_in_augroup = Self::collect_autocmd_group_warnings_recursive(
                &child,
                source,
                current_in_augroup,
                diagnostics,
            );
        }

        // Return the current state (for sibling propagation at parent level)
        inside_augroup
    }

    /// Collect warnings for `set compatible` / `set cp`
    ///
    /// Vi-compatible mode disables many Vim features and is rarely intended.
    fn collect_set_compatible_warnings_recursive(
        node: &tree_sitter::Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if node.kind() == "set_statement" {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "set_item" {
                    let mut item_cursor = child.walk();
                    for item_child in child.children(&mut item_cursor) {
                        // Look for option_name directly under set_item (not under no_option)
                        // set compatible -> set_item -> option_name
                        // set nocompatible -> set_item -> no_option -> option_name
                        if item_child.kind() == "option_name" {
                            if let Ok(opt_name) = item_child.utf8_text(source.as_bytes()) {
                                if opt_name == "compatible" || opt_name == "cp" {
                                    let start = node.start_position();
                                    let end = node.end_position();
                                    let text = node
                                        .utf8_text(source.as_bytes())
                                        .unwrap_or("set compatible");

                                    diagnostics.push(Diagnostic {
                                        range: Range {
                                            start: Position {
                                                line: start.row as u32,
                                                character: start.column as u32,
                                            },
                                            end: Position {
                                                line: end.row as u32,
                                                character: end.column as u32,
                                            },
                                        },
                                        severity: Some(DiagnosticSeverity::WARNING),
                                        source: Some("hjkls".to_string()),
                                        message: format!(
                                            "Suspicious: '{}' enables Vi-compatible mode, which disables many Vim features. Is this intended?",
                                            text.trim()
                                        ),
                                        ..Default::default()
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        // Recurse into children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::collect_set_compatible_warnings_recursive(&child, source, diagnostics);
        }
    }

    /// Collect warnings for `vim9script` not at the start of the file
    ///
    /// vim9script must be the very first statement in the file (even before comments).
    /// tree-sitter-vim parses vim9script as unknown_builtin_statement with
    /// unknown_command_name="vim" and arguments containing "9script".
    fn collect_vim9script_position_warnings(
        root: &tree_sitter::Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        // Only check script_file nodes
        if root.kind() != "script_file" {
            return;
        }

        let mut cursor = root.walk();
        let mut is_first_statement = true;

        for child in root.children(&mut cursor) {
            // Check if this is a vim9script declaration
            if child.kind() == "unknown_builtin_statement" {
                let mut child_cursor = child.walk();
                let children: Vec<_> = child.children(&mut child_cursor).collect();

                // Check for vim9script pattern: unknown_command_name="vim" + arguments="9script"
                let is_vim9script = children.iter().any(|c| {
                    c.kind() == "unknown_command_name"
                        && c.utf8_text(source.as_bytes()).unwrap_or("") == "vim"
                }) && children.iter().any(|c| {
                    c.kind() == "arguments"
                        && c.utf8_text(source.as_bytes())
                            .unwrap_or("")
                            .contains("9script")
                });

                if is_vim9script {
                    // vim9script found - warn if it's not the first statement
                    if !is_first_statement {
                        let start = child.start_position();
                        let end = child.end_position();

                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position {
                                    line: start.row as u32,
                                    character: start.column as u32,
                                },
                                end: Position {
                                    line: end.row as u32,
                                    character: end.column as u32,
                                },
                            },
                            severity: Some(DiagnosticSeverity::WARNING),
                            source: Some("hjkls".to_string()),
                            message: "Suspicious: `vim9script` must be at the very first line of the file.".to_string(),
                            ..Default::default()
                        });
                    }
                    // Stop checking after finding vim9script
                    return;
                }
            }

            // Any statement (including comments) before vim9script is a problem
            is_first_statement = false;
        }
    }

    /// Collect style hints (code style suggestions, DiagnosticSeverity::HINT)
    fn collect_style_hints(&self, tree: &Tree, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let root = tree.root_node();

        // double_dot: prefer `..` over `.` for string concatenation
        Self::collect_double_dot_hints_recursive(&root, source, &mut diagnostics);

        // function_bang: s: functions don't need `!`
        Self::collect_function_bang_hints_recursive(&root, source, &mut diagnostics);

        diagnostics
    }

    /// Collect hints for `.` string concatenation (should use `..`)
    fn collect_double_dot_hints_recursive(
        node: &tree_sitter::Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if node.kind() == "binary_operation" {
            let mut cursor = node.walk();
            let children: Vec<_> = node.children(&mut cursor).collect();

            // Check if this is a `.` concatenation (not `..`)
            // In tree-sitter-vim, the operator is a child node with kind "." or ".."
            let has_single_dot = children.iter().any(|c| c.kind() == ".");

            if has_single_dot {
                let start = node.start_position();
                let end = node.end_position();
                let text = node.utf8_text(source.as_bytes()).unwrap_or(".");

                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position {
                            line: start.row as u32,
                            character: start.column as u32,
                        },
                        end: Position {
                            line: end.row as u32,
                            character: end.column as u32,
                        },
                    },
                    severity: Some(DiagnosticSeverity::HINT),
                    source: Some("hjkls".to_string()),
                    message: format!(
                        "Style: '{}' uses `.` for string concatenation. Use `..` instead. In Vim9 script, `..` is required.",
                        text.trim()
                    ),
                    ..Default::default()
                });
            }
        }

        // Recurse into children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::collect_double_dot_hints_recursive(&child, source, diagnostics);
        }
    }

    /// Collect hints for `function!` with `s:` scope (bang is unnecessary)
    fn collect_function_bang_hints_recursive(
        node: &tree_sitter::Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if node.kind() == "function_definition" {
            let mut cursor = node.walk();
            let children: Vec<_> = node.children(&mut cursor).collect();

            // Check if function has bang (!)
            let has_bang = children.iter().any(|c| c.kind() == "bang");

            if has_bang {
                // Find function_declaration to get the function name
                if let Some(decl) = children.iter().find(|c| c.kind() == "function_declaration") {
                    let mut decl_cursor = decl.walk();
                    let decl_children: Vec<_> = decl.children(&mut decl_cursor).collect();

                    // Check if function name has s: scope
                    let is_script_local = decl_children.iter().any(|c| {
                        if c.kind() == "scoped_identifier" {
                            let mut scope_cursor = c.walk();
                            c.children(&mut scope_cursor).any(|sc| {
                                sc.kind() == "scope"
                                    && sc.utf8_text(source.as_bytes()).ok() == Some("s:")
                            })
                        } else {
                            false
                        }
                    });

                    if is_script_local {
                        let start = node.start_position();
                        // Get just the first line for cleaner message
                        let text = node.utf8_text(source.as_bytes()).unwrap_or("function!");
                        let first_line = text.lines().next().unwrap_or(text);

                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position {
                                    line: start.row as u32,
                                    character: start.column as u32,
                                },
                                end: Position {
                                    line: start.row as u32,
                                    character: start.column as u32 + first_line.len() as u32,
                                },
                            },
                            severity: Some(DiagnosticSeverity::HINT),
                            source: Some("hjkls".to_string()),
                            message: format!(
                                "Style: '{}' uses `function!` for script-local function. The `!` is unnecessary for `s:` functions.",
                                first_line.trim()
                            ),
                            ..Default::default()
                        });
                    }
                }
            }
        }

        // Recurse into children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::collect_function_bang_hints_recursive(&child, source, diagnostics);
        }
    }

    /// Collect warnings for undefined function calls.
    ///
    /// Checks:
    /// - Built-in functions (786 in BUILTIN_FUNCTIONS)
    /// - Script-local functions (s:) - must be defined in the same file
    /// - Global functions - checked in local symbols and workspace
    ///
    /// Skips:
    /// - Autoload functions (contain #) - handled by collect_autoload_warnings
    fn collect_undefined_function_warnings(
        &self,
        tree: &Tree,
        source: &str,
        uri: &Uri,
    ) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let mut cursor = tree.walk();

        // Get symbols from current document
        let uri_str = uri.to_string();
        let local_symbols = self.get_symbols(&uri_str, source);

        // Get all workspace functions (if indexing is complete)
        let workspace_functions: Vec<String> = if self.indexing_complete.load(Ordering::SeqCst) {
            let source_files = self.source_files.lock().unwrap();
            let db = self.salsa_db.lock().unwrap();
            source_files
                .iter()
                .filter(|(file_uri, _)| *file_uri != &uri_str)
                .flat_map(|(_, sf)| {
                    db::parse_symbols(&*db, *sf)
                        .iter()
                        .filter(|s| s.kind == symbols::SymbolKind::Function)
                        .filter(|s| {
                            // Only include global functions (not s:)
                            s.scope != symbols::VimScope::Script
                        })
                        .map(|s| s.full_name())
                        .collect::<Vec<_>>()
                })
                .collect()
        } else {
            vec![]
        };

        self.collect_undefined_function_warnings_recursive(
            &mut cursor,
            source,
            &local_symbols,
            &workspace_functions,
            &mut diagnostics,
        );

        diagnostics
    }

    fn collect_undefined_function_warnings_recursive(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        source: &str,
        local_symbols: &[symbols::Symbol],
        workspace_functions: &[String],
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        loop {
            let node = cursor.node();

            if node.kind() == "call_expression" {
                if let Some(func_node) = node.child(0) {
                    let func_name = func_node.utf8_text(source.as_bytes()).unwrap_or("");

                    // Skip empty names and autoload functions (handled separately)
                    if !func_name.is_empty() && !func_name.contains('#') {
                        let is_undefined = self.check_if_function_undefined(
                            func_name,
                            local_symbols,
                            workspace_functions,
                        );

                        if is_undefined {
                            let start = func_node.start_position();
                            let end = func_node.end_position();

                            diagnostics.push(Diagnostic {
                                range: Range {
                                    start: Position {
                                        line: start.row as u32,
                                        character: start.column as u32,
                                    },
                                    end: Position {
                                        line: end.row as u32,
                                        character: end.column as u32,
                                    },
                                },
                                severity: Some(DiagnosticSeverity::WARNING),
                                source: Some("hjkls".to_string()),
                                message: format!("Undefined function: {}", func_name),
                                ..Default::default()
                            });
                        }
                    }
                }
            }

            // Recurse into children
            if cursor.goto_first_child() {
                self.collect_undefined_function_warnings_recursive(
                    cursor,
                    source,
                    local_symbols,
                    workspace_functions,
                    diagnostics,
                );
                cursor.goto_parent();
            }

            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    /// Check if a function is undefined
    /// Returns true if the function should be reported as undefined
    fn check_if_function_undefined(
        &self,
        func_name: &str,
        local_symbols: &[symbols::Symbol],
        workspace_functions: &[String],
    ) -> bool {
        // Check built-in functions first
        if BUILTIN_FUNCTIONS.iter().any(|f| f.name == func_name) {
            return false;
        }

        // Script-local functions (s:Func) - must be in local symbols
        if func_name.starts_with("s:") {
            return !local_symbols
                .iter()
                .any(|s| s.kind == symbols::SymbolKind::Function && s.full_name() == func_name);
        }

        // Global functions with g: prefix
        if func_name.starts_with("g:") {
            // Check local symbols
            if local_symbols
                .iter()
                .any(|s| s.kind == symbols::SymbolKind::Function && s.full_name() == func_name)
            {
                return false;
            }
            // Check workspace
            return !workspace_functions.contains(&func_name.to_string());
        }

        // For all other functions (including lowercase not in built-ins),
        // check local symbols and workspace
        if local_symbols
            .iter()
            .any(|s| s.kind == symbols::SymbolKind::Function && s.full_name() == func_name)
        {
            return false;
        }

        // Check workspace
        !workspace_functions.contains(&func_name.to_string())
    }

    /// Collect folding ranges from tree-sitter AST
    fn collect_folding_ranges(node: &tree_sitter::Node, ranges: &mut Vec<FoldingRange>) {
        // Node types that define foldable regions
        let foldable_kinds = [
            "function_definition",
            "if_statement",
            "for_loop",
            "while_loop",
            "try_statement",
            "augroup",
        ];

        // Check if current node is foldable
        if foldable_kinds.contains(&node.kind()) {
            let start_line = node.start_position().row as u32;
            let end_line = node.end_position().row as u32;

            // Only create fold if it spans multiple lines
            if end_line > start_line {
                ranges.push(FoldingRange {
                    start_line,
                    start_character: None,
                    end_line,
                    end_character: None,
                    kind: Some(FoldingRangeKind::Region),
                    collapsed_text: None,
                });
            }
        }

        // Recurse into children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::collect_folding_ranges(&child, ranges);
        }
    }

    /// Build a SelectionRange chain from the innermost node to the root
    fn build_selection_range(
        tree: &tree_sitter::Tree,
        position: &Position,
    ) -> Option<SelectionRange> {
        let point = tree_sitter::Point {
            row: position.line as usize,
            column: position.character as usize,
        };

        // Get the smallest named node at the position
        let mut node = tree
            .root_node()
            .named_descendant_for_point_range(point, point)?;

        // Collect ranges from innermost to outermost
        let mut ranges: Vec<Range> = Vec::new();

        loop {
            let range = Range {
                start: Position {
                    line: node.start_position().row as u32,
                    character: node.start_position().column as u32,
                },
                end: Position {
                    line: node.end_position().row as u32,
                    character: node.end_position().column as u32,
                },
            };

            // Skip duplicate ranges (when parent has same range as child)
            if ranges.last().is_none_or(|last| *last != range) {
                ranges.push(range);
            }

            match node.parent() {
                Some(parent) => node = parent,
                None => break,
            }
        }

        // Build linked list from outermost to innermost
        let mut result: Option<SelectionRange> = None;
        for range in ranges.into_iter().rev() {
            result = Some(SelectionRange {
                range,
                parent: result.map(Box::new),
            });
        }

        result
    }

    /// Find autoload file in workspace or relative to a document
    fn find_autoload_file(
        &self,
        autoload_ref: &symbols::AutoloadRef,
        current_doc_uri: Option<&Uri>,
    ) -> Option<PathBuf> {
        let relative_path = autoload_ref.to_file_path();

        // First, try relative to the current document's directory
        // This handles cases where autoload/ is in a subdirectory (e.g., test/)
        if let Some(uri) = current_doc_uri {
            if let Some(doc_path) = uri.to_file_path() {
                if let Some(doc_dir) = doc_path.parent() {
                    let full_path = doc_dir.join(&relative_path);
                    if full_path.exists() {
                        return Some(full_path);
                    }
                }
            }
        }

        // Then, try workspace roots
        let roots = self.workspace_roots.lock().unwrap();
        for root in roots.iter() {
            let full_path = root.join(&relative_path);
            if full_path.exists() {
                return Some(full_path);
            }
        }

        // Finally, try $VIMRUNTIME
        if let Some(runtime) = &self.vimruntime {
            let full_path = runtime.join(&relative_path);
            if full_path.exists() {
                return Some(full_path);
            }
        }

        None
    }

    /// Parse text and return tree
    fn parse(&self, text: &str, old_tree: Option<&Tree>) -> Option<Tree> {
        let mut parser = self.parser.lock().unwrap();
        parser.parse(text, old_tree)
    }

    /// Open a new document
    fn open_document(&self, uri: Uri, content: String) -> Vec<Diagnostic> {
        // Use UTF-16 encoding for VSCode compatibility
        // TODO: Detect client encoding from capabilities
        // Guard against empty content - texter panics if row count becomes 0
        let content = if content.is_empty() {
            "\n".to_string()
        } else {
            content
        };
        let text = Text::new_utf16(content);
        let tree = match self.parse(&text.text, None) {
            Some(t) => t,
            None => return vec![],
        };

        // Collect syntax errors
        let mut diagnostics = {
            let mut diags = vec![];
            let mut cursor = tree.walk();
            collect_errors(&mut cursor, &text.text, &mut diags);
            diags
        };

        // Collect autoload warnings
        let autoload_warnings = self.collect_autoload_warnings(&tree, &text.text, Some(&uri));
        diagnostics.extend(autoload_warnings);

        // Collect arity warnings (argument count mismatch)
        let arity_warnings = self.collect_arity_warnings(&tree, &text.text, &uri);
        diagnostics.extend(arity_warnings);

        // Collect scope violation warnings (l: or a: outside functions)
        let scope_warnings = self.collect_scope_violations(&tree, &text.text);
        diagnostics.extend(scope_warnings);

        // Collect undefined function warnings
        let undefined_warnings = self.collect_undefined_function_warnings(&tree, &text.text, &uri);
        diagnostics.extend(undefined_warnings);

        // Collect suspicious lint warnings
        let suspicious_warnings = self.collect_suspicious_warnings(&tree, &text.text);
        diagnostics.extend(suspicious_warnings);

        // Collect style hints
        let style_hints = self.collect_style_hints(&tree, &text.text);
        diagnostics.extend(style_hints);

        let mut docs = self.documents.lock().unwrap();
        docs.insert(uri, Document { text, tree });

        diagnostics
    }

    /// Update document with full replacement
    /// Note: We recreate the document instead of using incremental update
    /// because texter's internal state can become corrupted after certain
    /// operations (like undo after rename), causing panics in eol_indexes.
    fn update_document(&self, uri: &Uri, content: String) -> Vec<Diagnostic> {
        // Guard against empty content - texter panics if row count becomes 0
        let content = if content.is_empty() {
            "\n".to_string()
        } else {
            content
        };

        // Recreate document from scratch to avoid texter state corruption
        let text = Text::new_utf16(content);
        let tree = match self.parse(&text.text, None) {
            Some(t) => t,
            None => return vec![],
        };

        // Collect syntax errors
        let mut diagnostics = {
            let mut diags = vec![];
            let mut cursor = tree.walk();
            collect_errors(&mut cursor, &text.text, &mut diags);
            diags
        };

        // Collect autoload warnings
        let autoload_warnings = self.collect_autoload_warnings(&tree, &text.text, Some(uri));
        diagnostics.extend(autoload_warnings);

        // Collect arity warnings (argument count mismatch)
        let arity_warnings = self.collect_arity_warnings(&tree, &text.text, uri);
        diagnostics.extend(arity_warnings);

        // Collect scope violation warnings (l: or a: outside functions)
        let scope_warnings = self.collect_scope_violations(&tree, &text.text);
        diagnostics.extend(scope_warnings);

        // Collect undefined function warnings
        let undefined_warnings = self.collect_undefined_function_warnings(&tree, &text.text, uri);
        diagnostics.extend(undefined_warnings);

        // Collect suspicious lint warnings
        let suspicious_warnings = self.collect_suspicious_warnings(&tree, &text.text);
        diagnostics.extend(suspicious_warnings);

        // Collect style hints
        let style_hints = self.collect_style_hints(&tree, &text.text);
        diagnostics.extend(style_hints);

        let mut docs = self.documents.lock().unwrap();
        docs.insert(uri.clone(), Document { text, tree });

        diagnostics
    }

    /// Build Ex command completions
    fn build_command_completions(&self, edit_range: Range) -> Vec<CompletionItem> {
        BUILTIN_COMMANDS
            .iter()
            .filter(|cmd| cmd.availability.is_compatible(self.editor_mode))
            .map(|cmd| {
                let label_suffix = cmd.availability.label_suffix();
                let documentation = if label_suffix.is_empty() {
                    cmd.description.to_string()
                } else {
                    format!("{}\n{}", label_suffix.trim(), cmd.description)
                };
                CompletionItem {
                    label: cmd.name.to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    documentation: Some(Documentation::String(documentation)),
                    text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                        range: edit_range,
                        new_text: cmd.name.to_string(),
                    })),
                    ..Default::default()
                }
            })
            .collect()
    }

    /// Build autocmd event completions
    fn build_autocmd_event_completions(&self, edit_range: Range) -> Vec<CompletionItem> {
        AUTOCMD_EVENTS
            .iter()
            .filter(|event| event.availability.is_compatible(self.editor_mode))
            .map(|event| {
                let label_suffix = event.availability.label_suffix();
                let documentation = if label_suffix.is_empty() {
                    event.description.to_string()
                } else {
                    format!("{}\n{}", label_suffix.trim(), event.description)
                };
                CompletionItem {
                    label: event.name.to_string(),
                    kind: Some(CompletionItemKind::EVENT),
                    documentation: Some(Documentation::String(documentation)),
                    text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                        range: edit_range,
                        new_text: event.name.to_string(),
                    })),
                    ..Default::default()
                }
            })
            .collect()
    }

    /// Build option completions
    fn build_option_completions(&self, edit_range: Range, _line: &str) -> Vec<CompletionItem> {
        BUILTIN_OPTIONS
            .iter()
            .filter(|opt| opt.availability.is_compatible(self.editor_mode))
            .flat_map(|opt| {
                let label_suffix = opt.availability.label_suffix();
                let documentation = if label_suffix.is_empty() {
                    opt.description.to_string()
                } else {
                    format!("{}\n{}", label_suffix.trim(), opt.description)
                };

                let mut items = vec![CompletionItem {
                    label: opt.name.to_string(),
                    kind: Some(CompletionItemKind::PROPERTY),
                    detail: opt.short.map(|s| format!("short: {}", s)),
                    documentation: Some(Documentation::String(documentation.clone())),
                    text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                        range: edit_range,
                        new_text: opt.name.to_string(),
                    })),
                    ..Default::default()
                }];

                // Also add short form if available
                if let Some(short) = opt.short {
                    items.push(CompletionItem {
                        label: short.to_string(),
                        kind: Some(CompletionItemKind::PROPERTY),
                        detail: Some(format!("long: {}", opt.name)),
                        documentation: Some(Documentation::String(documentation)),
                        text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                            range: edit_range,
                            new_text: short.to_string(),
                        })),
                        ..Default::default()
                    });
                }

                items
            })
            .collect()
    }

    /// Build map option completions
    fn build_map_option_completions(&self, edit_range: Range) -> Vec<CompletionItem> {
        MAP_OPTIONS
            .iter()
            .map(|opt| CompletionItem {
                label: opt.name.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                documentation: Some(Documentation::String(opt.description.to_string())),
                text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                    range: edit_range,
                    new_text: opt.name.to_string(),
                })),
                ..Default::default()
            })
            .collect()
    }

    /// Build has() feature completions
    fn build_has_feature_completions(&self, edit_range: Range) -> Vec<CompletionItem> {
        HAS_FEATURES
            .iter()
            .filter(|feat| feat.availability.is_compatible(self.editor_mode))
            .map(|feat| {
                let label_suffix = feat.availability.label_suffix();
                let documentation = if label_suffix.is_empty() {
                    feat.description.to_string()
                } else {
                    format!("{}\n{}", label_suffix.trim(), feat.description)
                };
                CompletionItem {
                    label: feat.name.to_string(),
                    kind: Some(CompletionItemKind::CONSTANT),
                    documentation: Some(Documentation::String(documentation)),
                    text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                        range: edit_range,
                        new_text: feat.name.to_string(),
                    })),
                    ..Default::default()
                }
            })
            .collect()
    }

    /// Build function/variable completions (original behavior)
    fn build_function_completions(
        &self,
        edit_range: Range,
        uri_str: &str,
        content: &str,
        input_has_scope: bool,
    ) -> Vec<CompletionItem> {
        // 1. Built-in functions (filtered by editor mode, with availability labels)
        let mut items: Vec<CompletionItem> = BUILTIN_FUNCTIONS
            .iter()
            .filter(|func| func.availability.is_compatible(self.editor_mode))
            .map(|func| {
                let label_suffix = func.availability.label_suffix();
                let documentation = if label_suffix.is_empty() {
                    func.description.to_string()
                } else {
                    format!("{}\n{}", label_suffix.trim(), func.description)
                };
                CompletionItem {
                    label: func.name.to_string(),
                    kind: Some(CompletionItemKind::FUNCTION),
                    detail: Some(func.signature.to_string()),
                    documentation: Some(Documentation::String(documentation)),
                    text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                        range: edit_range,
                        new_text: func.name.to_string(),
                    })),
                    ..Default::default()
                }
            })
            .collect();

        // 2. User-defined symbols from current document
        let symbols = self.get_symbols(uri_str, content);
        for sym in symbols {
            // Skip parameters and empty names
            if sym.kind == SymbolKind::Parameter || sym.name.is_empty() {
                continue;
            }
            let kind = match sym.kind {
                SymbolKind::Function => CompletionItemKind::FUNCTION,
                SymbolKind::Variable => CompletionItemKind::VARIABLE,
                SymbolKind::Parameter => continue,
            };
            let detail = sym.signature.clone().or_else(|| {
                if sym.kind == SymbolKind::Variable {
                    Some(format!(
                        "{} variable",
                        sym.scope.as_str().trim_end_matches(':')
                    ))
                } else {
                    None
                }
            });
            let full_name = sym.full_name();
            let has_scope = !sym.scope.as_str().is_empty();

            let filter_text = if has_scope && !input_has_scope {
                Some(sym.name.clone())
            } else {
                None
            };

            items.push(CompletionItem {
                label: full_name.clone(),
                filter_text,
                kind: Some(kind),
                detail,
                text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                    range: edit_range,
                    new_text: full_name,
                })),
                ..Default::default()
            });
        }

        // 3. Built-in variables (v:, b: scope)
        for var in BUILTIN_VARIABLES
            .iter()
            .filter(|v| v.availability.is_compatible(self.editor_mode))
        {
            let label_suffix = var.availability.label_suffix();
            let documentation = if label_suffix.is_empty() {
                var.description.to_string()
            } else {
                format!("{}\n{}", label_suffix.trim(), var.description)
            };
            items.push(CompletionItem {
                label: var.name.to_string(),
                kind: Some(CompletionItemKind::VARIABLE),
                detail: Some("predefined variable".to_string()),
                documentation: Some(Documentation::String(documentation)),
                text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                    range: edit_range,
                    new_text: var.name.to_string(),
                })),
                ..Default::default()
            });
        }

        items
    }
}

/// Background workspace indexing function
fn index_workspace_background(
    workspace_roots: Arc<Mutex<Vec<PathBuf>>>,
    salsa_db: Arc<Mutex<HjklsDatabase>>,
    source_files: Arc<Mutex<HashMap<String, SourceFile>>>,
    indexing_complete: Arc<AtomicBool>,
) {
    // Scan for .vim files
    let vim_files: Vec<PathBuf> = {
        let roots = workspace_roots.lock().unwrap();
        let mut files = Vec::new();
        for root in roots.iter() {
            scan_directory_recursive(root, &mut files);
        }
        files
    };

    let file_count = vim_files.len();
    log_debug!("indexing: starting, found {} .vim files", file_count);

    // Index each file
    for (i, path) in vim_files.iter().enumerate() {
        if let Ok(content) = std::fs::read_to_string(path) {
            let uri = path.to_string_lossy().to_string();

            let db = salsa_db.lock().unwrap();
            let mut sf_map = source_files.lock().unwrap();

            if !sf_map.contains_key(&uri) {
                let sf = SourceFile::new(&*db, uri.clone(), content);
                sf_map.insert(uri.clone(), sf);
                // Trigger symbol parsing to populate cache
                let _ = db::parse_symbols(&*db, sf);
            }
        }

        if (i + 1) % 50 == 0 {
            log_debug!("indexing: progress {}/{}", i + 1, file_count);
        }
    }

    indexing_complete.store(true, Ordering::SeqCst);
    log_debug!("indexing: complete, indexed {} files", file_count);
}

/// Recursively scan a directory for .vim files
fn scan_directory_recursive(dir: &PathBuf, files: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        // Skip hidden directories and common non-source directories
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.starts_with('.') || name == "node_modules" || name == "target" {
                continue;
            }
        }

        if path.is_dir() {
            scan_directory_recursive(&path, files);
        } else if path.extension().is_some_and(|ext| ext == "vim") {
            files.push(path);
        }
    }
}

/// Recursively collect ERROR nodes from the syntax tree
fn collect_errors(
    cursor: &mut tree_sitter::TreeCursor,
    source: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    loop {
        let node = cursor.node();

        if node.is_error() || node.is_missing() {
            let start = node.start_position();
            let end = node.end_position();

            let message = if node.is_missing() {
                format!("Missing: {}", node.kind())
            } else {
                let snippet: String = source
                    .lines()
                    .nth(start.row)
                    .map(|line| {
                        let start_col = start.column.min(line.len());
                        let end_col = if start.row == end.row {
                            end.column.min(line.len())
                        } else {
                            line.len()
                        };
                        line[start_col..end_col].to_string()
                    })
                    .unwrap_or_default();
                format!("Syntax error: unexpected `{}`", snippet.trim())
            };

            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position {
                        line: start.row as u32,
                        character: start.column as u32,
                    },
                    end: Position {
                        line: end.row as u32,
                        character: end.column as u32,
                    },
                },
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("hjkls".to_string()),
                message,
                ..Default::default()
            });
        }

        // Recurse into children
        if cursor.goto_first_child() {
            collect_errors(cursor, source, diagnostics);
            cursor.goto_parent();
        }

        // Move to next sibling
        if !cursor.goto_next_sibling() {
            break;
        }
    }
}

/// Find the start position of a completion token, including scope prefix.
/// For Vim script, this includes scope prefixes like s:, g:, l:, a:, b:, w:, t:, v:
/// e.g., for "call s:Priv|" (| is cursor), returns the position of 's'
fn find_completion_token_start(line: &str, cursor_col: usize) -> usize {
    let bytes: Vec<char> = line.chars().collect();
    let col = cursor_col.min(bytes.len());

    if col == 0 {
        return 0;
    }

    // First, find the start of the identifier part (alphanumeric, underscore, #)
    let mut start = col;
    while start > 0 {
        let ch = bytes[start - 1];
        if ch.is_alphanumeric() || ch == '_' || ch == '#' {
            start -= 1;
        } else {
            break;
        }
    }

    // Then, check if there's a scope prefix (s:, g:, etc.) before the identifier
    if start >= 2 && bytes[start - 1] == ':' {
        let scope_char = bytes[start - 2];
        if matches!(scope_char, 's' | 'g' | 'l' | 'a' | 'b' | 'w' | 't' | 'v') {
            // Verify this is actually a scope prefix (not part of another word)
            if start < 3 || !bytes[start - 3].is_alphanumeric() {
                start -= 2; // Include the scope prefix
            }
        }
    }

    start
}

/// Parse parameter names from a function signature string
/// e.g., "substitute({string}, {pat}, {sub}, {flags})" -> ["{string}", "{pat}", "{sub}", "{flags}"]
fn parse_signature_params(signature: &str) -> Vec<String> {
    let mut params = Vec::new();

    // Find the content between parentheses
    let Some(start) = signature.find('(') else {
        return params;
    };
    let Some(end) = signature.rfind(')') else {
        return params;
    };

    if start + 1 >= end {
        return params;
    }

    let args_str = &signature[start + 1..end];

    // Split by comma, but handle nested brackets
    let mut depth = 0;
    let mut current_param = String::new();

    for ch in args_str.chars() {
        match ch {
            '[' | '{' => {
                depth += 1;
                current_param.push(ch);
            }
            ']' | '}' => {
                depth -= 1;
                current_param.push(ch);
            }
            ',' if depth == 0 => {
                let trimmed = current_param.trim().to_string();
                if !trimmed.is_empty() {
                    params.push(trimmed);
                }
                current_param.clear();
            }
            _ => {
                current_param.push(ch);
            }
        }
    }

    // Don't forget the last parameter
    let trimmed = current_param.trim().to_string();
    if !trimmed.is_empty() {
        params.push(trimmed);
    }

    params
}

/// Get the minimum and maximum argument count from a function signature
/// Returns (min_args, max_args) where max_args is None if unlimited (e.g., varargs)
fn get_param_count_range(signature: &str) -> (usize, Option<usize>) {
    // Check for varargs
    if signature.contains("...") {
        // Count required args before varargs
        let params = parse_signature_params(signature);
        let min_args = params
            .iter()
            .filter(|p| !p.trim().starts_with('[') && !p.contains('=') && !p.contains("..."))
            .count();
        return (min_args, None);
    }

    // Find the content between parentheses
    let Some(start) = signature.find('(') else {
        return (0, Some(0));
    };
    let Some(end) = signature.rfind(')') else {
        return (0, Some(0));
    };

    if start + 1 >= end {
        return (0, Some(0));
    }

    let args_str = &signature[start + 1..end];

    // Count required and optional arguments directly from signature string
    let mut min_args = 0;
    let mut max_args = 0;
    let mut in_optional = false;
    let mut depth = 0;
    let mut current_arg = String::new();

    for ch in args_str.chars() {
        match ch {
            '[' => {
                if depth == 0 {
                    in_optional = true;
                }
                depth += 1;
            }
            ']' => {
                depth -= 1;
            }
            '{' => {
                // Start of an argument like {pattern}
                current_arg.clear();
            }
            '}' => {
                // End of an argument - count it
                if !current_arg.is_empty() {
                    max_args += 1;
                    if !in_optional {
                        min_args += 1;
                    }
                }
                current_arg.clear();
            }
            ',' if depth == 0 => {
                // Top-level comma - check if current_arg has content (user-defined style)
                let trimmed = current_arg.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('{') {
                    // User-defined param like "name" or "name = 'default'"
                    max_args += 1;
                    if !trimmed.contains('=') {
                        min_args += 1;
                    }
                }
                current_arg.clear();
            }
            _ => {
                current_arg.push(ch);
            }
        }
    }

    // Handle last argument (user-defined style without braces)
    let trimmed = current_arg.trim();
    if !trimmed.is_empty() && !trimmed.starts_with('{') && !trimmed.starts_with('[') {
        max_args += 1;
        if !trimmed.contains('=') {
            min_args += 1;
        }
    }

    (min_args, Some(max_args))
}

/// Count the number of arguments in a call_expression node
fn count_call_arguments(node: tree_sitter::Node, _source: &str) -> usize {
    let mut count = 0;
    let mut cursor = node.walk();

    // Skip the function name (first child)
    if !cursor.goto_first_child() {
        return 0;
    }

    // Iterate through remaining children
    while cursor.goto_next_sibling() {
        let child = cursor.node();
        let kind = child.kind();
        // Count actual argument nodes (not punctuation)
        // Arguments can be various expression types
        if kind != "(" && kind != ")" && kind != "," {
            count += 1;
        }
    }

    count
}

/// Determine what kind of completion is appropriate based on cursor context
fn get_completion_context(line: &str, col: usize) -> CompletionContext {
    let before_cursor = &line[..col.min(line.len())];
    let trimmed = before_cursor.trim_start();

    // Empty line or only whitespace -> command context
    if trimmed.is_empty() {
        return CompletionContext::Command;
    }

    // Check for specific command patterns
    // autocmd [group] EVENT -> autocmd event completion
    if let Some(rest) = trimmed
        .strip_prefix("autocmd")
        .or_else(|| trimmed.strip_prefix("au "))
    {
        let rest = rest.trim_start();
        // Skip optional group name (if it doesn't look like an event)
        // Events are typically CamelCase, groups can be anything
        let parts: Vec<&str> = rest.split_whitespace().collect();
        // If we have 0 or 1 parts, we're likely typing the event (or group then event)
        if parts.len() <= 1 {
            return CompletionContext::AutocmdEvent;
        }
        // If 2+ parts, check if cursor is still in the "event" area
        // This is a simplification - we assume event comes after optional group
    }

    // set/setlocal/setglobal OPTION -> option completion
    if trimmed.starts_with("set ")
        || trimmed.starts_with("setlocal ")
        || trimmed.starts_with("setglobal ")
        || trimmed.starts_with("se ")
        || trimmed.starts_with("setl ")
        || trimmed.starts_with("setg ")
    {
        return CompletionContext::Option;
    }

    // map commands with < suggesting map option
    let map_commands = [
        "map", "nmap", "vmap", "xmap", "smap", "imap", "cmap", "omap", "lmap", "tmap", "noremap",
        "nnoremap", "vnoremap", "xnoremap", "snoremap", "inoremap", "cnoremap", "onoremap",
        "lnoremap", "tnoremap",
    ];
    for cmd in &map_commands {
        if let Some(rest) = trimmed.strip_prefix(cmd) {
            if rest.starts_with(' ') || rest.is_empty() {
                let rest = rest.trim_start();
                // If typing <... it's a map option
                if rest.ends_with('<')
                    || rest
                        .split_whitespace()
                        .last()
                        .is_some_and(|s| s.starts_with('<'))
                {
                    return CompletionContext::MapOption;
                }
            }
        }
    }

    // has('... -> feature completion
    if before_cursor.contains("has('") || before_cursor.contains("has(\"") {
        // Check if we're inside the has() call
        let last_has = before_cursor.rfind("has(");
        if let Some(pos) = last_has {
            let after_has = &before_cursor[pos..];
            // If there's an opening quote but no closing quote, we're inside
            if (after_has.contains('\'') && after_has.matches('\'').count() == 1)
                || (after_has.contains('"') && after_has.matches('"').count() == 1)
            {
                return CompletionContext::HasFeature;
            }
        }
    }

    // Check if line starts with a command (no = or function call pattern)
    // This is a heuristic: if the line doesn't have = and doesn't look like an expression
    let first_word = trimmed.split_whitespace().next().unwrap_or("");
    if !trimmed.contains('=') && !trimmed.contains('(') && !first_word.is_empty() {
        // If typing the first word, it's likely a command
        let first_word_end = trimmed.find(char::is_whitespace).unwrap_or(trimmed.len());
        if col <= before_cursor.len() - trimmed.len() + first_word_end {
            return CompletionContext::Command;
        }
    }

    // Default: function/expression context
    CompletionContext::Function
}

impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        // Capture workspace roots for cross-file features
        self.set_workspace_roots(&params);

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        will_save: None,
                        will_save_wait_until: None,
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(true),
                        })),
                    },
                )),
                completion_provider: Some(CompletionOptions::default()),
                signature_help_provider: Some(SignatureHelpOptions {
                    trigger_characters: Some(vec!["(".to_string(), ",".to_string()]),
                    retrigger_characters: None,
                    work_done_progress_options: Default::default(),
                }),
                definition_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                references_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Right(RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress_options: Default::default(),
                })),
                document_highlight_provider: Some(OneOf::Left(true)),
                folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
                selection_range_provider: Some(SelectionRangeProviderCapability::Simple(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "hjkls initialized!")
            .await;

        // Start background indexing
        let workspace_roots = Arc::clone(&self.workspace_roots);
        let salsa_db = Arc::clone(&self.salsa_db);
        let source_files = Arc::clone(&self.source_files);
        let indexing_complete = Arc::clone(&self.indexing_complete);

        std::thread::spawn(move || {
            index_workspace_background(workspace_roots, salsa_db, source_files, indexing_complete);
        });
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;

        let diagnostics = self.open_document(uri.clone(), text);
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        // We use FULL sync, so take the last change
        let Some(change) = params.content_changes.into_iter().last() else {
            return;
        };

        log_debug!(
            "did_change: len={}, lines={}, empty={}",
            change.text.len(),
            change.text.lines().count(),
            change.text.is_empty()
        );

        let diagnostics = self.update_document(&uri, change.text);
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let mut docs = self.documents.lock().unwrap();
        docs.remove(&params.text_document.uri);
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        // Update the salsa index when a file is saved
        let uri = params.text_document.uri;
        let uri_str = uri.to_string();

        // Get the saved content (if include_text is enabled)
        let content = if let Some(text) = params.text {
            text
        } else {
            // Fall back to reading from the document store
            let docs = self.documents.lock().unwrap();
            if let Some(doc) = docs.get(&uri) {
                doc.text.text.clone()
            } else {
                return;
            }
        };

        // Update the salsa cache
        let _ = self.get_symbols(&uri_str, &content);
        log_debug!("did_save: updated index for {}", uri_str);
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        // Get document content and determine completion context
        let (uri_str, content, token_start, input_has_scope, context, line_text) = {
            let docs = self.documents.lock().unwrap();
            let Some(doc) = docs.get(&uri) else {
                return Ok(Some(CompletionResponse::Array(vec![])));
            };
            let content = doc.text.text.clone();

            // Find token start position (including scope prefix like s:, g:)
            let line = content
                .lines()
                .nth(position.line as usize)
                .unwrap_or("")
                .to_string();
            let col = position.character as usize;
            let token_start = find_completion_token_start(&line, col);

            // Check if current input contains a scope prefix (e.g., "g:", "s:")
            let current_input = &line[token_start..col.min(line.len())];
            let input_has_scope = current_input.contains(':');

            // Determine completion context based on cursor position
            let context = get_completion_context(&line, col);

            (
                uri.to_string(),
                content,
                token_start,
                input_has_scope,
                context,
                line,
            )
        };

        // Create text edit range for replacing the current token
        let edit_range = Range {
            start: Position {
                line: position.line,
                character: token_start as u32,
            },
            end: position,
        };

        // Build completions based on context
        let items: Vec<CompletionItem> = match context {
            CompletionContext::Command => {
                // Ex commands completion
                self.build_command_completions(edit_range)
            }
            CompletionContext::AutocmdEvent => {
                // Autocmd event completion
                self.build_autocmd_event_completions(edit_range)
            }
            CompletionContext::Option => {
                // Option completion
                self.build_option_completions(edit_range, &line_text)
            }
            CompletionContext::MapOption => {
                // Map option completion
                self.build_map_option_completions(edit_range)
            }
            CompletionContext::HasFeature => {
                // has() feature completion
                self.build_has_feature_completions(edit_range)
            }
            CompletionContext::Function => {
                // Function/expression context - original behavior
                self.build_function_completions(edit_range, &uri_str, &content, input_has_scope)
            }
        };

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn signature_help(&self, params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        log_debug!(
            "signature_help: position={}:{}",
            position.line,
            position.character
        );

        let docs = self.documents.lock().unwrap();
        let Some(doc) = docs.get(&uri) else {
            log_debug!("signature_help: document not found");
            return Ok(None);
        };

        // Find the function call at cursor position
        let call_info = find_call_at_position(
            &doc.tree,
            &doc.text.text,
            position.line as usize,
            position.character as usize,
        );

        let Some(call_info) = call_info else {
            log_debug!("signature_help: no call found at position");
            return Ok(None);
        };

        log_debug!(
            "signature_help: found call '{}', param={}",
            call_info.function_name,
            call_info.active_param
        );

        // First, check if it's a built-in function
        if let Some(builtin) = BUILTIN_FUNCTIONS
            .iter()
            .find(|f| f.name == call_info.function_name)
        {
            let params = parse_signature_params(builtin.signature);
            let parameters: Vec<ParameterInformation> = params
                .iter()
                .map(|p| ParameterInformation {
                    label: ParameterLabel::Simple(p.clone()),
                    documentation: None,
                })
                .collect();

            let signature = SignatureInformation {
                label: builtin.signature.to_string(),
                documentation: Some(Documentation::String(builtin.description.to_string())),
                parameters: Some(parameters),
                active_parameter: Some(call_info.active_param as u32),
            };

            return Ok(Some(SignatureHelp {
                signatures: vec![signature],
                active_signature: Some(0),
                active_parameter: Some(call_info.active_param as u32),
            }));
        }

        // Then, check user-defined functions
        let uri_str = uri.to_string();
        let content = doc.text.text.clone();
        drop(docs);

        let symbols = self.get_symbols(&uri_str, &content);

        // Look for matching function (handle both scoped and autoload)
        let symbol = symbols.iter().find(|s| {
            s.kind == SymbolKind::Function
                && (s.name == call_info.function_name
                    || s.full_name() == call_info.function_name
                    || call_info
                        .autoload
                        .as_ref()
                        .is_some_and(|a| a.full_name == s.name))
        });

        if let Some(symbol) = symbol {
            let sig_str = symbol
                .signature
                .clone()
                .unwrap_or_else(|| format!("{}()", symbol.full_name()));

            let params = parse_signature_params(&sig_str);
            let parameters: Vec<ParameterInformation> = params
                .iter()
                .map(|p| ParameterInformation {
                    label: ParameterLabel::Simple(p.clone()),
                    documentation: None,
                })
                .collect();

            let signature = SignatureInformation {
                label: sig_str,
                documentation: None,
                parameters: Some(parameters),
                active_parameter: Some(call_info.active_param as u32),
            };

            return Ok(Some(SignatureHelp {
                signatures: vec![signature],
                active_signature: Some(0),
                active_parameter: Some(call_info.active_param as u32),
            }));
        }

        Ok(None)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let docs = self.documents.lock().unwrap();
        let Some(doc) = docs.get(&uri) else {
            return Ok(None);
        };

        // Find the identifier at the cursor position
        let reference = find_identifier_at_position(
            &doc.tree,
            &doc.text.text,
            position.line as usize,
            position.character as usize,
        );

        let Some(reference) = reference else {
            return Ok(None);
        };

        // Check if this is an autoload function call
        if let Some(autoload_ref) = &reference.autoload {
            // Release the lock before doing file I/O
            drop(docs);

            log_debug!("goto_definition: autoload={}", autoload_ref.full_name);

            // Try to find the autoload file (search relative to current doc first)
            let Some(file_path) = self.find_autoload_file(autoload_ref, Some(&uri)) else {
                log_debug!(
                    "goto_definition: file not found for {}",
                    autoload_ref.to_file_path()
                );
                return Ok(None);
            };
            log_debug!("goto_definition: found {:?}", file_path);

            // Parse the file and find the function definition
            let content = match std::fs::read_to_string(&file_path) {
                Ok(c) => c,
                Err(_) => {
                    log_debug!("goto_definition: failed to read {:?}", file_path);
                    return Ok(None);
                }
            };

            let file_uri = file_path.to_string_lossy().to_string();
            let symbols = self.get_symbols(&file_uri, &content);
            log_debug!(
                "goto_definition: symbols={:?}",
                symbols.iter().map(|s| &s.name).collect::<Vec<_>>()
            );

            // Look for the function with matching name
            // Autoload files define functions with full name (e.g., myplugin#util#helper)
            let Some(symbol) = symbols
                .iter()
                .find(|s| s.kind == SymbolKind::Function && s.name == autoload_ref.full_name)
            else {
                log_debug!("goto_definition: no match for '{}'", autoload_ref.full_name);
                return Ok(None);
            };

            let Some(target_uri) = Uri::from_file_path(&file_path) else {
                log_debug!("goto_definition: invalid URI for {:?}", file_path);
                return Ok(None);
            };

            log_debug!("goto_definition: jumping to {:?}", target_uri);
            let location = Location {
                uri: target_uri,
                range: Range {
                    start: Position {
                        line: symbol.start.0 as u32,
                        character: symbol.start.1 as u32,
                    },
                    end: Position {
                        line: symbol.end.0 as u32,
                        character: symbol.end.1 as u32,
                    },
                },
            };
            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
        }

        // Extract symbols and find the definition in current file
        let uri_str = uri.to_string();
        let content = doc.text.text.clone();
        drop(docs); // Release lock before calling get_symbols

        let symbols = self.get_symbols(&uri_str, &content);

        // Find matching symbol definition
        let definition = symbols.iter().find(|s| {
            s.name == reference.name
                && (reference.scope == symbols::VimScope::Implicit || s.scope == reference.scope)
                && (reference.is_call == (s.kind == SymbolKind::Function)
                    || !reference.is_call && s.kind == SymbolKind::Variable)
        });

        if let Some(symbol) = definition {
            let location = Location {
                uri: uri.clone(),
                range: Range {
                    start: Position {
                        line: symbol.start.0 as u32,
                        character: symbol.start.1 as u32,
                    },
                    end: Position {
                        line: symbol.end.0 as u32,
                        character: symbol.end.1 as u32,
                    },
                },
            };
            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
        }

        Ok(None)
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let docs = self.documents.lock().unwrap();
        let Some(doc) = docs.get(&uri) else {
            return Ok(None);
        };

        // Find the identifier at the cursor position
        let reference = find_identifier_at_position(
            &doc.tree,
            &doc.text.text,
            position.line as usize,
            position.character as usize,
        );

        let Some(reference) = reference else {
            return Ok(None);
        };

        // First, check if it's an autoload function
        if let Some(autoload) = &reference.autoload {
            let contents = format!(
                "```vim\n{}()\n```\n\n*autoload function*\n\nExpected file: `{}`",
                autoload.full_name,
                autoload.to_file_path()
            );
            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: contents,
                }),
                range: None,
            }));
        }

        // Then, check if it's a built-in function
        if reference.is_call {
            if let Some(builtin) = BUILTIN_FUNCTIONS.iter().find(|f| f.name == reference.name) {
                let contents = format!(
                    "```vim\n{}\n```\n\n{}",
                    builtin.signature, builtin.description
                );
                return Ok(Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: contents,
                    }),
                    range: None,
                }));
            }
        }

        // Then, check user-defined symbols
        let uri_str = uri.to_string();
        let content = doc.text.text.clone();
        drop(docs); // Release lock before calling get_symbols

        let symbols = self.get_symbols(&uri_str, &content);
        let symbol = symbols.iter().find(|s| {
            s.name == reference.name
                && (reference.scope == symbols::VimScope::Implicit || s.scope == reference.scope)
        });

        if let Some(symbol) = symbol {
            let kind_str = match symbol.kind {
                SymbolKind::Function => "function",
                SymbolKind::Variable => "variable",
                SymbolKind::Parameter => "parameter",
            };

            let contents = if let Some(sig) = &symbol.signature {
                format!("```vim\n{}\n```\n\n*{}*", sig, kind_str)
            } else {
                format!(
                    "```vim\n{}{}\n```\n\n*{}*",
                    symbol.scope.as_str(),
                    symbol.name,
                    kind_str
                )
            };

            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: contents,
                }),
                range: None,
            }));
        }

        Ok(None)
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let start_time = std::time::Instant::now();

        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let include_declaration = params.context.include_declaration;

        let docs = self.documents.lock().unwrap();
        let Some(doc) = docs.get(&uri) else {
            return Ok(None);
        };

        // Find the identifier at the cursor position
        let reference = find_identifier_at_position(
            &doc.tree,
            &doc.text.text,
            position.line as usize,
            position.character as usize,
        );

        let Some(reference) = reference else {
            return Ok(None);
        };

        // Find all references in the current file
        let current_file_locations = find_references(
            &doc.tree,
            &doc.text.text,
            &reference.name,
            reference.scope,
            include_declaration,
        );

        // Release the documents lock before searching other files
        drop(docs);

        let mut result: Vec<Location> = current_file_locations
            .into_iter()
            .map(|loc| Location {
                uri: uri.clone(),
                range: Range {
                    start: Position {
                        line: loc.start.0 as u32,
                        character: loc.start.1 as u32,
                    },
                    end: Position {
                        line: loc.end.0 as u32,
                        character: loc.end.1 as u32,
                    },
                },
            })
            .collect();

        // Search in other indexed files if:
        // 1. Indexing is complete
        // 2. The symbol is visible across files (autoload or global scope)
        let is_cross_file_visible = reference.autoload.is_some()
            || reference.scope == symbols::VimScope::Global
            || reference.scope == symbols::VimScope::Implicit && reference.name.contains('#');

        if is_cross_file_visible && self.indexing_complete.load(Ordering::SeqCst) {
            let current_uri_str = uri.to_string();
            let source_files = self.source_files.lock().unwrap();
            let db = self.salsa_db.lock().unwrap();

            for (file_uri, source_file) in source_files.iter() {
                // Skip the current file (already searched)
                if file_uri == &current_uri_str {
                    continue;
                }

                let content = source_file.content(&*db);

                // Parse the file to search for references
                let mut parser = tree_sitter::Parser::new();
                parser
                    .set_language(&tree_sitter_vim::language())
                    .expect("Error loading vim grammar");

                if let Some(tree) = parser.parse(&content, None) {
                    let locations = find_references(
                        &tree,
                        &content,
                        &reference.name,
                        reference.scope,
                        include_declaration,
                    );

                    for loc in locations {
                        // Convert file path to URI
                        if let Some(file_uri) = Uri::from_file_path(file_uri) {
                            result.push(Location {
                                uri: file_uri,
                                range: Range {
                                    start: Position {
                                        line: loc.start.0 as u32,
                                        character: loc.start.1 as u32,
                                    },
                                    end: Position {
                                        line: loc.end.0 as u32,
                                        character: loc.end.1 as u32,
                                    },
                                },
                            });
                        }
                    }
                }
            }
        }

        log_debug!(
            "references: found {} refs for '{}' in {:?}",
            result.len(),
            reference.name,
            start_time.elapsed()
        );

        if result.is_empty() {
            return Ok(None);
        }

        Ok(Some(result))
    }

    async fn document_highlight(
        &self,
        params: DocumentHighlightParams,
    ) -> Result<Option<Vec<DocumentHighlight>>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let docs = self.documents.lock().unwrap();
        let Some(doc) = docs.get(&uri) else {
            return Ok(None);
        };

        // Find the identifier at the cursor position
        let reference = find_identifier_at_position(
            &doc.tree,
            &doc.text.text,
            position.line as usize,
            position.character as usize,
        );

        let Some(reference) = reference else {
            return Ok(None);
        };

        // Find all references in the current file with declaration info
        let refs =
            find_references_with_kind(&doc.tree, &doc.text.text, &reference.name, reference.scope);

        if refs.is_empty() {
            return Ok(None);
        }

        let highlights: Vec<DocumentHighlight> = refs
            .into_iter()
            .map(|r| DocumentHighlight {
                range: Range {
                    start: Position {
                        line: r.location.start.0 as u32,
                        character: r.location.start.1 as u32,
                    },
                    end: Position {
                        line: r.location.end.0 as u32,
                        character: r.location.end.1 as u32,
                    },
                },
                kind: Some(if r.is_declaration {
                    DocumentHighlightKind::WRITE
                } else {
                    DocumentHighlightKind::READ
                }),
            })
            .collect();

        log_debug!(
            "document_highlight: found {} highlights for '{}'",
            highlights.len(),
            reference.name
        );

        Ok(Some(highlights))
    }

    async fn folding_range(&self, params: FoldingRangeParams) -> Result<Option<Vec<FoldingRange>>> {
        let uri = params.text_document.uri;

        let docs = self.documents.lock().unwrap();
        let Some(doc) = docs.get(&uri) else {
            return Ok(None);
        };

        let mut ranges = Vec::new();
        Self::collect_folding_ranges(&doc.tree.root_node(), &mut ranges);

        log_debug!("folding_range: found {} foldable regions", ranges.len());

        if ranges.is_empty() {
            Ok(None)
        } else {
            Ok(Some(ranges))
        }
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;

        let (uri_str, content) = {
            let docs = self.documents.lock().unwrap();
            let Some(doc) = docs.get(&uri) else {
                return Ok(None);
            };
            (uri.to_string(), doc.text.text.clone())
        };

        let symbols = self.get_symbols(&uri_str, &content);

        // Convert our symbols to LSP DocumentSymbol
        // Note: We use ls_types::SymbolKind to avoid conflict with our symbols::SymbolKind
        let lsp_symbols: Vec<DocumentSymbol> = symbols
            .into_iter()
            .map(|s| {
                let kind = match s.kind {
                    SymbolKind::Function => tower_lsp_server::ls_types::SymbolKind::FUNCTION,
                    SymbolKind::Variable => tower_lsp_server::ls_types::SymbolKind::VARIABLE,
                    SymbolKind::Parameter => tower_lsp_server::ls_types::SymbolKind::VARIABLE,
                };

                // For the range, we use the symbol's position as both range and selection_range
                // since Vim script function/variable definitions are typically single-line names
                let range = Range {
                    start: Position {
                        line: s.start.0 as u32,
                        character: s.start.1 as u32,
                    },
                    end: Position {
                        line: s.end.0 as u32,
                        character: s.end.1 as u32,
                    },
                };

                #[allow(deprecated)]
                DocumentSymbol {
                    name: s.full_name(),
                    detail: s.signature,
                    kind,
                    tags: None,
                    deprecated: None,
                    range,
                    selection_range: range,
                    children: None,
                }
            })
            .collect();

        Ok(Some(DocumentSymbolResponse::Nested(lsp_symbols)))
    }

    async fn symbol(
        &self,
        params: WorkspaceSymbolParams,
    ) -> Result<Option<WorkspaceSymbolResponse>> {
        // Wait for indexing to complete for accurate results
        if !self.indexing_complete.load(Ordering::SeqCst) {
            log_debug!("workspace_symbol: indexing not complete yet");
            return Ok(Some(WorkspaceSymbolResponse::Flat(Vec::new())));
        }

        let query = params.query.to_lowercase();
        let mut results: Vec<SymbolInformation> = Vec::new();

        // Limit results to avoid overwhelming the client
        const MAX_RESULTS: usize = 500;

        let source_files = self.source_files.lock().unwrap();
        let db = self.salsa_db.lock().unwrap();

        for (file_uri, source_file) in source_files.iter() {
            if results.len() >= MAX_RESULTS {
                break;
            }

            let symbols = db::parse_symbols(&*db, *source_file);

            for s in symbols {
                // Filter by query (case-insensitive partial match)
                // Empty query returns all symbols
                if !query.is_empty() && !s.full_name().to_lowercase().contains(&query) {
                    continue;
                }

                let kind = match s.kind {
                    SymbolKind::Function => tower_lsp_server::ls_types::SymbolKind::FUNCTION,
                    SymbolKind::Variable => tower_lsp_server::ls_types::SymbolKind::VARIABLE,
                    SymbolKind::Parameter => tower_lsp_server::ls_types::SymbolKind::VARIABLE,
                };

                let range = Range {
                    start: Position {
                        line: s.start.0 as u32,
                        character: s.start.1 as u32,
                    },
                    end: Position {
                        line: s.end.0 as u32,
                        character: s.end.1 as u32,
                    },
                };

                // Convert file path to URI
                let Some(uri) = Uri::from_file_path(file_uri) else {
                    continue;
                };

                #[allow(deprecated)]
                results.push(SymbolInformation {
                    name: s.full_name(),
                    kind,
                    tags: None,
                    deprecated: None,
                    location: Location { uri, range },
                    container_name: s.signature,
                });

                if results.len() >= MAX_RESULTS {
                    break;
                }
            }
        }

        log_debug!(
            "workspace_symbol: query='{}', found {} symbols",
            params.query,
            results.len()
        );

        Ok(Some(WorkspaceSymbolResponse::Flat(results)))
    }

    async fn prepare_rename(
        &self,
        params: TextDocumentPositionParams,
    ) -> Result<Option<PrepareRenameResponse>> {
        let uri = params.text_document.uri;
        let position = params.position;

        let docs = self.documents.lock().unwrap();
        let Some(doc) = docs.get(&uri) else {
            return Ok(None);
        };

        // Find the identifier at the cursor position
        let reference = find_identifier_at_position(
            &doc.tree,
            &doc.text.text,
            position.line as usize,
            position.character as usize,
        );

        let Some(reference) = reference else {
            return Ok(None);
        };

        // Don't allow renaming built-in functions
        if reference.is_call && BUILTIN_FUNCTIONS.iter().any(|f| f.name == reference.name) {
            return Ok(None);
        }

        // For autoload functions, return the full name
        let name = if let Some(autoload) = &reference.autoload {
            autoload.full_name.clone()
        } else {
            format!("{}{}", reference.scope.as_str(), reference.name)
        };

        Ok(Some(PrepareRenameResponse::RangeWithPlaceholder {
            range: Range {
                start: position,
                end: Position {
                    line: position.line,
                    character: position.character + name.len() as u32,
                },
            },
            placeholder: name,
        }))
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let new_name = params.new_name;

        let docs = self.documents.lock().unwrap();
        let Some(doc) = docs.get(&uri) else {
            return Ok(None);
        };

        // Find the identifier at the cursor position
        let reference = find_identifier_at_position(
            &doc.tree,
            &doc.text.text,
            position.line as usize,
            position.character as usize,
        );

        let Some(reference) = reference else {
            return Ok(None);
        };

        // Find all references in the current file
        let current_file_locations = find_references(
            &doc.tree,
            &doc.text.text,
            &reference.name,
            reference.scope,
            true, // include declaration
        );

        // Release the documents lock before searching other files
        drop(docs);

        // Collect all edits grouped by file
        let mut changes: HashMap<Uri, Vec<TextEdit>> = HashMap::new();

        // Add edits for current file
        let current_edits: Vec<TextEdit> = current_file_locations
            .into_iter()
            .map(|loc| TextEdit {
                range: Range {
                    start: Position {
                        line: loc.start.0 as u32,
                        character: loc.start.1 as u32,
                    },
                    end: Position {
                        line: loc.end.0 as u32,
                        character: loc.end.1 as u32,
                    },
                },
                new_text: new_name.clone(),
            })
            .collect();

        if !current_edits.is_empty() {
            changes.insert(uri.clone(), current_edits);
        }

        // Search in other indexed files for cross-file visible symbols
        let is_cross_file_visible = reference.autoload.is_some()
            || reference.scope == symbols::VimScope::Global
            || reference.scope == symbols::VimScope::Implicit && reference.name.contains('#');

        if is_cross_file_visible && self.indexing_complete.load(Ordering::SeqCst) {
            let current_uri_str = uri.to_string();
            let source_files = self.source_files.lock().unwrap();
            let db = self.salsa_db.lock().unwrap();

            for (file_uri, source_file) in source_files.iter() {
                // Skip the current file (already processed)
                if file_uri == &current_uri_str {
                    continue;
                }

                let content = source_file.content(&*db);

                // Parse the file to search for references
                let mut parser = tree_sitter::Parser::new();
                parser
                    .set_language(&tree_sitter_vim::language())
                    .expect("Error loading vim grammar");

                if let Some(tree) = parser.parse(&content, None) {
                    let locations = find_references(
                        &tree,
                        &content,
                        &reference.name,
                        reference.scope,
                        true, // include declaration
                    );

                    if !locations.is_empty() {
                        if let Some(file_uri_parsed) = Uri::from_file_path(file_uri) {
                            let edits: Vec<TextEdit> = locations
                                .into_iter()
                                .map(|loc| TextEdit {
                                    range: Range {
                                        start: Position {
                                            line: loc.start.0 as u32,
                                            character: loc.start.1 as u32,
                                        },
                                        end: Position {
                                            line: loc.end.0 as u32,
                                            character: loc.end.1 as u32,
                                        },
                                    },
                                    new_text: new_name.clone(),
                                })
                                .collect();

                            changes.insert(file_uri_parsed, edits);
                        }
                    }
                }
            }
        }

        log_debug!(
            "rename: '{}' -> '{}', {} files affected",
            reference.name,
            new_name,
            changes.len()
        );

        if changes.is_empty() {
            return Ok(None);
        }

        Ok(Some(WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        }))
    }

    async fn selection_range(
        &self,
        params: SelectionRangeParams,
    ) -> Result<Option<Vec<SelectionRange>>> {
        let uri = params.text_document.uri;

        let docs = self.documents.lock().unwrap();
        let Some(doc) = docs.get(&uri) else {
            return Ok(None);
        };

        let ranges: Vec<SelectionRange> = params
            .positions
            .iter()
            .filter_map(|pos| Self::build_selection_range(&doc.tree, pos))
            .collect();

        log_debug!(
            "selection_range: {} positions requested, {} ranges returned",
            params.positions.len(),
            ranges.len()
        );

        if ranges.is_empty() {
            Ok(None)
        } else {
            Ok(Some(ranges))
        }
    }
}

fn print_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

fn print_help() {
    println!(
        "{} - {}

Usage: {} [OPTIONS]

Options:
  -V, --version          Show version information
      --vim-only         Show only Vim-compatible functions in completion
      --neovim-only      Show only Neovim-compatible functions in completion
      --vimruntime=<PATH> Override $VIMRUNTIME path for autoload resolution
      --log=<PATH>       Enable debug logging to specified file
  -h, --help             Show this help message

This is an LSP server for Vim script. It communicates via stdin/stdout
using the Language Server Protocol.",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_NAME")
    );
}

#[tokio::main]
async fn main() {
    // Parse CLI arguments
    let args: Vec<String> = std::env::args().collect();

    let mut vim_only = false;
    let mut neovim_only = false;

    for arg in &args[1..] {
        match arg.as_str() {
            "-V" | "--version" => {
                print_version();
                return;
            }
            "-h" | "--help" => {
                print_help();
                return;
            }
            "--vim-only" => vim_only = true,
            "--neovim-only" => neovim_only = true,
            _ => {}
        }
    }

    // Check for conflicting options
    if vim_only && neovim_only {
        eprintln!("error: --vim-only and --neovim-only cannot be used together");
        std::process::exit(1);
    }

    let editor_mode = if vim_only {
        EditorMode::VimOnly
    } else if neovim_only {
        EditorMode::NeovimOnly
    } else {
        EditorMode::Both
    };

    // Parse --log=PATH argument
    let log_path = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--log=").map(String::from));
    logger::init(log_path);

    // Parse --vimruntime=PATH or get from environment
    let vimruntime: Option<PathBuf> = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--vimruntime=").map(PathBuf::from))
        .or_else(|| std::env::var("VIMRUNTIME").ok().map(PathBuf::from))
        .filter(|p| p.exists());

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) =
        LspService::new(|client| Backend::new(client, editor_mode, vimruntime.clone()));
    Server::new(stdin, stdout, socket).serve(service).await;
}

#[cfg(test)]
mod completion_context_tests {
    use super::*;

    #[test]
    fn test_empty_line_returns_command() {
        assert_eq!(get_completion_context("", 0), CompletionContext::Command);
        assert_eq!(
            get_completion_context("    ", 4),
            CompletionContext::Command
        );
    }

    #[test]
    fn test_autocmd_event_context() {
        // "autocmd " followed by typing event name
        assert_eq!(
            get_completion_context("autocmd Buf", 11),
            CompletionContext::AutocmdEvent
        );
        assert_eq!(
            get_completion_context("autocmd ", 8),
            CompletionContext::AutocmdEvent
        );
        // "au " shorthand
        assert_eq!(
            get_completion_context("au FileType", 11),
            CompletionContext::AutocmdEvent
        );
    }

    #[test]
    fn test_set_option_context() {
        // "set " followed by option name
        assert_eq!(
            get_completion_context("set nu", 6),
            CompletionContext::Option
        );
        assert_eq!(
            get_completion_context("setlocal expandtab", 18),
            CompletionContext::Option
        );
        assert_eq!(
            get_completion_context("setg ", 5),
            CompletionContext::Option
        );
    }

    #[test]
    fn test_map_option_context() {
        // Map commands with <...> options
        assert_eq!(
            get_completion_context("nnoremap <silent", 16),
            CompletionContext::MapOption
        );
        assert_eq!(
            get_completion_context("nmap <buf", 9),
            CompletionContext::MapOption
        );
        assert_eq!(
            get_completion_context("inoremap <", 10),
            CompletionContext::MapOption
        );
    }

    #[test]
    fn test_has_feature_context() {
        // Inside has('...') call
        assert_eq!(
            get_completion_context("if has('nvi", 11),
            CompletionContext::HasFeature
        );
        assert_eq!(
            get_completion_context("if has(\"py", 10),
            CompletionContext::HasFeature
        );
        assert_eq!(
            get_completion_context("  has('", 7),
            CompletionContext::HasFeature
        );
    }

    #[test]
    fn test_command_context() {
        // Line start with command
        assert_eq!(get_completion_context("ech", 3), CompletionContext::Command);
        assert_eq!(get_completion_context("let", 3), CompletionContext::Command);
    }

    #[test]
    fn test_function_context() {
        // Expression context
        assert_eq!(
            get_completion_context("let x = str", 11),
            CompletionContext::Function
        );
        assert_eq!(
            get_completion_context("call MyFunc(arg", 15),
            CompletionContext::Function
        );
        assert_eq!(
            get_completion_context("return strlen(s", 15),
            CompletionContext::Function
        );
    }

    #[test]
    fn test_operator_not_confused_with_command() {
        // Operators should not trigger Command context
        // `<` as comparison operator, not Ex command
        assert_eq!(
            get_completion_context("if a < b", 6),
            CompletionContext::Function
        );
        // `<` after `=` assignment
        assert_eq!(
            get_completion_context("let x = <", 9),
            CompletionContext::Function
        );
        // `>` as comparison operator
        assert_eq!(
            get_completion_context("if a > b", 6),
            CompletionContext::Function
        );
        // `<` at line start IS a valid Ex command (shift left)
        assert_eq!(get_completion_context("<", 1), CompletionContext::Command);
        assert_eq!(get_completion_context(">", 1), CompletionContext::Command);
    }
}
