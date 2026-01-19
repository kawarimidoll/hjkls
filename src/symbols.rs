//! Symbol extraction from Vim script syntax trees

// Some items will be used in go-to-definition/hover implementation
#![allow(dead_code)]

use tree_sitter::{Node, Tree};

/// Vim script variable scope
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VimScope {
    /// Global scope (g:)
    Global,
    /// Script-local scope (s:)
    Script,
    /// Local scope (l:)
    Local,
    /// Buffer-local scope (b:)
    Buffer,
    /// Window-local scope (w:)
    Window,
    /// Tab-local scope (t:)
    Tab,
    /// Vim predefined scope (v:)
    Vim,
    /// Function argument (a:)
    Argument,
    /// No explicit scope (defaults to local in functions, global otherwise)
    Implicit,
}

impl VimScope {
    /// Parse scope from tree-sitter scope node text
    pub fn from_str(s: &str) -> Self {
        match s {
            "g:" => Self::Global,
            "s:" => Self::Script,
            "l:" => Self::Local,
            "b:" => Self::Buffer,
            "w:" => Self::Window,
            "t:" => Self::Tab,
            "v:" => Self::Vim,
            "a:" => Self::Argument,
            _ => Self::Implicit,
        }
    }

    /// Get the scope prefix string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Global => "g:",
            Self::Script => "s:",
            Self::Local => "l:",
            Self::Buffer => "b:",
            Self::Window => "w:",
            Self::Tab => "t:",
            Self::Vim => "v:",
            Self::Argument => "a:",
            Self::Implicit => "",
        }
    }
}

/// Kind of symbol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    Function,
    Variable,
    Parameter,
}

/// A symbol in Vim script
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    /// Symbol name (without scope prefix)
    pub name: String,
    /// Symbol scope
    pub scope: VimScope,
    /// Symbol kind
    pub kind: SymbolKind,
    /// Start position (row, column)
    pub start: (usize, usize),
    /// End position (row, column)
    pub end: (usize, usize),
    /// Function signature (for functions)
    pub signature: Option<String>,
}

impl Symbol {
    /// Get the full name including scope prefix
    pub fn full_name(&self) -> String {
        format!("{}{}", self.scope.as_str(), self.name)
    }
}

/// Parsed autoload function reference
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutoloadRef {
    /// The full autoload name (e.g., "myplugin#util#helper")
    pub full_name: String,
    /// Path components (e.g., ["myplugin", "util"])
    pub path_parts: Vec<String>,
    /// Function name (e.g., "helper")
    pub func_name: String,
}

impl AutoloadRef {
    /// Parse an autoload function name
    /// Returns None if the name doesn't contain '#'
    pub fn parse(name: &str) -> Option<Self> {
        if !name.contains('#') {
            return None;
        }

        let parts: Vec<&str> = name.split('#').collect();
        if parts.len() < 2 {
            return None;
        }

        let func_name = parts.last()?.to_string();
        let path_parts: Vec<String> = parts[..parts.len() - 1]
            .iter()
            .map(|s| s.to_string())
            .collect();

        Some(Self {
            full_name: name.to_string(),
            path_parts,
            func_name,
        })
    }

    /// Get the expected file path relative to runtimepath
    /// e.g., "myplugin#util#helper" -> "autoload/myplugin/util.vim"
    pub fn to_file_path(&self) -> String {
        format!("autoload/{}.vim", self.path_parts.join("/"))
    }
}

#[cfg(test)]
mod autoload_tests {
    use super::*;

    #[test]
    fn test_parse_autoload_simple() {
        let result = AutoloadRef::parse("myplugin#func").unwrap();
        assert_eq!(result.path_parts, vec!["myplugin"]);
        assert_eq!(result.func_name, "func");
        assert_eq!(result.to_file_path(), "autoload/myplugin.vim");
    }

    #[test]
    fn test_parse_autoload_nested() {
        let result = AutoloadRef::parse("myplugin#util#helper").unwrap();
        assert_eq!(result.path_parts, vec!["myplugin", "util"]);
        assert_eq!(result.func_name, "helper");
        assert_eq!(result.to_file_path(), "autoload/myplugin/util.vim");
    }

    #[test]
    fn test_parse_autoload_deep() {
        let result = AutoloadRef::parse("a#b#c#d#func").unwrap();
        assert_eq!(result.path_parts, vec!["a", "b", "c", "d"]);
        assert_eq!(result.func_name, "func");
        assert_eq!(result.to_file_path(), "autoload/a/b/c/d.vim");
    }

    #[test]
    fn test_parse_non_autoload() {
        assert!(AutoloadRef::parse("regular_func").is_none());
        assert!(AutoloadRef::parse("s:private").is_none());
    }
}

/// A reference to a symbol at a specific location
#[derive(Debug, Clone)]
pub struct Reference {
    /// Symbol name (without scope prefix for regular symbols, full name for autoload)
    pub name: String,
    /// Symbol scope
    pub scope: VimScope,
    /// Whether this is a function call
    pub is_call: bool,
    /// Autoload reference info (if this is an autoload function call)
    pub autoload: Option<AutoloadRef>,
}

/// Find the identifier at a given position in the syntax tree
pub fn find_identifier_at_position(
    tree: &Tree,
    source: &str,
    row: usize,
    col: usize,
) -> Option<Reference> {
    let root = tree.root_node();
    find_identifier_in_node(&root, source, row, col)
}

fn find_identifier_in_node(node: &Node, source: &str, row: usize, col: usize) -> Option<Reference> {
    // Check if position is within this node
    let start = node.start_position();
    let end = node.end_position();

    if row < start.row || row > end.row {
        return None;
    }
    if row == start.row && col < start.column {
        return None;
    }
    if row == end.row && col > end.column {
        return None;
    }

    // Check children first (more specific match)
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if let Some(reference) = find_identifier_in_node(&child, source, row, col) {
            return Some(reference);
        }
    }

    // Check if this node is an identifier
    match node.kind() {
        "identifier" => {
            let name = node.utf8_text(source.as_bytes()).ok()?.to_string();
            // Check if parent is a call_expression
            let is_call = node.parent().is_some_and(|p| p.kind() == "call_expression");

            // Check if this is an autoload function reference
            let autoload = if is_call {
                AutoloadRef::parse(&name)
            } else {
                None
            };

            Some(Reference {
                name,
                scope: VimScope::Implicit,
                is_call,
                autoload,
            })
        }
        "scoped_identifier" => {
            let mut cursor = node.walk();
            let children: Vec<_> = node.children(&mut cursor).collect();

            let scope_node = children.iter().find(|c| c.kind() == "scope")?;
            let ident_node = children.iter().find(|c| c.kind() == "identifier")?;

            let scope_text = scope_node.utf8_text(source.as_bytes()).ok()?;
            let name = ident_node.utf8_text(source.as_bytes()).ok()?.to_string();

            let is_call = node.parent().is_some_and(|p| p.kind() == "call_expression");
            Some(Reference {
                name,
                scope: VimScope::from_str(scope_text),
                is_call,
                autoload: None,
            })
        }
        _ => None,
    }
}

/// Extract symbols from a syntax tree
pub fn extract_symbols(tree: &Tree, source: &str) -> Vec<Symbol> {
    let mut symbols = Vec::new();
    let root = tree.root_node();
    extract_symbols_from_node(&root, source, &mut symbols);
    symbols
}

fn extract_symbols_from_node(node: &Node, source: &str, symbols: &mut Vec<Symbol>) {
    match node.kind() {
        "function_definition" => {
            if let Some(symbol) = extract_function_symbol(node, source) {
                symbols.push(symbol);
            }
        }
        "let_statement" | "const_statement" => {
            if let Some(symbol) = extract_variable_symbol(node, source) {
                symbols.push(symbol);
            }
        }
        _ => {}
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        extract_symbols_from_node(&child, source, symbols);
    }
}

fn extract_function_symbol(node: &Node, source: &str) -> Option<Symbol> {
    let decl = node.child_by_field_name("name").or_else(|| {
        // Find function_declaration child
        let mut cursor = node.walk();
        node.children(&mut cursor)
            .find(|c| c.kind() == "function_declaration")
    })?;

    let (name, scope, name_start, name_end) = extract_name_and_scope(&decl, source)?;

    // Extract parameters for signature
    let params = extract_function_params(&decl, source);
    let signature = format!("{}({})", name, params.join(", "));

    Some(Symbol {
        name,
        scope,
        kind: SymbolKind::Function,
        start: (name_start.row, name_start.column),
        end: (name_end.row, name_end.column),
        signature: Some(signature),
    })
}

fn extract_variable_symbol(node: &Node, source: &str) -> Option<Symbol> {
    // Find the identifier or scoped_identifier
    let mut cursor = node.walk();
    let name_node = node
        .children(&mut cursor)
        .find(|c| c.kind() == "identifier" || c.kind() == "scoped_identifier")?;

    let (name, scope, start, end) = extract_name_and_scope(&name_node, source)?;

    Some(Symbol {
        name,
        scope,
        kind: SymbolKind::Variable,
        start: (start.row, start.column),
        end: (end.row, end.column),
        signature: None,
    })
}

fn extract_name_and_scope(
    node: &Node,
    source: &str,
) -> Option<(String, VimScope, tree_sitter::Point, tree_sitter::Point)> {
    match node.kind() {
        "identifier" => {
            let name = node.utf8_text(source.as_bytes()).ok()?.to_string();
            Some((
                name,
                VimScope::Implicit,
                node.start_position(),
                node.end_position(),
            ))
        }
        "scoped_identifier" => {
            let mut cursor = node.walk();
            let children: Vec<_> = node.children(&mut cursor).collect();

            let scope_node = children.iter().find(|c| c.kind() == "scope")?;
            let ident_node = children.iter().find(|c| c.kind() == "identifier")?;

            let scope_text = scope_node.utf8_text(source.as_bytes()).ok()?;
            let name = ident_node.utf8_text(source.as_bytes()).ok()?.to_string();

            Some((
                name,
                VimScope::from_str(scope_text),
                ident_node.start_position(),
                ident_node.end_position(),
            ))
        }
        "function_declaration" => {
            // Look for identifier or scoped_identifier child
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "identifier" || child.kind() == "scoped_identifier" {
                    return extract_name_and_scope(&child, source);
                }
            }
            None
        }
        _ => None,
    }
}

fn extract_function_params(decl: &Node, source: &str) -> Vec<String> {
    let mut params = Vec::new();

    // Find parameters node
    let mut cursor = decl.walk();
    let params_node = decl
        .children(&mut cursor)
        .find(|c| c.kind() == "parameters");

    if let Some(params_node) = params_node {
        let mut cursor = params_node.walk();
        for child in params_node.children(&mut cursor) {
            if child.kind() == "identifier" {
                if let Ok(name) = child.utf8_text(source.as_bytes()) {
                    params.push(name.to_string());
                }
            }
        }
    }

    params
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    fn parse(code: &str) -> Tree {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_vim::language()).unwrap();
        parser.parse(code, None).unwrap()
    }

    #[test]
    fn test_extract_global_function() {
        let code = "function! MyFunc(a, b)\nendfunction";
        let tree = parse(code);
        let symbols = extract_symbols(&tree, code);

        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].name, "MyFunc");
        assert_eq!(symbols[0].scope, VimScope::Implicit);
        assert_eq!(symbols[0].kind, SymbolKind::Function);
        assert_eq!(symbols[0].signature, Some("MyFunc(a, b)".to_string()));
    }

    #[test]
    fn test_extract_script_local_function() {
        let code = "function! s:PrivateFunc()\nendfunction";
        let tree = parse(code);
        let symbols = extract_symbols(&tree, code);

        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].name, "PrivateFunc");
        assert_eq!(symbols[0].scope, VimScope::Script);
        assert_eq!(symbols[0].full_name(), "s:PrivateFunc");
    }

    #[test]
    fn test_extract_variables() {
        let code = "let g:global_var = 1\nlet s:script_var = 2";
        let tree = parse(code);
        let symbols = extract_symbols(&tree, code);

        assert_eq!(symbols.len(), 2);
        assert_eq!(symbols[0].name, "global_var");
        assert_eq!(symbols[0].scope, VimScope::Global);
        assert_eq!(symbols[1].name, "script_var");
        assert_eq!(symbols[1].scope, VimScope::Script);
    }

    #[test]
    fn test_extract_autoload_function() {
        // Autoload functions have their full name including #
        let code = "function! myplugin#util#helper()\n  return 42\nendfunction";
        let tree = parse(code);
        let symbols = extract_symbols(&tree, code);

        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].name, "myplugin#util#helper");
        assert_eq!(symbols[0].scope, VimScope::Implicit);
        assert_eq!(symbols[0].kind, SymbolKind::Function);
        assert_eq!(
            symbols[0].signature,
            Some("myplugin#util#helper()".to_string())
        );
    }
}
