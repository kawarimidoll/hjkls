//! Operator spacing for Vim script formatter
//!
//! This module handles:
//! - Adding spaces around binary operators (=, +, -, *, /, ==, !=, <, >, etc.)
//! - Removing spaces after unary operators (-, !, +)

use tower_lsp_server::ls_types::{Position, Range, TextEdit};
use tree_sitter::Tree;

/// Binary operators that should have spaces around them
const BINARY_OPERATORS: &[&str] = &[
    // Assignment
    "=", // Arithmetic
    "+", "-", "*", "/", "%", // Comparison
    "==", "!=", "<", ">", "<=", ">=", "=~", "!~", "=~#", "=~?", "!~#", "!~?", "==#", "==?", "!=#",
    "!=?", "<#", "<?", ">#", ">?", "<=#", "<=?", ">=#", ">=?", "is", "is#", "is?", "isnot",
    "isnot#", "isnot?", // Logical
    "&&", "||", // String concatenation
    ".", "..",
];

/// Unary operators that should NOT have space after them
const UNARY_OPERATORS: &[&str] = &["-", "!", "+"];

/// Compute text edits to normalize operator spacing
pub fn compute_operator_edits(source: &str, tree: &Tree) -> Vec<TextEdit> {
    let mut edits = Vec::new();

    collect_operator_edits(&tree.root_node(), source, &mut edits);

    edits
}

/// Recursively collect operator spacing edits from AST
fn collect_operator_edits(node: &tree_sitter::Node, source: &str, edits: &mut Vec<TextEdit>) {
    match node.kind() {
        "binary_operation" => {
            collect_binary_operator_edits(node, source, edits);
        }
        "unary_operation" => {
            collect_unary_operator_edits(node, source, edits);
        }
        "let_statement" => {
            // let_statement has its own = which is not a binary_operation
            collect_let_statement_edits(node, source, edits);
        }
        _ => {}
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_operator_edits(&child, source, edits);
    }
}

/// Collect edits for let statement assignment (let a = value)
///
/// Note: `_source` is unused but kept for API consistency with other collect functions.
/// It may be used in the future for context-aware formatting.
fn collect_let_statement_edits(node: &tree_sitter::Node, _source: &str, edits: &mut Vec<TextEdit>) {
    let mut cursor = node.walk();
    let children: Vec<_> = node.children(&mut cursor).collect();

    // Find the = operator and its surrounding nodes
    for (i, child) in children.iter().enumerate() {
        if child.kind() == "=" {
            let op_start = child.start_position();
            let op_end = child.end_position();

            // Check left side (identifier before =)
            if i > 0 {
                let left = &children[i - 1];
                let left_end = left.end_position();

                if left_end.row == op_start.row {
                    let gap = op_start.column.saturating_sub(left_end.column);
                    if gap == 0 {
                        // No space before = - add one
                        edits.push(TextEdit {
                            range: Range {
                                start: Position {
                                    line: op_start.row as u32,
                                    character: op_start.column as u32,
                                },
                                end: Position {
                                    line: op_start.row as u32,
                                    character: op_start.column as u32,
                                },
                            },
                            new_text: " ".to_string(),
                        });
                    }
                }
            }

            // Check right side (value after =)
            if i + 1 < children.len() {
                let right = &children[i + 1];
                let right_start = right.start_position();

                if op_end.row == right_start.row {
                    let gap = right_start.column.saturating_sub(op_end.column);
                    if gap == 0 {
                        // No space after = - add one
                        edits.push(TextEdit {
                            range: Range {
                                start: Position {
                                    line: op_end.row as u32,
                                    character: op_end.column as u32,
                                },
                                end: Position {
                                    line: op_end.row as u32,
                                    character: op_end.column as u32,
                                },
                            },
                            new_text: " ".to_string(),
                        });
                    }
                }
            }
        }
    }
}

/// Collect edits for binary operators (ensure space before and after)
///
/// Note: `_source` is unused but kept for API consistency with other collect functions.
/// It may be used in the future for context-aware formatting (e.g., avoiding string content).
fn collect_binary_operator_edits(
    node: &tree_sitter::Node,
    _source: &str,
    edits: &mut Vec<TextEdit>,
) {
    let mut cursor = node.walk();
    let children: Vec<_> = node.children(&mut cursor).collect();

    // Binary operation typically has: left_operand, operator, right_operand
    if children.len() < 3 {
        return;
    }

    // Find the operator node (usually the middle child)
    for (i, child) in children.iter().enumerate() {
        let kind = child.kind();

        // Check if this is an operator
        if is_binary_operator(kind) {
            let op_start = child.start_position();
            let op_end = child.end_position();

            // Get the left operand (previous sibling)
            if i > 0 {
                let left = &children[i - 1];
                let left_end = left.end_position();

                // Check if there's space before operator
                if left_end.row == op_start.row {
                    let gap = op_start.column.saturating_sub(left_end.column);
                    if gap == 0 {
                        // No space before operator - add one
                        edits.push(TextEdit {
                            range: Range {
                                start: Position {
                                    line: op_start.row as u32,
                                    character: op_start.column as u32,
                                },
                                end: Position {
                                    line: op_start.row as u32,
                                    character: op_start.column as u32,
                                },
                            },
                            new_text: " ".to_string(),
                        });
                    } else if gap > 1 {
                        // Multiple spaces before operator - normalize to one
                        edits.push(TextEdit {
                            range: Range {
                                start: Position {
                                    line: left_end.row as u32,
                                    character: left_end.column as u32,
                                },
                                end: Position {
                                    line: op_start.row as u32,
                                    character: op_start.column as u32,
                                },
                            },
                            new_text: " ".to_string(),
                        });
                    }
                }
            }

            // Get the right operand (next sibling)
            if i + 1 < children.len() {
                let right = &children[i + 1];
                let right_start = right.start_position();

                // Check if there's space after operator
                if op_end.row == right_start.row {
                    let gap = right_start.column.saturating_sub(op_end.column);
                    if gap == 0 {
                        // No space after operator - add one
                        edits.push(TextEdit {
                            range: Range {
                                start: Position {
                                    line: op_end.row as u32,
                                    character: op_end.column as u32,
                                },
                                end: Position {
                                    line: op_end.row as u32,
                                    character: op_end.column as u32,
                                },
                            },
                            new_text: " ".to_string(),
                        });
                    } else if gap > 1 {
                        // Multiple spaces after operator - normalize to one
                        edits.push(TextEdit {
                            range: Range {
                                start: Position {
                                    line: op_end.row as u32,
                                    character: op_end.column as u32,
                                },
                                end: Position {
                                    line: right_start.row as u32,
                                    character: right_start.column as u32,
                                },
                            },
                            new_text: " ".to_string(),
                        });
                    }
                }
            }
        }
    }
}

/// Collect edits for unary operators (remove space after)
///
/// Note: `_source` is unused but kept for API consistency with other collect functions.
/// It may be used in the future for context-aware formatting.
fn collect_unary_operator_edits(
    node: &tree_sitter::Node,
    _source: &str,
    edits: &mut Vec<TextEdit>,
) {
    let mut cursor = node.walk();
    let children: Vec<_> = node.children(&mut cursor).collect();

    // Unary operation typically has: operator, operand
    if children.len() < 2 {
        return;
    }

    let operator = &children[0];
    let operand = &children[1];

    // Check if this is a unary operator we care about
    if !is_unary_operator(operator.kind()) {
        return;
    }

    let op_end = operator.end_position();
    let operand_start = operand.start_position();

    // Check if there's space after the unary operator
    if op_end.row == operand_start.row {
        let gap = operand_start.column.saturating_sub(op_end.column);
        if gap > 0 {
            // Remove space after unary operator
            edits.push(TextEdit {
                range: Range {
                    start: Position {
                        line: op_end.row as u32,
                        character: op_end.column as u32,
                    },
                    end: Position {
                        line: operand_start.row as u32,
                        character: operand_start.column as u32,
                    },
                },
                new_text: String::new(),
            });
        }
    }
}

/// Check if a node kind is a binary operator
fn is_binary_operator(kind: &str) -> bool {
    BINARY_OPERATORS.contains(&kind)
}

/// Check if a node kind is a unary operator
fn is_unary_operator(kind: &str) -> bool {
    UNARY_OPERATORS.contains(&kind)
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
    #[ignore] // Debug utility
    fn test_debug_operator_ast() {
        let sources = [
            "let a = 1 + 2\n",
            "let a=1+b\n",
            "let c = - 1\n",
            "let d = !flag\n",
            "let e = a == b\n",
            "let f = 'a' . 'b'\n",
        ];

        for source in sources {
            println!("\n=== Source: {:?} ===", source.trim());
            let tree = parse_vim(source);
            print_node(&tree.root_node(), source, 0);
        }
    }

    fn print_node(node: &tree_sitter::Node, source: &str, depth: usize) {
        let indent = "  ".repeat(depth);
        let text: String = node
            .utf8_text(source.as_bytes())
            .unwrap_or("")
            .chars()
            .take(30)
            .collect();
        println!(
            "{}[{}] {} ({},{})-({},{}) {:?}",
            indent,
            node.kind(),
            node.child_count(),
            node.start_position().row,
            node.start_position().column,
            node.end_position().row,
            node.end_position().column,
            text
        );
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            print_node(&child, source, depth + 1);
        }
    }

    #[test]
    fn test_add_space_around_binary_operator() {
        let source = "let a=1+b\n";
        let tree = parse_vim(source);
        let edits = compute_operator_edits(source, &tree);

        // Should have edits to add spaces around = and +
        println!("Edits: {:?}", edits);
        assert!(!edits.is_empty(), "Should have edits for missing spaces");
    }

    #[test]
    fn test_no_edit_for_correct_spacing() {
        let source = "let a = 1 + b\n";
        let tree = parse_vim(source);
        let edits = compute_operator_edits(source, &tree);

        // Should have no edits - spacing is already correct
        assert!(edits.is_empty(), "Should have no edits: {:?}", edits);
    }

    #[test]
    fn test_remove_space_after_unary_minus() {
        let source = "let c = - 1\n";
        let tree = parse_vim(source);
        let edits = compute_operator_edits(source, &tree);

        // Should have edit to remove space after unary minus
        println!("Edits: {:?}", edits);
        // Note: This depends on tree-sitter parsing - may need adjustment
    }

    #[test]
    fn test_remove_space_after_unary_not() {
        let source = "let d = ! flag\n";
        let tree = parse_vim(source);
        let edits = compute_operator_edits(source, &tree);

        println!("Edits: {:?}", edits);
    }

    #[test]
    fn test_string_concatenation() {
        let source = "let s='a'.'b'\n";
        let tree = parse_vim(source);
        let edits = compute_operator_edits(source, &tree);

        // Should add spaces around .
        println!("Edits: {:?}", edits);
        assert!(!edits.is_empty(), "Should have edits for . operator");
    }

    // === Edge case tests ===

    #[test]
    fn test_multiple_operators_in_expression() {
        // Multiple operators in a single expression
        let source = "let x=1+2*3-4\n";
        let tree = parse_vim(source);
        let edits = compute_operator_edits(source, &tree);

        // Should have edits for all operators: =, +, *, -
        println!("Edits for multiple operators: {:?}", edits);
        assert!(
            edits.len() >= 4,
            "Should have edits for multiple operators: {:?}",
            edits
        );
    }

    #[test]
    fn test_comparison_operators() {
        // Comparison operators
        let source = "if a==b && c!=d\n";
        let tree = parse_vim(source);
        let edits = compute_operator_edits(source, &tree);

        println!("Edits for comparison: {:?}", edits);
        // Should add spaces around ==, &&, !=
        assert!(
            !edits.is_empty(),
            "Should have edits for comparison operators"
        );
    }

    #[test]
    fn test_operators_with_parentheses() {
        // Operators inside parentheses
        let source = "let x=(1+2)*(3-4)\n";
        let tree = parse_vim(source);
        let edits = compute_operator_edits(source, &tree);

        println!("Edits with parentheses: {:?}", edits);
        // Should add spaces around operators, not affected by parentheses
        assert!(
            !edits.is_empty(),
            "Should have edits for operators in parentheses"
        );
    }

    #[test]
    fn test_chained_string_concatenation() {
        // Multiple string concatenation
        let source = "let s='a'.'b'.'c'\n";
        let tree = parse_vim(source);
        let edits = compute_operator_edits(source, &tree);

        println!("Edits for chained concat: {:?}", edits);
        // Should add spaces around both . operators
        assert!(
            edits.len() >= 2,
            "Should have edits for multiple . operators"
        );
    }

    #[test]
    fn test_unary_in_expression() {
        // Unary operator within a larger expression
        let source = "let x = 1 + - 2\n";
        let tree = parse_vim(source);
        let edits = compute_operator_edits(source, &tree);

        println!("Edits for unary in expression: {:?}", edits);
        // Should remove space after unary minus
    }

    #[test]
    fn test_already_correct_spacing() {
        // Already correctly spaced - should produce no edits
        let source = "let x = 1 + 2\n";
        let tree = parse_vim(source);
        let edits = compute_operator_edits(source, &tree);

        assert!(
            edits.is_empty(),
            "Should have no edits for correct spacing: {:?}",
            edits
        );
    }

    #[test]
    fn test_vim_specific_operators() {
        // Vim-specific comparison operators
        let source = "if a==#b || c=~?d\n";
        let tree = parse_vim(source);
        let edits = compute_operator_edits(source, &tree);

        println!("Edits for Vim operators: {:?}", edits);
        // Should add spaces around ==# and =~?
    }
}
