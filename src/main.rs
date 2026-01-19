mod builtins;
mod db;
mod logger;
mod symbols;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use texter::change::Change;
use texter::core::text::Text;
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::*;
use tower_lsp_server::{Client, LanguageServer, LspService, Server};
use tree_sitter::{Parser, Tree};

use builtins::BUILTIN_FUNCTIONS;
use db::{HjklsDatabase, SourceFile};
use salsa::Setter;
use symbols::{SymbolKind, find_identifier_at_position, find_references};

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

    /// Find autoload file in workspace or relative to a document
    /// Returns the file path if found
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
        let text = Text::new_utf16(content);
        let tree = match self.parse(&text.text, None) {
            Some(t) => t,
            None => return vec![],
        };

        let diagnostics = {
            let mut diags = vec![];
            let mut cursor = tree.walk();
            collect_errors(&mut cursor, &text.text, &mut diags);
            diags
        };

        let mut docs = self.documents.lock().unwrap();
        docs.insert(uri, Document { text, tree });

        diagnostics
    }

    /// Update document with incremental change
    fn update_document(&self, uri: &Uri, content: String) -> Vec<Diagnostic> {
        let mut docs = self.documents.lock().unwrap();

        if let Some(doc) = docs.get_mut(uri) {
            // Apply full replacement using texter
            let change = Change::ReplaceFull(content.into());
            if doc.text.update(change, &mut doc.tree).is_ok() {
                // Re-parse with old tree for incremental parsing
                if let Some(new_tree) = self.parse(&doc.text.text, Some(&doc.tree)) {
                    doc.tree = new_tree;
                }
            }

            let mut diagnostics = vec![];
            let mut cursor = doc.tree.walk();
            collect_errors(&mut cursor, &doc.text.text, &mut diagnostics);
            return diagnostics;
        }

        drop(docs);
        // Document not found, open as new
        self.open_document(uri.clone(), content)
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
                definition_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                references_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
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

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        let items: Vec<CompletionItem> = BUILTIN_FUNCTIONS
            .iter()
            .map(|func| CompletionItem {
                label: func.name.to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(func.signature.to_string()),
                documentation: Some(Documentation::String(func.description.to_string())),
                ..Default::default()
            })
            .collect();

        Ok(Some(CompletionResponse::Array(items)))
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
