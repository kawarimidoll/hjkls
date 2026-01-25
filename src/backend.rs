use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use salsa::Setter;
use texter::core::text::Text;
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::*;
use tower_lsp_server::{Client, LanguageServer};
use tree_sitter::{Parser, Tree};

use crate::builtins::{
    AUTOCMD_EVENTS, BUILTIN_COMMANDS, BUILTIN_FUNCTIONS, BUILTIN_OPTIONS, BUILTIN_VARIABLES,
    EditorMode, HAS_FEATURES, MAP_OPTIONS,
};
use crate::completion::CompletionContext;
use crate::config::Config;
use crate::db::{self, HjklsDatabase, SourceFile};
use crate::diagnostics;
use crate::log_debug;
use crate::symbols::{
    self, SymbolKind, find_call_at_position, find_identifier_at_position, find_references,
    find_references_with_kind,
};

/// Document state holding text and syntax tree
pub(crate) struct Document {
    text: Text,
    tree: Tree,
}

/// LSP backend for Vim script
pub struct Backend {
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
    /// Lint configuration loaded from .hjkls.toml
    config: Arc<Mutex<Config>>,
}

impl Backend {
    pub fn new(client: Client, editor_mode: EditorMode, vimruntime: Option<PathBuf>) -> Self {
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
            config: Arc::new(Mutex::new(Config::default())),
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

        // Load configuration from workspace
        if let Some(loaded_config) = Config::find_in_workspace(&roots) {
            log_debug!("Loaded config from workspace");
            let mut config = self.config.lock().unwrap();
            *config = loaded_config;
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
                                code: Some(NumberOrString::String(
                                    "hjkls/autoload_missing".to_string(),
                                )),
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
                                    code: Some(NumberOrString::String(
                                        "hjkls/arity_mismatch".to_string(),
                                    )),
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
                            code: Some(NumberOrString::String("hjkls/scope_violation".to_string())),
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
                code: Some(NumberOrString::String("hjkls/scope_violation".to_string())),
                ..Default::default()
            });
        }

        // Recurse into children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::collect_scope_violations_recursive(&child, source, in_func, diagnostics);
        }
    }

    /// Collect style hints (code style suggestions, DiagnosticSeverity::HINT)
    fn collect_style_hints(&self, tree: &Tree, source: &str) -> Vec<Diagnostic> {
        diagnostics::collect_style_hints(tree, source)
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
                    let func_kind = func_node.kind();

                    // Skip dynamic/runtime function calls that cannot be statically checked:
                    // - field_expression: dictionary methods (dict.method(), self.method())
                    // - index_expression: dictionary subscript (a:args['callback']())
                    // - argument: a: scope variables (a:callback())
                    // - scoped_identifier with l: prefix: local variables (l:Func())
                    let is_dynamic_call = func_kind == "field_expression"
                        || func_kind == "index_expression"
                        || func_kind == "argument"
                        || (func_kind == "scoped_identifier" && func_name.starts_with("l:"));

                    // For identifiers, check if it's a variable (lambda/funcref stored in variable)
                    let is_variable_call = func_kind == "identifier"
                        && local_symbols.iter().any(|s| {
                            s.kind == symbols::SymbolKind::Variable && s.name == func_name
                        });

                    // Skip empty names, autoload functions, and dynamic/variable calls
                    if !func_name.is_empty()
                        && !func_name.contains('#')
                        && !is_dynamic_call
                        && !is_variable_call
                    {
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
                                code: Some(NumberOrString::String(
                                    "hjkls/undefined_function".to_string(),
                                )),
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

    /// Replace single dot concatenation with double dot in Vim script
    /// Only replaces `.` that is surrounded by spaces (string concatenation)
    fn replace_single_dot_with_double(text: &str) -> String {
        // Pattern: " . " (single dot with spaces) should become " .. "
        // We need to be careful not to replace ".." or method calls like ".call"
        let mut result = String::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if chars[i] == '.' {
                // Check if this is a single dot (not part of ..)
                let prev_is_dot = i > 0 && chars[i - 1] == '.';
                let next_is_dot = i + 1 < chars.len() && chars[i + 1] == '.';

                if !prev_is_dot && !next_is_dot {
                    // This is a single dot - replace with ..
                    result.push_str("..");
                    i += 1;
                    continue;
                }
            }
            result.push(chars[i]);
            i += 1;
        }

        result
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
        diagnostics.extend(diagnostics::collect_suspicious_warnings(&tree, &text.text));

        // Collect style hints
        let style_hints = self.collect_style_hints(&tree, &text.text);
        diagnostics.extend(style_hints);

        // Filter diagnostics based on inline ignore directives
        let directives = diagnostics::parse_ignore_directives(&text.text);
        let diagnostics = diagnostics::filter_diagnostics(diagnostics, &directives);

        // Filter diagnostics based on config settings
        let diagnostics = {
            let config = self.config.lock().unwrap();
            diagnostics::filter_by_config(diagnostics, &config)
        };

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
        diagnostics.extend(diagnostics::collect_suspicious_warnings(&tree, &text.text));

        // Collect style hints
        let style_hints = self.collect_style_hints(&tree, &text.text);
        diagnostics.extend(style_hints);

        // Filter diagnostics based on inline ignore directives
        let directives = diagnostics::parse_ignore_directives(&text.text);
        let diagnostics = diagnostics::filter_diagnostics(diagnostics, &directives);

        // Filter diagnostics based on config settings
        let diagnostics = {
            let config = self.config.lock().unwrap();
            diagnostics::filter_by_config(diagnostics, &config)
        };

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
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
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
            let token_start = crate::completion::find_completion_token_start(&line, col);

            // Check if current input contains a scope prefix (e.g., "g:", "s:")
            let current_input = &line[token_start..col.min(line.len())];
            let input_has_scope = current_input.contains(':');

            // Determine completion context based on cursor position
            let context = crate::completion::get_completion_context(&line, col);

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

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let uri = params.text_document.uri;

        let docs = self.documents.lock().unwrap();
        let Some(doc) = docs.get(&uri) else {
            return Ok(None);
        };
        let source = doc.text.to_string();
        drop(docs);

        let mut actions = Vec::new();

        for diag in params.context.diagnostics {
            // Get the diagnostic code
            let code = match &diag.code {
                Some(NumberOrString::String(s)) => s.as_str(),
                _ => continue,
            };

            // Get the text at the diagnostic range
            let start_line = diag.range.start.line as usize;
            let end_line = diag.range.end.line as usize;
            let lines: Vec<&str> = source.lines().collect();

            if start_line >= lines.len() {
                continue;
            }

            let line = lines.get(start_line).unwrap_or(&"");
            let start_col = diag.range.start.character as usize;
            let end_col = if start_line == end_line {
                diag.range.end.character as usize
            } else {
                line.len()
            };

            // Create text edit based on the rule
            // Returns: Option<(title, range, new_text)>
            let edit: Option<(&str, Range, String)> = match code {
                "hjkls/double_dot" => {
                    if end_col <= line.len() {
                        let text = &line[start_col..end_col];
                        let new_text = Self::replace_single_dot_with_double(text);
                        if new_text != text {
                            Some(("Use `..` for string concatenation", diag.range, new_text))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                "hjkls/single_quote" => {
                    if end_col <= line.len() {
                        let text = &line[start_col..end_col];
                        if text.starts_with('"') && text.ends_with('"') && text.len() >= 2 {
                            let inner = &text[1..text.len() - 1];
                            Some(("Use single quotes", diag.range, format!("'{}'", inner)))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                "hjkls/key_notation" => {
                    if end_col <= line.len() {
                        let text = &line[start_col..end_col];
                        diagnostics::style::normalize_key_notation(text)
                            .map(|normalized| ("Normalize key notation", diag.range, normalized))
                    } else {
                        None
                    }
                }
                "hjkls/normal_bang" => line.get(start_col..).and_then(|text_after| {
                    text_after.to_lowercase().find("normal").and_then(|pos| {
                        let normal_start = start_col + pos;
                        let normal_end = normal_start + 6;
                        let original = line.get(normal_start..normal_end)?;
                        let after = line.get(normal_end..).unwrap_or("");

                        if !after.starts_with('!') {
                            Some((
                                "Use `normal!` to ignore user mappings",
                                Range {
                                    start: Position {
                                        line: diag.range.start.line,
                                        character: normal_start as u32,
                                    },
                                    end: Position {
                                        line: diag.range.start.line,
                                        character: normal_end as u32,
                                    },
                                },
                                format!("{}!", original),
                            ))
                        } else {
                            None
                        }
                    })
                }),
                "hjkls/function_bang" => line.get(start_col..).and_then(|text_after| {
                    text_after.to_lowercase().find("function!").map(|pos| {
                        let func_start = start_col + pos;
                        let func_end = func_start + 9;
                        let original = line.get(func_start..func_end).unwrap_or("function!");

                        (
                            "Remove unnecessary `!` from s: function",
                            Range {
                                start: Position {
                                    line: diag.range.start.line,
                                    character: func_start as u32,
                                },
                                end: Position {
                                    line: diag.range.start.line,
                                    character: func_end as u32,
                                },
                            },
                            original.get(..8).unwrap_or("function").to_string(),
                        )
                    })
                }),
                "hjkls/match_case" => {
                    if end_col <= line.len() {
                        let text = &line[start_col..end_col];
                        text.find("=~").and_then(|pos| {
                            let after = text.get(pos + 2..).unwrap_or("");
                            if !after.starts_with('#') && !after.starts_with('?') {
                                let op_start = start_col + pos;
                                let op_end = op_start + 2;
                                Some((
                                    "Use `=~#` for case-sensitive match",
                                    Range {
                                        start: Position {
                                            line: diag.range.start.line,
                                            character: op_start as u32,
                                        },
                                        end: Position {
                                            line: diag.range.start.line,
                                            character: op_end as u32,
                                        },
                                    },
                                    "=~#".to_string(),
                                ))
                            } else {
                                None
                            }
                        })
                    } else {
                        None
                    }
                }
                "hjkls/abort" => {
                    // Add `abort` attribute to function definition
                    // The diagnostic range covers the first line of the function
                    // Insert ` abort` at the end of the line (before newline)
                    let line_end = line.len();
                    Some((
                        "Add `abort` attribute",
                        Range {
                            start: Position {
                                line: diag.range.start.line,
                                character: line_end as u32,
                            },
                            end: Position {
                                line: diag.range.start.line,
                                character: line_end as u32,
                            },
                        },
                        " abort".to_string(),
                    ))
                }
                "hjkls/plug_noremap" => {
                    // Replace map command with noremap equivalent
                    // The diagnostic range covers just the map command (e.g., "nmap")
                    if end_col <= line.len() {
                        let cmd = &line[start_col..end_col];
                        diagnostics::style::get_noremap_equivalent(cmd).map(|noremap_cmd| {
                            (
                                "Use noremap for <Plug> mapping",
                                diag.range,
                                noremap_cmd.to_string(),
                            )
                        })
                    } else {
                        None
                    }
                }
                // Other rules don't have simple auto-fixes
                _ => None,
            };

            // Create the code action if we have an edit
            if let Some((title, range, new_text)) = edit {
                let text_edit = TextEdit { range, new_text };

                let mut changes = HashMap::new();
                changes.insert(uri.clone(), vec![text_edit]);

                let workspace_edit = WorkspaceEdit {
                    changes: Some(changes),
                    document_changes: None,
                    change_annotations: None,
                };

                actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                    title: title.to_string(),
                    kind: Some(CodeActionKind::QUICKFIX),
                    diagnostics: Some(vec![diag.clone()]),
                    edit: Some(workspace_edit),
                    command: None,
                    is_preferred: Some(true),
                    disabled: None,
                    data: None,
                }));
            }
        }

        if actions.is_empty() {
            Ok(None)
        } else {
            Ok(Some(actions))
        }
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri;

        let (source, tree) = {
            let docs = self.documents.lock().unwrap();
            let Some(doc) = docs.get(&uri) else {
                return Ok(None);
            };
            (doc.text.text.clone(), doc.tree.clone())
        };

        // Clone format config to minimize lock hold time
        let format_config = {
            let config = self.config.lock().unwrap();
            config.format.clone()
        };
        let edits = crate::formatter::format(&source, &tree, &format_config);

        if edits.is_empty() {
            Ok(None)
        } else {
            Ok(Some(edits))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_single_dot_with_double() {
        // Basic replacement
        assert_eq!(Backend::replace_single_dot_with_double("a . b"), "a .. b");
        assert_eq!(Backend::replace_single_dot_with_double("x.y"), "x..y");

        // Already double dot - no change
        assert_eq!(Backend::replace_single_dot_with_double("a .. b"), "a .. b");
        assert_eq!(Backend::replace_single_dot_with_double("x..y"), "x..y");

        // Single dot only
        assert_eq!(Backend::replace_single_dot_with_double("."), "..");

        // Multiple single dots
        assert_eq!(
            Backend::replace_single_dot_with_double("a . b . c"),
            "a .. b .. c"
        );

        // Mixed single and double dots
        assert_eq!(
            Backend::replace_single_dot_with_double("a . b .. c"),
            "a .. b .. c"
        );

        // No dots
        assert_eq!(Backend::replace_single_dot_with_double("abc"), "abc");

        // Empty string
        assert_eq!(Backend::replace_single_dot_with_double(""), "");
    }
}
