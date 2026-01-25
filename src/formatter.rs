//! Vim script formatter
//!
//! This module provides formatting functionality for Vim script files.
//!
//! # Features
//!
//! - Trailing whitespace removal
//! - Final newline insertion
//! - Block indentation (function/if/for/while/try/augroup)
//! - Line continuation indentation (\ at end of line)
//!
//! # Example
//!
//! ```ignore
//! use hjkls::formatter::{format, FormatConfig};
//!
//! let source = "function! Test()\nlet x = 1\nendfunction";
//! let tree = parser.parse(source, None).unwrap();
//! let config = FormatConfig::default();
//!
//! let edits = format(source, &tree, &config);
//! ```

mod indent;
mod rules;
mod spaces;

pub use crate::config::FormatConfig;

use tower_lsp_server::ls_types::TextEdit;
use tree_sitter::Tree;

/// Format Vim script source code and return text edits
///
/// This function analyzes the source code and syntax tree to produce
/// a list of text edits that will format the code according to the
/// configuration settings.
///
/// # Arguments
///
/// * `source` - The source code to format
/// * `tree` - The parsed syntax tree
/// * `config` - Format configuration
///
/// # Returns
///
/// A vector of `TextEdit` that can be applied to format the source
pub fn format(source: &str, tree: &Tree, config: &FormatConfig) -> Vec<TextEdit> {
    let mut edits = Vec::new();

    // Compute indent edits first (these modify line starts)
    edits.extend(indent::compute_indent_edits(source, tree, config));

    // Compute space normalization edits (multiple spaces → single space)
    if config.normalize_spaces {
        edits.extend(spaces::compute_space_edits(source, tree));
    }

    // Compute line-level edits (trailing whitespace, final newline)
    edits.extend(rules::compute_line_edits(source, config));

    // Sort edits by position (in reverse order for correct application)
    edits.sort_by(|a, b| {
        let pos_a = (a.range.start.line, a.range.start.character);
        let pos_b = (b.range.start.line, b.range.start.character);
        pos_b.cmp(&pos_a)
    });

    // Remove duplicate edits for the same range
    edits.dedup_by(|a, b| a.range == b.range);

    edits
}

/// Format Vim script source code and return the formatted string
///
/// This is a convenience function that applies all edits and returns
/// the resulting string.
///
/// # Arguments
///
/// * `source` - The source code to format
/// * `tree` - The parsed syntax tree
/// * `config` - Format configuration
///
/// # Returns
///
/// The formatted source code as a string
#[cfg(test)]
pub fn format_to_string(source: &str, tree: &Tree, config: &FormatConfig) -> String {
    let edits = format(source, tree, config);
    apply_edits(source, &edits)
}

/// Apply text edits to source code
///
/// Edits are expected to be sorted in reverse order (last position first)
#[cfg(test)]
fn apply_edits(source: &str, edits: &[TextEdit]) -> String {
    let mut result = source.to_string();

    for edit in edits {
        let start_offset = position_to_offset(&result, edit.range.start);
        let end_offset = position_to_offset(&result, edit.range.end);

        if let (Some(start), Some(end)) = (start_offset, end_offset) {
            result.replace_range(start..end, &edit.new_text);
        }
    }

    result
}

/// Convert LSP position to byte offset
#[cfg(test)]
fn position_to_offset(
    source: &str,
    position: tower_lsp_server::ls_types::Position,
) -> Option<usize> {
    let mut offset = 0;
    let mut current_line = 0;

    for line in source.lines() {
        if current_line == position.line as usize {
            let char_offset = position.character as usize;
            if char_offset <= line.len() {
                return Some(offset + char_offset);
            } else {
                return Some(offset + line.len());
            }
        }
        offset += line.len() + 1; // +1 for newline
        current_line += 1;
    }

    // Handle position at the end of file
    if current_line == position.line as usize && position.character == 0 {
        Some(offset)
    } else {
        None
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
    fn test_format_basic() {
        let source = "function! Test()\nlet x = 1\nendfunction";
        let tree = parse_vim(source);
        let config = FormatConfig::default();

        let result = format_to_string(source, &tree, &config);

        // Should have proper indentation and final newline
        assert!(result.contains("  let x = 1"));
        assert!(result.ends_with('\n'));
    }

    #[test]
    fn test_format_trailing_whitespace() {
        let source = "let x = 1   \nlet y = 2\n";
        let tree = parse_vim(source);
        let config = FormatConfig::default();

        let result = format_to_string(source, &tree, &config);

        // Trailing whitespace should be removed
        assert!(!result.contains("1   "));
        assert!(result.contains("let x = 1\n"), "Result: {:?}", result);
    }

    #[test]
    fn test_format_idempotent() {
        let source = "function! Test()\n  let x = 1\nendfunction\n";
        let tree = parse_vim(source);
        let config = FormatConfig::default();

        let result1 = format_to_string(source, &tree, &config);
        let tree2 = parse_vim(&result1);
        let result2 = format_to_string(&result1, &tree2, &config);

        // Formatting twice should produce the same result
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_format_preserves_comments() {
        let source = "\" This is a comment\nlet x = 1\n";
        let tree = parse_vim(source);
        let config = FormatConfig::default();

        let result = format_to_string(source, &tree, &config);

        // Comment should be preserved
        assert!(result.contains("\" This is a comment"));
    }

    #[test]
    fn test_format_nested_blocks() {
        let source = "function! Test()\nif a == 1\nlet x = 1\nendif\nendfunction\n";
        let tree = parse_vim(source);
        let config = FormatConfig::default();

        let result = format_to_string(source, &tree, &config);

        // Should have proper nested indentation
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines[0], "function! Test()");
        assert_eq!(lines[1], "  if a == 1");
        assert_eq!(lines[2], "    let x = 1");
        assert_eq!(lines[3], "  endif");
        assert_eq!(lines[4], "endfunction");
    }

    #[test]
    fn test_position_to_offset() {
        let source = "line1\nline2\nline3";

        // Position at start of line 0
        assert_eq!(
            position_to_offset(
                source,
                tower_lsp_server::ls_types::Position {
                    line: 0,
                    character: 0
                }
            ),
            Some(0)
        );

        // Position at start of line 1
        assert_eq!(
            position_to_offset(
                source,
                tower_lsp_server::ls_types::Position {
                    line: 1,
                    character: 0
                }
            ),
            Some(6)
        );

        // Position in middle of line 1
        assert_eq!(
            position_to_offset(
                source,
                tower_lsp_server::ls_types::Position {
                    line: 1,
                    character: 3
                }
            ),
            Some(9)
        );
    }

    #[test]
    fn test_format_normalize_spaces() {
        // Normalize excessive whitespace: multiple spaces → single space
        let source = "function!     Hello         ( name   = 'world' )   abort\n  echo       'Hello, '  ..          a:name\nendfunction\n";
        let tree = parse_vim(source);
        let config = FormatConfig::default();

        let result = format_to_string(source, &tree, &config);

        // Should normalize multiple spaces to single space while preserving string content
        // Note: single spaces around parentheses are kept (removing them would be a different rule)
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines[0], "function! Hello ( name = 'world' ) abort");
        assert_eq!(lines[1], "  echo 'Hello, ' .. a:name");
        assert_eq!(lines[2], "endfunction");
    }

    #[test]
    fn test_format_preserves_string_spaces() {
        // Ensure spaces inside strings are preserved
        let source = "let msg = 'hello     world'\n";
        let tree = parse_vim(source);
        let config = FormatConfig::default();

        let result = format_to_string(source, &tree, &config);

        // String content must be preserved
        assert!(result.contains("'hello     world'"));
    }

    #[test]
    fn test_format_preserves_comment_spaces() {
        // Ensure spaces inside comments are preserved
        let source = "\" This   is   a   comment\nlet x = 1\n";
        let tree = parse_vim(source);
        let config = FormatConfig::default();

        let result = format_to_string(source, &tree, &config);

        // Comment content must be preserved
        assert!(result.contains("\" This   is   a   comment"));
    }
}
