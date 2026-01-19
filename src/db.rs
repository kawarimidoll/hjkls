//! Salsa database for incremental computation

// Functions will be used in go-to-definition implementation
#![allow(dead_code)]

use crate::symbols::{Symbol, VimScope};
use salsa::Database;

/// Source file input for salsa
#[salsa::input]
pub struct SourceFile {
    /// File URI
    pub uri: String,
    /// File content
    pub content: String,
}

/// Parse symbols from a source file (memoized by salsa)
#[salsa::tracked]
pub fn parse_symbols(db: &dyn Database, file: SourceFile) -> Vec<Symbol> {
    let content = file.content(db);

    // Parse with tree-sitter
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_vim::language())
        .expect("Error loading vim grammar");

    if let Some(tree) = parser.parse(&content, None) {
        crate::symbols::extract_symbols(&tree, &content)
    } else {
        Vec::new()
    }
}

/// Find symbol at a given position
pub fn find_symbol_at_position(
    db: &dyn Database,
    file: SourceFile,
    row: usize,
    col: usize,
) -> Option<Symbol> {
    let symbols = parse_symbols(db, file);

    symbols.into_iter().find(|s| {
        let (start_row, start_col) = s.start;
        let (end_row, end_col) = s.end;

        if row < start_row || row > end_row {
            return false;
        }
        if row == start_row && col < start_col {
            return false;
        }
        if row == end_row && col > end_col {
            return false;
        }
        true
    })
}

/// Find symbol definition by name
pub fn find_symbol_definition(
    db: &dyn Database,
    file: SourceFile,
    name: &str,
    scope: Option<VimScope>,
) -> Option<Symbol> {
    let symbols = parse_symbols(db, file);

    symbols
        .into_iter()
        .find(|s| s.name == name && scope.is_none_or(|sc| s.scope == sc))
}
