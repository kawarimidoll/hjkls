//! Indentation processing for Vim script formatter
//!
//! This module handles:
//! - Block indentation (function/if/for/while/try/augroup)
//! - Line continuation indentation (\ at end of line)
//!
//! Uses tree-sitter AST to accurately determine indentation levels,
//! avoiding false positives on keywords inside comments or strings.

use tower_lsp_server::ls_types::{Position, Range, TextEdit};
use tree_sitter::Tree;

use super::FormatConfig;

/// Compute indentation edits for the source using AST analysis
pub fn compute_indent_edits(source: &str, tree: &Tree, config: &FormatConfig) -> Vec<TextEdit> {
    let mut edits = Vec::new();
    let lines: Vec<&str> = source.lines().collect();

    // Calculate expected indent levels for each line
    let indent_levels = compute_indent_levels(source, tree, config);

    for (line_num, (line, expected_indent)) in lines.iter().zip(indent_levels.iter()).enumerate() {
        if let Some(edit) = compute_line_indent_edit(line_num, line, *expected_indent, config) {
            edits.push(edit);
        }
    }

    edits
}

/// Compute expected indent level for each line using AST
fn compute_indent_levels(source: &str, tree: &Tree, config: &FormatConfig) -> Vec<usize> {
    let lines: Vec<&str> = source.lines().collect();
    let line_count = lines.len();
    let mut levels = vec![0usize; line_count];

    if lines.is_empty() {
        return levels;
    }

    let indent_width = config.indent_width;
    let line_continuation_indent = config.effective_line_continuation_indent();

    // First pass: compute base indentation from AST structure
    let root = tree.root_node();
    compute_ast_indent_levels(source, &root, &mut levels, indent_width);

    // Handle augroup blocks (they are parsed as separate nodes by tree-sitter)
    compute_augroup_indent_levels(source, &root, &mut levels, indent_width);

    // Second pass: handle line continuations (lines starting with \)
    let mut in_continuation = false;
    let mut continuation_base = 0usize;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }

        // Check if this line starts with \ (line continuation)
        if trimmed.starts_with('\\') {
            if !in_continuation {
                in_continuation = true;
                continuation_base = levels[i.saturating_sub(1)];
            }
            levels[i] = continuation_base + line_continuation_indent;
        } else if in_continuation {
            in_continuation = false;
        }
    }

    levels
}

/// Recursively compute indent levels from AST nodes
fn compute_ast_indent_levels(
    source: &str,
    node: &tree_sitter::Node,
    levels: &mut [usize],
    indent_width: usize,
) {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let child_kind = child.kind();

        // Check if this is a block-creating node
        if is_block_node(child_kind) {
            // Find the body children and indent them
            indent_block_body(source, &child, levels, indent_width);
        }

        // Recurse into children
        compute_ast_indent_levels(source, &child, levels, indent_width);
    }
}

/// Check if a node creates a new indentation block
fn is_block_node(kind: &str) -> bool {
    matches!(
        kind,
        "function_definition" | "if_statement" | "for_loop" | "while_loop" | "try_statement"
    )
}

/// Indent the body of a block node
fn indent_block_body(
    source: &str,
    node: &tree_sitter::Node,
    levels: &mut [usize],
    indent_width: usize,
) {
    let node_kind = node.kind();
    let start_line = node.start_position().row;
    let end_line = node.end_position().row;

    // Skip single-line blocks
    if start_line >= end_line {
        return;
    }

    let mut cursor = node.walk();
    let children: Vec<_> = node.children(&mut cursor).collect();

    match node_kind {
        "function_definition" | "for_loop" | "while_loop" => {
            // Indent body nodes
            for child in &children {
                if child.kind() == "body" {
                    indent_body_node(child, source, levels, indent_width);
                }
            }
        }
        "if_statement" => {
            // Handle if body and nested elseif/else statements
            indent_if_statement(source, &children, levels, indent_width);
        }
        "try_statement" => {
            // Handle try body and nested catch/finally statements
            indent_try_statement(source, &children, levels, indent_width);
        }
        _ => {}
    }
}

/// Indent all lines within a body node
fn indent_body_node(
    body: &tree_sitter::Node,
    source: &str,
    levels: &mut [usize],
    indent_width: usize,
) {
    let start_pos = body.start_position();
    let end = body.end_position().row;

    // Check if body starts mid-line (e.g., after `else` keyword)
    // vs starting at an indented position (already formatted code)
    let skip_first_line = if start_pos.column > 0 {
        // Check if there's non-whitespace content before body start on the same line
        if let Some(line_text) = source.lines().nth(start_pos.row) {
            let prefix = &line_text[..start_pos.column.min(line_text.len())];
            prefix.chars().any(|c| !c.is_whitespace())
        } else {
            false
        }
    } else {
        false
    };

    let start = if skip_first_line {
        start_pos.row + 1
    } else {
        start_pos.row
    };

    for line in start..end {
        if line < levels.len() {
            levels[line] += indent_width;
        }
    }
}

/// Handle if/elseif/else/endif indentation
fn indent_if_statement(
    source: &str,
    children: &[tree_sitter::Node],
    levels: &mut [usize],
    indent_width: usize,
) {
    for child in children {
        match child.kind() {
            "body" => {
                // Direct body of if
                indent_body_node(child, source, levels, indent_width);
            }
            "elseif_statement" | "else_statement" => {
                // Recurse into elseif/else to find their body
                let mut cursor = child.walk();
                for grandchild in child.children(&mut cursor) {
                    if grandchild.kind() == "body" {
                        indent_body_node(&grandchild, source, levels, indent_width);
                    }
                }
            }
            _ => {}
        }
    }
}

/// Handle try/catch/finally/endtry indentation
fn indent_try_statement(
    source: &str,
    children: &[tree_sitter::Node],
    levels: &mut [usize],
    indent_width: usize,
) {
    for child in children {
        match child.kind() {
            "body" => {
                // Direct body of try
                indent_body_node(child, source, levels, indent_width);
            }
            "catch_statement" | "finally_statement" => {
                // Recurse into catch/finally to find their body
                let mut cursor = child.walk();
                for grandchild in child.children(&mut cursor) {
                    if grandchild.kind() == "body" {
                        indent_body_node(&grandchild, source, levels, indent_width);
                    }
                }
            }
            _ => {}
        }
    }
}

/// Handle augroup blocks
/// tree-sitter parses `augroup Name` and `augroup END` as separate nodes,
/// so we need to find matching pairs and indent lines between them.
fn compute_augroup_indent_levels(
    source: &str,
    root: &tree_sitter::Node,
    levels: &mut [usize],
    indent_width: usize,
) {
    // Collect all augroup_statement nodes with their lines
    let mut augroup_lines: Vec<(usize, bool)> = Vec::new(); // (line, is_end)

    collect_augroup_nodes(root, source, &mut augroup_lines);

    // Sort by line number
    augroup_lines.sort_by_key(|(line, _)| *line);

    // Match opening augroups with closing ones
    let mut stack: Vec<usize> = Vec::new();

    for (line, is_end) in augroup_lines {
        if is_end {
            if let Some(open_line) = stack.pop() {
                // Indent lines between open_line and this line
                for l in (open_line + 1)..line {
                    if l < levels.len() {
                        levels[l] += indent_width;
                    }
                }
            }
        } else {
            stack.push(line);
        }
    }
}

/// Collect all augroup_statement nodes
fn collect_augroup_nodes(node: &tree_sitter::Node, source: &str, result: &mut Vec<(usize, bool)>) {
    if node.kind() == "augroup_statement" {
        let line = node.start_position().row;
        // Check if this is "augroup END"
        if let Some(line_text) = source.lines().nth(line) {
            let is_end = line_text.trim().eq_ignore_ascii_case("augroup END")
                || line_text.trim().eq_ignore_ascii_case("augroup! END");
            result.push((line, is_end));
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_augroup_nodes(&child, source, result);
    }
}

/// Create a TextEdit to fix indentation for a line
fn compute_line_indent_edit(
    line_num: usize,
    line: &str,
    expected_indent: usize,
    config: &FormatConfig,
) -> Option<TextEdit> {
    let trimmed = line.trim_start();

    // Skip empty lines (preserve their indentation)
    if trimmed.is_empty() {
        return None;
    }

    // Calculate current indentation
    let current_indent = line.len() - trimmed.len();
    let current_indent_chars: String = line.chars().take(current_indent).collect();

    // Calculate expected indentation string
    let expected_indent_str = if config.use_tabs {
        let tabs = expected_indent / config.indent_width;
        let spaces = expected_indent % config.indent_width;
        format!("{}{}", "\t".repeat(tabs), " ".repeat(spaces))
    } else {
        " ".repeat(expected_indent)
    };

    // Only create edit if indentation differs
    if current_indent_chars != expected_indent_str {
        Some(TextEdit {
            range: Range {
                start: Position {
                    line: line_num as u32,
                    character: 0,
                },
                end: Position {
                    line: line_num as u32,
                    character: current_indent as u32,
                },
            },
            new_text: expected_indent_str,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::FormatConfig;
    use tree_sitter::Parser;

    fn parse_vim(source: &str) -> Tree {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_vim::language())
            .expect("Error loading vim grammar");
        parser.parse(source, None).unwrap()
    }

    #[test]
    fn test_function_indentation() {
        let source = "function! Test()\nlet x = 1\nendfunction\n";
        let tree = parse_vim(source);
        let config = FormatConfig::default();

        let levels = compute_indent_levels(source, &tree, &config);

        assert_eq!(levels[0], 0); // function!
        assert_eq!(levels[1], 2); // let x = 1
        assert_eq!(levels[2], 0); // endfunction
    }

    #[test]
    fn test_function_indentation_already_indented() {
        let source = "function! Test()\n  let x = 1\nendfunction\n";
        let tree = parse_vim(source);
        let config = FormatConfig::default();

        let levels = compute_indent_levels(source, &tree, &config);

        // Should still return indent level 2 for line 1
        assert_eq!(levels[0], 0); // function!
        assert_eq!(levels[1], 2); // let x = 1 (already indented)
        assert_eq!(levels[2], 0); // endfunction
    }

    #[test]
    fn test_if_indentation() {
        let source = "if a == 1\nlet x = 1\nendif\n";
        let tree = parse_vim(source);
        let config = FormatConfig::default();

        let levels = compute_indent_levels(source, &tree, &config);

        assert_eq!(levels[0], 0); // if
        assert_eq!(levels[1], 2); // let x = 1
        assert_eq!(levels[2], 0); // endif
    }

    #[test]
    fn test_line_continuation() {
        let source = "let x = [\n\\ 'a',\n\\ 'b',\n\\ ]\n";
        let tree = parse_vim(source);
        let config = FormatConfig::default();

        let levels = compute_indent_levels(source, &tree, &config);

        assert_eq!(levels[0], 0); // let x = [
        assert_eq!(levels[1], 6); // \ 'a',
        assert_eq!(levels[2], 6); // \ 'b',
        assert_eq!(levels[3], 6); // \ ]
    }

    #[test]
    fn test_nested_structure() {
        let source = "function! Test()\nif a == 1\nlet x = 1\nendif\nendfunction\n";
        let tree = parse_vim(source);
        let config = FormatConfig::default();

        let levels = compute_indent_levels(source, &tree, &config);

        assert_eq!(levels[0], 0); // function!
        assert_eq!(levels[1], 2); // if
        assert_eq!(levels[2], 4); // let x = 1
        assert_eq!(levels[3], 2); // endif
        assert_eq!(levels[4], 0); // endfunction
    }

    #[test]
    fn test_else_elseif_indentation() {
        let source = "if a == 1\nlet x = 1\nelseif a == 2\nlet x = 2\nelse\nlet x = 3\nendif\n";
        let tree = parse_vim(source);
        let config = FormatConfig::default();

        let levels = compute_indent_levels(source, &tree, &config);

        assert_eq!(levels[0], 0); // if
        assert_eq!(levels[1], 2); // let x = 1
        assert_eq!(levels[2], 0); // elseif
        assert_eq!(levels[3], 2); // let x = 2
        assert_eq!(levels[4], 0); // else
        assert_eq!(levels[5], 2); // let x = 3
        assert_eq!(levels[6], 0); // endif
    }

    #[test]
    fn test_try_catch_finally() {
        let source = "try\nlet x = 1\ncatch\nlet y = 2\nfinally\nlet z = 3\nendtry\n";
        let tree = parse_vim(source);
        let config = FormatConfig::default();

        let levels = compute_indent_levels(source, &tree, &config);

        assert_eq!(levels[0], 0); // try
        assert_eq!(levels[1], 2); // let x = 1
        assert_eq!(levels[2], 0); // catch
        assert_eq!(levels[3], 2); // let y = 2
        assert_eq!(levels[4], 0); // finally
        assert_eq!(levels[5], 2); // let z = 3
        assert_eq!(levels[6], 0); // endtry
    }

    #[test]
    fn test_augroup() {
        let source = "augroup MyGroup\nautocmd!\naugroup END\n";
        let tree = parse_vim(source);
        let config = FormatConfig::default();

        let levels = compute_indent_levels(source, &tree, &config);

        assert_eq!(levels[0], 0); // augroup MyGroup
        assert_eq!(levels[1], 2); // autocmd!
        assert_eq!(levels[2], 0); // augroup END
    }

    #[test]
    fn test_for_loop_indentation() {
        let source = "for i in range(10)\necho i\nendfor\n";
        let tree = parse_vim(source);
        let config = FormatConfig::default();

        let levels = compute_indent_levels(source, &tree, &config);

        assert_eq!(levels[0], 0); // for
        assert_eq!(levels[1], 2); // echo i
        assert_eq!(levels[2], 0); // endfor
    }

    #[test]
    fn test_while_loop_indentation() {
        let source = "while i < 10\nlet i += 1\nendwhile\n";
        let tree = parse_vim(source);
        let config = FormatConfig::default();

        let levels = compute_indent_levels(source, &tree, &config);

        assert_eq!(levels[0], 0); // while
        assert_eq!(levels[1], 2); // let i += 1
        assert_eq!(levels[2], 0); // endwhile
    }

    #[test]
    fn test_comment_with_keyword_ignored() {
        // The keyword 'function' inside a comment should not affect indentation
        let source = "\" function! This is a comment\nlet x = 1\n";
        let tree = parse_vim(source);
        let config = FormatConfig::default();

        let levels = compute_indent_levels(source, &tree, &config);

        // Both lines should be at indent level 0
        assert_eq!(levels[0], 0);
        assert_eq!(levels[1], 0);
    }

    #[test]
    fn test_string_with_keyword_ignored() {
        // The keyword 'endif' inside a string should not affect indentation
        let source = "let msg = \"endif is not a keyword here\"\nlet x = 1\n";
        let tree = parse_vim(source);
        let config = FormatConfig::default();

        let levels = compute_indent_levels(source, &tree, &config);

        // Both lines should be at indent level 0
        assert_eq!(levels[0], 0);
        assert_eq!(levels[1], 0);
    }
}
