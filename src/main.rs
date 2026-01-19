mod builtins;

use std::collections::HashMap;
use std::sync::Mutex;

use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::*;
use tower_lsp_server::{Client, LanguageServer, LspService, Server};
use tree_sitter::{Parser, Tree};

use builtins::BUILTIN_FUNCTIONS;

/// Document state holding source text and parsed tree
#[allow(dead_code)] // Fields will be used for incremental parsing
struct Document {
    text: String,
    tree: Option<Tree>,
}

/// LSP backend for Vim script
struct Backend {
    client: Client,
    parser: Mutex<Parser>,
    documents: Mutex<HashMap<Uri, Document>>,
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
        }
    }

    /// Parse document and collect syntax errors
    fn parse_and_diagnose(&self, uri: &Uri, text: &str) -> Vec<Diagnostic> {
        let tree = {
            let mut parser = self.parser.lock().unwrap();
            parser.parse(text, None)
        };

        let Some(tree) = tree else {
            return vec![];
        };

        // Store document
        {
            let mut docs = self.documents.lock().unwrap();
            docs.insert(
                uri.clone(),
                Document {
                    text: text.to_string(),
                    tree: Some(tree.clone()),
                },
            );
        }

        // Collect ERROR nodes as diagnostics
        let mut diagnostics = vec![];
        let mut cursor = tree.walk();
        collect_errors(&mut cursor, text, &mut diagnostics);

        diagnostics
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
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions::default()),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "hjkls initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;

        let diagnostics = self.parse_and_diagnose(&uri, &text);
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

        let diagnostics = self.parse_and_diagnose(&uri, &change.text);
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        // Remove document from cache
        let mut docs = self.documents.lock().unwrap();
        docs.remove(&params.text_document.uri);
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
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
