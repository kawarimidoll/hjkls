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

use builtins::BUILTIN_FUNCTIONS;
use db::{HjklsDatabase, SourceFile};
use salsa::Setter;
use symbols::{SymbolKind, find_call_at_position, find_identifier_at_position, find_references};

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
}

impl Backend {
    fn new(client: Client) -> Self {
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

        let mut docs = self.documents.lock().unwrap();
        docs.insert(uri.clone(), Document { text, tree });

        diagnostics
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
                rename_provider: Some(OneOf::Right(RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress_options: Default::default(),
                })),
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

        // Get document content and find the completion token range
        let (uri_str, content, token_start, input_has_scope) = {
            let docs = self.documents.lock().unwrap();
            let Some(doc) = docs.get(&uri) else {
                return Ok(Some(CompletionResponse::Array(vec![])));
            };
            let content = doc.text.text.clone();

            // Find token start position (including scope prefix like s:, g:)
            let line = content.lines().nth(position.line as usize).unwrap_or("");
            let col = position.character as usize;
            let token_start = find_completion_token_start(line, col);

            // Check if current input contains a scope prefix (e.g., "g:", "s:")
            let current_input = &line[token_start..col.min(line.len())];
            let input_has_scope = current_input.contains(':');

            (uri.to_string(), content, token_start, input_has_scope)
        };

        // Create text edit range for replacing the current token
        let edit_range = Range {
            start: Position {
                line: position.line,
                character: token_start as u32,
            },
            end: position,
        };

        // 1. Built-in functions
        let mut items: Vec<CompletionItem> = BUILTIN_FUNCTIONS
            .iter()
            .map(|func| CompletionItem {
                label: func.name.to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(func.signature.to_string()),
                documentation: Some(Documentation::String(func.description.to_string())),
                text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                    range: edit_range,
                    new_text: func.name.to_string(),
                })),
                ..Default::default()
            })
            .collect();

        // 2. User-defined symbols from current document
        let symbols = self.get_symbols(&uri_str, &content);
        for sym in symbols {
            // Skip parameters and empty names
            if sym.kind == SymbolKind::Parameter || sym.name.is_empty() {
                continue;
            }
            let kind = match sym.kind {
                SymbolKind::Function => CompletionItemKind::FUNCTION,
                SymbolKind::Variable => CompletionItemKind::VARIABLE,
                SymbolKind::Parameter => continue, // already handled above
            };
            let detail = sym.signature.clone().or_else(|| {
                // For variables, show scope as detail
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

            // For scoped symbols, choose filter strategy based on user input:
            // - If user typed scope prefix (e.g., "g:"), match by full name
            // - If user typed without scope (e.g., "m"), match by name only
            let filter_text = if has_scope && !input_has_scope {
                // User typing without scope -> filter by name only
                Some(sym.name.clone())
            } else {
                // User typing with scope or symbol has no scope -> filter by label (full_name)
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
        if reference.is_call {
            if BUILTIN_FUNCTIONS.iter().any(|f| f.name == reference.name) {
                return Ok(None);
            }
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
}

#[tokio::main]
async fn main() {
    // Parse --log=PATH argument
    let log_path = std::env::args().find_map(|arg| arg.strip_prefix("--log=").map(String::from));
    logger::init(log_path);

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
