//! Whitespace normalization for Vim script formatter
//!
//! This module handles:
//! - Normalizing multiple consecutive spaces to single space
//! - Preserving whitespace inside string literals and comments

use tower_lsp_server::ls_types::{Position, Range, TextEdit};
use tree_sitter::Tree;

/// Compute text edits to normalize whitespace (multiple spaces → single space)
///
/// Preserves whitespace inside:
/// - String literals (both single and double quoted)
/// - Comments (lines starting with " or #, or trailing comments)
pub fn compute_space_edits(source: &str, tree: &Tree) -> Vec<TextEdit> {
    let mut edits = Vec::new();

    // Collect protected ranges (strings and comments) where we shouldn't normalize
    let protected_ranges = collect_protected_ranges(source, tree);

    // Process each line with cumulative byte offset (O(n) instead of O(n²))
    let mut line_start_byte = 0;
    for (line_num, line) in source.lines().enumerate() {
        edits.extend(normalize_line_spaces(
            line_num,
            line,
            line_start_byte,
            &protected_ranges,
        ));
        line_start_byte += line.len() + 1; // +1 for newline
    }

    edits
}

/// A byte range that should be protected from whitespace normalization
#[derive(Debug, Clone)]
struct ProtectedRange {
    start_byte: usize,
    end_byte: usize,
}

impl ProtectedRange {
    fn contains(&self, byte_offset: usize) -> bool {
        byte_offset >= self.start_byte && byte_offset < self.end_byte
    }
}

/// Collect all ranges that should be protected from normalization
fn collect_protected_ranges(source: &str, tree: &Tree) -> Vec<ProtectedRange> {
    let mut ranges = Vec::new();

    // Collect string literals from AST
    collect_string_ranges(&tree.root_node(), &mut ranges);

    // Collect comment ranges (heuristic-based since tree-sitter-vim may not have comment nodes)
    collect_comment_ranges(source, &mut ranges);

    // Sort by start position for efficient lookup
    ranges.sort_by_key(|r| r.start_byte);

    ranges
}

/// Recursively collect string_literal and comment ranges from AST
fn collect_string_ranges(node: &tree_sitter::Node, ranges: &mut Vec<ProtectedRange>) {
    match node.kind() {
        "string_literal" | "comment" => {
            ranges.push(ProtectedRange {
                start_byte: node.start_byte(),
                end_byte: node.end_byte(),
            });
        }
        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_string_ranges(&child, ranges);
    }
}

/// Collect comment ranges using heuristic detection (fallback for edge cases)
fn collect_comment_ranges(source: &str, ranges: &mut Vec<ProtectedRange>) {
    let mut byte_offset = 0;

    for line in source.lines() {
        if let Some(comment_start) = find_comment_start_in_line(line) {
            let range_start = byte_offset + comment_start;
            let range_end = byte_offset + line.len();

            // Only add if not already covered by AST-based detection
            let already_covered = ranges
                .iter()
                .any(|r| r.start_byte <= range_start && r.end_byte >= range_end);
            if !already_covered {
                ranges.push(ProtectedRange {
                    start_byte: range_start,
                    end_byte: range_end,
                });
            }
        }
        byte_offset += line.len() + 1; // +1 for newline
    }
}

/// Find comment start position in a line (returns byte offset within line)
fn find_comment_start_in_line(line: &str) -> Option<usize> {
    let trimmed = line.trim_start();
    let leading_ws = line.len() - trimmed.len();

    // Line starts with comment character
    if trimmed.starts_with('"') || trimmed.starts_with('#') {
        return Some(leading_ws);
    }

    // Look for trailing comment (preceded by whitespace)
    // This is a heuristic - " and # can also appear in strings
    // but strings are handled separately via AST
    for (i, c) in line.char_indices() {
        if (c == '"' || c == '#') && i > 0 {
            let prev_char = line[..i].chars().last();
            if prev_char == Some(' ') || prev_char == Some('\t') {
                return Some(i);
            }
        }
    }

    None
}

/// Check if a byte offset is within any protected range
fn is_protected(byte_offset: usize, protected_ranges: &[ProtectedRange]) -> bool {
    // Binary search would be more efficient, but linear is fine for typical file sizes
    protected_ranges.iter().any(|r| r.contains(byte_offset))
}

/// Normalize spaces in a single line, avoiding protected ranges
///
/// Note: Mixed tabs and spaces (e.g., `<TAB><SPACE><TAB>`) are normalized to a single space.
/// This is intentional - if tabs should be preserved, use the indent module's tab handling.
fn normalize_line_spaces(
    line_num: usize,
    line: &str,
    line_start_byte: usize,
    protected_ranges: &[ProtectedRange],
) -> Vec<TextEdit> {
    let mut edits = Vec::new();

    // Skip leading whitespace (handled by indent module)
    let trimmed = line.trim_start();
    let indent_len = line.len() - trimmed.len();

    // Also skip trailing whitespace (handled by rules module)
    let trimmed_end = line.trim_end();
    let content_end = trimmed_end.len();

    // Find consecutive space sequences after indentation and before trailing whitespace
    let mut i = indent_len;
    let chars: Vec<char> = line.chars().collect();

    while i < chars.len() && i < content_end {
        // Look for start of multiple consecutive spaces
        if chars[i] == ' ' || chars[i] == '\t' {
            let space_start = i;
            let mut space_count = 0;

            // Count consecutive whitespace (but stop at content end)
            while i < chars.len() && i < content_end && (chars[i] == ' ' || chars[i] == '\t') {
                space_count += 1;
                i += 1;
            }

            // If multiple spaces and not protected, normalize to single space
            if space_count > 1 {
                let byte_offset = line_start_byte + char_offset_to_byte(line, space_start);

                // Check if this range is protected
                if !is_protected(byte_offset, protected_ranges) {
                    edits.push(TextEdit {
                        range: Range {
                            start: Position {
                                line: line_num as u32,
                                character: space_start as u32,
                            },
                            end: Position {
                                line: line_num as u32,
                                character: (space_start + space_count) as u32,
                            },
                        },
                        new_text: " ".to_string(),
                    });
                }
            }
        } else {
            i += 1;
        }
    }

    edits
}

/// Convert character offset to byte offset in a string
fn char_offset_to_byte(s: &str, char_offset: usize) -> usize {
    s.char_indices()
        .nth(char_offset)
        .map(|(i, _)| i)
        .unwrap_or(s.len())
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
    fn test_normalize_multiple_spaces() {
        let source = "let x   =   1\n";
        let tree = parse_vim(source);
        let edits = compute_space_edits(source, &tree);

        // Should have 2 edits: "   " -> " " twice
        assert_eq!(edits.len(), 2);
    }

    #[test]
    fn test_preserve_string_spaces() {
        let source = "let x = 'hello   world'\n";
        let tree = parse_vim(source);
        let edits = compute_space_edits(source, &tree);

        // Should have no edits - spaces are inside string
        assert!(edits.is_empty());
    }

    #[test]
    fn test_preserve_comment_spaces() {
        let source = "\" This   is   a   comment\n";
        let tree = parse_vim(source);
        let edits = compute_space_edits(source, &tree);

        // Should have no edits - entire line is a comment
        assert!(edits.is_empty());
    }

    #[test]
    fn test_function_signature_spaces() {
        let source = "function!     Hello         ( name   = 'world' )   abort\n";
        let tree = parse_vim(source);
        let edits = compute_space_edits(source, &tree);

        // Should normalize spaces around function name and parameters
        assert!(!edits.is_empty());
    }

    #[test]
    fn test_mixed_code_and_string() {
        let source = "echo       'hello   world'  ..          a:name\n";
        let tree = parse_vim(source);
        let edits = compute_space_edits(source, &tree);

        // Should normalize spaces around operators, but not inside string
        // "       " after echo -> " "
        // "  " after string -> " "
        // "          " after .. -> " "
        assert!(edits.len() >= 2);

        // Verify string content is not affected
        for edit in &edits {
            // None of the edits should be inside the string (columns ~11-25)
            assert!(
                edit.range.end.character <= 11 || edit.range.start.character >= 25,
                "Edit should not be inside string: {:?}",
                edit
            );
        }
    }

    #[test]
    fn test_indent_preserved() {
        let source = "  let x   =   1\n";
        let tree = parse_vim(source);
        let edits = compute_space_edits(source, &tree);

        // Should only normalize spaces after indentation
        for edit in &edits {
            assert!(
                edit.range.start.character >= 2,
                "Edit should not affect indentation"
            );
        }
    }

    #[test]
    fn test_trailing_comment() {
        let source = "let x   =   1 \" trailing   comment\n";
        let tree = parse_vim(source);
        let edits = compute_space_edits(source, &tree);

        // Should normalize spaces in code, but not in comment
        for edit in &edits {
            // Comment starts at column 14
            assert!(
                edit.range.end.character <= 14,
                "Edit should not be inside comment: {:?}",
                edit
            );
        }
    }

    #[test]
    #[ignore] // Debug utility - run manually with: cargo test test_debug_ast -- --ignored
    fn test_debug_ast() {
        // Debug test to see AST structure
        let source = "function! Test()\n  echo 'hello'\n  \" comment\nendfunction\n";
        let tree = parse_vim(source);

        fn print_node(node: &tree_sitter::Node, source: &str, depth: usize) {
            let indent = "  ".repeat(depth);
            let text: String = node
                .utf8_text(source.as_bytes())
                .unwrap_or("")
                .chars()
                .take(30)
                .collect();
            println!(
                "{}{} [{}..{}] {:?}",
                indent,
                node.kind(),
                node.start_byte(),
                node.end_byte(),
                text
            );
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                print_node(&child, source, depth + 1);
            }
        }

        print_node(&tree.root_node(), source, 0);
    }
}
