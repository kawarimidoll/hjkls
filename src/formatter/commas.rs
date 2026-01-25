//! Comma spacing for Vim script formatter
//!
//! This module handles:
//! - Adding space after commas in function calls, lists, and dictionaries
//! - Normalizing multiple spaces after commas to single space

use tower_lsp_server::ls_types::{Position, Range, TextEdit};
use tree_sitter::Tree;

/// Node types that contain comma-separated elements
const COMMA_CONTAINERS: &[&str] = &[
    "call_expression", // func(a, b, c)
    "list",            // [1, 2, 3]
    "dictionnary",     // {'a': 1, 'b': 2} (typo in tree-sitter-vim)
    "parameters",      // function! F(a, b, c)
];

/// Compute text edits to normalize comma spacing
pub fn compute_comma_edits(source: &str, tree: &Tree) -> Vec<TextEdit> {
    let mut edits = Vec::new();

    collect_comma_edits(&tree.root_node(), source, &mut edits);

    edits
}

/// Recursively collect comma spacing edits from AST
fn collect_comma_edits(node: &tree_sitter::Node, source: &str, edits: &mut Vec<TextEdit>) {
    let kind = node.kind();

    // Check if this is a container with comma-separated elements
    if COMMA_CONTAINERS.contains(&kind) {
        collect_container_comma_edits(node, source, edits);
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_comma_edits(&child, source, edits);
    }
}

/// Collect edits for commas within a container node
///
/// Note: `_source` is unused but kept for API consistency and potential future use
/// for context-aware formatting.
fn collect_container_comma_edits(
    node: &tree_sitter::Node,
    _source: &str,
    edits: &mut Vec<TextEdit>,
) {
    let mut cursor = node.walk();
    let children: Vec<_> = node.children(&mut cursor).collect();

    for (i, child) in children.iter().enumerate() {
        if child.kind() == "," {
            let comma_end = child.end_position();

            // Get the next sibling (skip if it's the closing bracket/paren)
            if i + 1 < children.len() {
                let next = &children[i + 1];
                let next_kind = next.kind();

                // Skip if next is closing bracket/paren/brace
                if next_kind == ")" || next_kind == "]" || next_kind == "}" {
                    continue;
                }

                let next_start = next.start_position();

                // Check if comma and next element are on the same line
                if comma_end.row == next_start.row {
                    let gap = next_start.column.saturating_sub(comma_end.column);

                    if gap == 0 {
                        // No space after comma - add one
                        edits.push(TextEdit {
                            range: Range {
                                start: Position {
                                    line: comma_end.row as u32,
                                    character: comma_end.column as u32,
                                },
                                end: Position {
                                    line: comma_end.row as u32,
                                    character: comma_end.column as u32,
                                },
                            },
                            new_text: " ".to_string(),
                        });
                    } else if gap > 1 {
                        // Multiple spaces after comma - normalize to one
                        edits.push(TextEdit {
                            range: Range {
                                start: Position {
                                    line: comma_end.row as u32,
                                    character: comma_end.column as u32,
                                },
                                end: Position {
                                    line: next_start.row as u32,
                                    character: next_start.column as u32,
                                },
                            },
                            new_text: " ".to_string(),
                        });
                    }
                }
                // If on different lines, don't modify (multiline lists are intentional)
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
    fn test_function_call_no_space() {
        let source = "call Test(a,b,c)\n";
        let tree = parse_vim(source);
        let edits = compute_comma_edits(source, &tree);

        // Should have 2 edits: add space after each comma
        assert_eq!(edits.len(), 2, "Edits: {:?}", edits);
    }

    #[test]
    fn test_function_call_correct_spacing() {
        let source = "call Test(a, b, c)\n";
        let tree = parse_vim(source);
        let edits = compute_comma_edits(source, &tree);

        // Should have no edits - spacing is correct
        assert!(edits.is_empty(), "Edits: {:?}", edits);
    }

    #[test]
    fn test_list_no_space() {
        let source = "let x = [1,2,3]\n";
        let tree = parse_vim(source);
        let edits = compute_comma_edits(source, &tree);

        // Should have 2 edits
        assert_eq!(edits.len(), 2, "Edits: {:?}", edits);
    }

    #[test]
    fn test_list_correct_spacing() {
        let source = "let x = [1, 2, 3]\n";
        let tree = parse_vim(source);
        let edits = compute_comma_edits(source, &tree);

        assert!(edits.is_empty(), "Edits: {:?}", edits);
    }

    #[test]
    fn test_dictionary_no_space() {
        let source = "let d = {'a': 1,'b': 2}\n";
        let tree = parse_vim(source);
        let edits = compute_comma_edits(source, &tree);

        // Should have 1 edit (space after comma between entries)
        assert_eq!(edits.len(), 1, "Edits: {:?}", edits);
    }

    #[test]
    fn test_function_parameters() {
        let source = "function! F(a,b,c)\nendfunction\n";
        let tree = parse_vim(source);
        let edits = compute_comma_edits(source, &tree);

        // Should have 2 edits
        assert_eq!(edits.len(), 2, "Edits: {:?}", edits);
    }

    #[test]
    fn test_multiple_spaces_after_comma() {
        let source = "call Test(a,   b,  c)\n";
        let tree = parse_vim(source);
        let edits = compute_comma_edits(source, &tree);

        // Should have 2 edits to normalize spaces
        assert_eq!(edits.len(), 2, "Edits: {:?}", edits);
        // Each edit should replace multiple spaces with single space
        for edit in &edits {
            assert_eq!(edit.new_text, " ");
        }
    }

    #[test]
    fn test_trailing_comma_ignored() {
        // Trailing comma before closing bracket - no space needed
        let source = "let x = [1, 2, 3,]\n";
        let tree = parse_vim(source);
        let edits = compute_comma_edits(source, &tree);

        // Should have no edits (trailing comma before ] is fine)
        assert!(edits.is_empty(), "Edits: {:?}", edits);
    }

    #[test]
    fn test_multiline_list() {
        // Multiline lists should not be modified
        let source = "let x = [\n\\ 1,\n\\ 2,\n\\ ]\n";
        let tree = parse_vim(source);
        let edits = compute_comma_edits(source, &tree);

        // Commas at end of line - no modification needed
        assert!(edits.is_empty(), "Edits: {:?}", edits);
    }

    #[test]
    fn test_nested_calls() {
        let source = "call Outer(Inner(a,b),c)\n";
        let tree = parse_vim(source);
        let edits = compute_comma_edits(source, &tree);

        // Should have 2 edits: Inner(a,b) and Outer(...,c)
        assert_eq!(edits.len(), 2, "Edits: {:?}", edits);
    }
}
