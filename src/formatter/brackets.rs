//! Bracket spacing for Vim script formatter
//!
//! This module handles:
//! - Removing space after opening brackets: `( x` → `(x`
//! - Removing space before closing brackets: `x )` → `x)`
//! - Applies to parentheses (), square brackets [], and curly braces {}

use tower_lsp_server::ls_types::{Position, Range, TextEdit};
use tree_sitter::Tree;

/// Node types that contain bracketed content
const BRACKET_CONTAINERS: &[&str] = &[
    "call_expression", // func(a, b)
    "list",            // [1, 2, 3]
    "dictionnary",     // {'a': 1} (typo in tree-sitter-vim)
    "parameters",      // function! F(a, b)
];

/// Opening brackets
const OPEN_BRACKETS: &[&str] = &["(", "[", "{"];

/// Closing brackets
const CLOSE_BRACKETS: &[&str] = &[")", "]", "}"];

/// Compute text edits to remove spaces inside brackets
pub fn compute_bracket_edits(source: &str, tree: &Tree) -> Vec<TextEdit> {
    let mut edits = Vec::new();

    collect_bracket_edits(&tree.root_node(), source, &mut edits);

    edits
}

/// Recursively collect bracket spacing edits from AST
fn collect_bracket_edits(node: &tree_sitter::Node, source: &str, edits: &mut Vec<TextEdit>) {
    let kind = node.kind();

    // Check if this is a container with brackets
    if BRACKET_CONTAINERS.contains(&kind) {
        collect_container_bracket_edits(node, source, edits);
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_bracket_edits(&child, source, edits);
    }
}

/// Collect edits to remove spaces inside brackets within a container node
///
/// Note: `_source` is unused but kept for API consistency and potential future use.
fn collect_container_bracket_edits(
    node: &tree_sitter::Node,
    _source: &str,
    edits: &mut Vec<TextEdit>,
) {
    let mut cursor = node.walk();
    let children: Vec<_> = node.children(&mut cursor).collect();

    for (i, child) in children.iter().enumerate() {
        let kind = child.kind();

        // Handle opening brackets - remove space after
        if OPEN_BRACKETS.contains(&kind) {
            if i + 1 < children.len() {
                let next = &children[i + 1];
                let bracket_end = child.end_position();
                let next_start = next.start_position();

                // Only process if on same line
                if bracket_end.row == next_start.row {
                    let gap = next_start.column.saturating_sub(bracket_end.column);
                    if gap > 0 {
                        // Remove space after opening bracket
                        edits.push(TextEdit {
                            range: Range {
                                start: Position {
                                    line: bracket_end.row as u32,
                                    character: bracket_end.column as u32,
                                },
                                end: Position {
                                    line: next_start.row as u32,
                                    character: next_start.column as u32,
                                },
                            },
                            new_text: String::new(),
                        });
                    }
                }
            }
        }

        // Handle closing brackets - remove space before
        if CLOSE_BRACKETS.contains(&kind) {
            if i > 0 {
                let prev = &children[i - 1];
                let prev_end = prev.end_position();
                let bracket_start = child.start_position();

                // Only process if on same line
                if prev_end.row == bracket_start.row {
                    let gap = bracket_start.column.saturating_sub(prev_end.column);
                    if gap > 0 {
                        // Remove space before closing bracket
                        edits.push(TextEdit {
                            range: Range {
                                start: Position {
                                    line: prev_end.row as u32,
                                    character: prev_end.column as u32,
                                },
                                end: Position {
                                    line: bracket_start.row as u32,
                                    character: bracket_start.column as u32,
                                },
                            },
                            new_text: String::new(),
                        });
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    fn parse_vim(source: &str) -> Tree {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_vim::language())
            .expect("Error loading vim grammar");
        parser.parse(source, None).unwrap()
    }

    #[test]
    fn test_function_call_space_inside() {
        let source = "call Test( a, b )\n";
        let tree = parse_vim(source);
        let edits = compute_bracket_edits(source, &tree);

        // Should have 2 edits: remove space after ( and before )
        assert_eq!(edits.len(), 2, "Edits: {:?}", edits);
    }

    #[test]
    fn test_function_call_no_space() {
        let source = "call Test(a, b)\n";
        let tree = parse_vim(source);
        let edits = compute_bracket_edits(source, &tree);

        // Should have no edits - no spaces inside brackets
        assert!(edits.is_empty(), "Edits: {:?}", edits);
    }

    #[test]
    fn test_list_space_inside() {
        let source = "let x = [ 1, 2, 3 ]\n";
        let tree = parse_vim(source);
        let edits = compute_bracket_edits(source, &tree);

        // Should have 2 edits
        assert_eq!(edits.len(), 2, "Edits: {:?}", edits);
    }

    #[test]
    fn test_list_no_space() {
        let source = "let x = [1, 2, 3]\n";
        let tree = parse_vim(source);
        let edits = compute_bracket_edits(source, &tree);

        assert!(edits.is_empty(), "Edits: {:?}", edits);
    }

    #[test]
    fn test_dict_space_inside() {
        let source = "let d = { 'a': 1 }\n";
        let tree = parse_vim(source);
        let edits = compute_bracket_edits(source, &tree);

        // Should have 2 edits
        assert_eq!(edits.len(), 2, "Edits: {:?}", edits);
    }

    #[test]
    fn test_dict_no_space() {
        let source = "let d = {'a': 1}\n";
        let tree = parse_vim(source);
        let edits = compute_bracket_edits(source, &tree);

        assert!(edits.is_empty(), "Edits: {:?}", edits);
    }

    #[test]
    fn test_function_params_space_inside() {
        let source = "function! F( a, b )\nendfunction\n";
        let tree = parse_vim(source);
        let edits = compute_bracket_edits(source, &tree);

        // Should have 2 edits
        assert_eq!(edits.len(), 2, "Edits: {:?}", edits);
    }

    #[test]
    fn test_only_space_after_open() {
        let source = "call Test( a, b)\n";
        let tree = parse_vim(source);
        let edits = compute_bracket_edits(source, &tree);

        // Should have 1 edit (only after opening paren)
        assert_eq!(edits.len(), 1, "Edits: {:?}", edits);
    }

    #[test]
    fn test_only_space_before_close() {
        let source = "call Test(a, b )\n";
        let tree = parse_vim(source);
        let edits = compute_bracket_edits(source, &tree);

        // Should have 1 edit (only before closing paren)
        assert_eq!(edits.len(), 1, "Edits: {:?}", edits);
    }

    #[test]
    fn test_empty_parens() {
        let source = "call Test()\n";
        let tree = parse_vim(source);
        let edits = compute_bracket_edits(source, &tree);

        // Should have no edits
        assert!(edits.is_empty(), "Edits: {:?}", edits);
    }

    #[test]
    fn test_empty_list() {
        let source = "let x = []\n";
        let tree = parse_vim(source);
        let edits = compute_bracket_edits(source, &tree);

        assert!(edits.is_empty(), "Edits: {:?}", edits);
    }

    #[test]
    fn test_nested_brackets() {
        let source = "call Outer( Inner( a ) )\n";
        let tree = parse_vim(source);
        let edits = compute_bracket_edits(source, &tree);

        // Should have 4 edits (2 for outer, 2 for inner)
        assert_eq!(edits.len(), 4, "Edits: {:?}", edits);
    }

    #[test]
    fn test_multiple_spaces() {
        let source = "call Test(   a,  b   )\n";
        let tree = parse_vim(source);
        let edits = compute_bracket_edits(source, &tree);

        // Should have 2 edits (multiple spaces treated same as single)
        assert_eq!(edits.len(), 2, "Edits: {:?}", edits);
    }

    #[test]
    fn test_multiline_brackets_preserved() {
        // Multiline brackets should not be modified (intentional formatting)
        let source = "call Test(\n  a, b\n)\n";
        let tree = parse_vim(source);
        let edits = compute_bracket_edits(source, &tree);

        // Different lines - no edits
        assert!(edits.is_empty(), "Edits: {:?}", edits);
    }

    #[test]
    fn test_multiline_list_preserved() {
        let source = "let x = [\n  1,\n  2,\n]\n";
        let tree = parse_vim(source);
        let edits = compute_bracket_edits(source, &tree);

        assert!(edits.is_empty(), "Edits: {:?}", edits);
    }
}
