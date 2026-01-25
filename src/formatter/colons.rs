//! Dictionary colon spacing for Vim script formatter
//!
//! This module handles:
//! - Adding space after colons in dictionary entries
//! - Normalizing multiple spaces after colons to single space

use tower_lsp_server::ls_types::{Position, Range, TextEdit};
use tree_sitter::Tree;

/// Compute text edits to normalize dictionary colon spacing
pub fn compute_colon_edits(source: &str, tree: &Tree) -> Vec<TextEdit> {
    let mut edits = Vec::new();

    collect_colon_edits(&tree.root_node(), source, &mut edits);

    edits
}

/// Recursively collect colon spacing edits from AST
fn collect_colon_edits(node: &tree_sitter::Node, source: &str, edits: &mut Vec<TextEdit>) {
    // Check if this is a dictionary entry (contains key: value)
    // Note: tree-sitter-vim uses "dictionnary_entry" (with typo)
    if node.kind() == "dictionnary_entry" {
        collect_entry_colon_edits(node, source, edits);
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_colon_edits(&child, source, edits);
    }
}

/// Collect edits for colons within a dictionary entry
///
/// Note: `_source` is unused but kept for API consistency and potential future use.
fn collect_entry_colon_edits(node: &tree_sitter::Node, _source: &str, edits: &mut Vec<TextEdit>) {
    let mut cursor = node.walk();
    let children: Vec<_> = node.children(&mut cursor).collect();

    // dictionnary_entry has: key, ":", value
    for (i, child) in children.iter().enumerate() {
        if child.kind() == ":" {
            let colon_end = child.end_position();

            // Get the value (next sibling after colon)
            if i + 1 < children.len() {
                let value = &children[i + 1];
                let value_start = value.start_position();

                // Check if colon and value are on the same line
                if colon_end.row == value_start.row {
                    let gap = value_start.column.saturating_sub(colon_end.column);

                    if gap == 0 {
                        // No space after colon - add one
                        edits.push(TextEdit {
                            range: Range {
                                start: Position {
                                    line: colon_end.row as u32,
                                    character: colon_end.column as u32,
                                },
                                end: Position {
                                    line: colon_end.row as u32,
                                    character: colon_end.column as u32,
                                },
                            },
                            new_text: " ".to_string(),
                        });
                    } else if gap > 1 {
                        // Multiple spaces after colon - normalize to one
                        edits.push(TextEdit {
                            range: Range {
                                start: Position {
                                    line: colon_end.row as u32,
                                    character: colon_end.column as u32,
                                },
                                end: Position {
                                    line: value_start.row as u32,
                                    character: value_start.column as u32,
                                },
                            },
                            new_text: " ".to_string(),
                        });
                    }
                }
                // If on different lines, don't modify (multiline dicts are intentional)
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
    fn test_dict_no_space_after_colon() {
        let source = "let d = {'a':1}\n";
        let tree = parse_vim(source);
        let edits = compute_colon_edits(source, &tree);

        // Should have 1 edit: add space after colon
        assert_eq!(edits.len(), 1, "Edits: {:?}", edits);
    }

    #[test]
    fn test_dict_correct_spacing() {
        let source = "let d = {'a': 1}\n";
        let tree = parse_vim(source);
        let edits = compute_colon_edits(source, &tree);

        // Should have no edits - spacing is correct
        assert!(edits.is_empty(), "Edits: {:?}", edits);
    }

    #[test]
    fn test_dict_multiple_entries() {
        let source = "let d = {'a':1, 'b':2, 'c':3}\n";
        let tree = parse_vim(source);
        let edits = compute_colon_edits(source, &tree);

        // Should have 3 edits (one for each entry)
        assert_eq!(edits.len(), 3, "Edits: {:?}", edits);
    }

    #[test]
    fn test_dict_multiple_spaces_after_colon() {
        let source = "let d = {'a':   1}\n";
        let tree = parse_vim(source);
        let edits = compute_colon_edits(source, &tree);

        // Should have 1 edit to normalize spaces
        assert_eq!(edits.len(), 1, "Edits: {:?}", edits);
        assert_eq!(edits[0].new_text, " ");
    }

    #[test]
    fn test_nested_dict() {
        let source = "let d = {'a':{'b':1}}\n";
        let tree = parse_vim(source);
        let edits = compute_colon_edits(source, &tree);

        // Should have 2 edits (outer and inner dict)
        assert_eq!(edits.len(), 2, "Edits: {:?}", edits);
    }

    #[test]
    fn test_dict_with_string_value() {
        let source = "let d = {'key':'value'}\n";
        let tree = parse_vim(source);
        let edits = compute_colon_edits(source, &tree);

        // Should have 1 edit
        assert_eq!(edits.len(), 1, "Edits: {:?}", edits);
    }

    #[test]
    fn test_dict_with_expression_value() {
        let source = "let d = {'key':a + b}\n";
        let tree = parse_vim(source);
        let edits = compute_colon_edits(source, &tree);

        // Should have 1 edit
        assert_eq!(edits.len(), 1, "Edits: {:?}", edits);
    }

    #[test]
    fn test_empty_dict() {
        let source = "let d = {}\n";
        let tree = parse_vim(source);
        let edits = compute_colon_edits(source, &tree);

        // Should have no edits
        assert!(edits.is_empty(), "Edits: {:?}", edits);
    }

    #[test]
    fn test_dict_mixed_spacing() {
        // Some entries have correct spacing, some don't
        let source = "let d = {'a': 1, 'b':2}\n";
        let tree = parse_vim(source);
        let edits = compute_colon_edits(source, &tree);

        // Should have 1 edit (only for 'b':2)
        assert_eq!(edits.len(), 1, "Edits: {:?}", edits);
    }
}
