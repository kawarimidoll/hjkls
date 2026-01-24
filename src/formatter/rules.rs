//! Line-level formatting rules for Vim script
//!
//! This module handles:
//! - Trailing whitespace removal
//! - Final newline insertion

use tower_lsp_server::ls_types::{Position, Range, TextEdit};

/// Compute text edits for line-level rules (trailing whitespace, final newline)
pub fn compute_line_edits(source: &str, config: &super::FormatConfig) -> Vec<TextEdit> {
    let mut edits = Vec::new();

    // Process each line for trailing whitespace
    if config.trim_trailing_whitespace {
        for (line_num, line) in source.lines().enumerate() {
            if let Some(edit) = trim_trailing_whitespace_edit(line_num, line) {
                edits.push(edit);
            }
        }
    }

    // Handle final newline
    if config.insert_final_newline && !source.ends_with('\n') {
        let line_count = source.lines().count();
        let last_line_len = source.lines().last().map(|l| l.len()).unwrap_or(0);

        // If the source is empty, insert at the beginning
        let (line, character) = if source.is_empty() {
            (0, 0)
        } else {
            (line_count.saturating_sub(1) as u32, last_line_len as u32)
        };

        edits.push(TextEdit {
            range: Range {
                start: Position { line, character },
                end: Position { line, character },
            },
            new_text: "\n".to_string(),
        });
    }

    edits
}

/// Create a TextEdit to remove trailing whitespace from a line, if any
fn trim_trailing_whitespace_edit(line_num: usize, line: &str) -> Option<TextEdit> {
    let trimmed = line.trim_end();
    if trimmed.len() < line.len() {
        let start_col = trimmed.len();
        let end_col = line.len();
        Some(TextEdit {
            range: Range {
                start: Position {
                    line: line_num as u32,
                    character: start_col as u32,
                },
                end: Position {
                    line: line_num as u32,
                    character: end_col as u32,
                },
            },
            new_text: String::new(),
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::FormatConfig;

    #[test]
    fn test_trim_trailing_whitespace() {
        let config = FormatConfig::default();
        let source = "let x = 1   \nlet y = 2\n";
        let edits = compute_line_edits(source, &config);

        // Should have one edit for trailing spaces on line 0
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].range.start.line, 0);
        assert_eq!(edits[0].range.start.character, 9); // "let x = 1"
        assert_eq!(edits[0].range.end.character, 12); // "let x = 1   "
        assert_eq!(edits[0].new_text, "");
    }

    #[test]
    fn test_insert_final_newline() {
        let config = FormatConfig::default();
        let source = "let x = 1";
        let edits = compute_line_edits(source, &config);

        // Should have one edit to add final newline
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].range.start.line, 0);
        assert_eq!(edits[0].range.start.character, 9);
        assert_eq!(edits[0].new_text, "\n");
    }

    #[test]
    fn test_no_final_newline_when_present() {
        let config = FormatConfig::default();
        let source = "let x = 1\n";
        let edits = compute_line_edits(source, &config);

        // Should have no edits
        assert!(edits.is_empty());
    }

    #[test]
    fn test_disabled_rules() {
        let config = FormatConfig {
            trim_trailing_whitespace: false,
            insert_final_newline: false,
            ..Default::default()
        };
        let source = "let x = 1   ";
        let edits = compute_line_edits(source, &config);

        // Should have no edits
        assert!(edits.is_empty());
    }

    #[test]
    fn test_multiple_trailing_spaces() {
        let config = FormatConfig::default();
        let source = "line1  \nline2    \nline3\n";
        let edits = compute_line_edits(source, &config);

        // Should have edits for lines 0 and 1
        assert_eq!(edits.len(), 2);
        assert_eq!(edits[0].range.start.line, 0);
        assert_eq!(edits[1].range.start.line, 1);
    }

    #[test]
    fn test_newline_only_file() {
        let config = FormatConfig::default();

        // Single newline - already has final newline
        let source = "\n";
        let edits = compute_line_edits(source, &config);
        assert!(edits.is_empty());

        // Multiple newlines - already has final newline
        let source = "\n\n\n";
        let edits = compute_line_edits(source, &config);
        assert!(edits.is_empty());
    }

    #[test]
    fn test_empty_file() {
        let config = FormatConfig::default();
        let source = "";
        let edits = compute_line_edits(source, &config);

        // Should add final newline
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].new_text, "\n");
    }

    #[test]
    fn test_trailing_whitespace_with_multibyte() {
        let config = FormatConfig::default();
        // Japanese text with trailing spaces
        let source = "let x = '日本語'   \n";
        let edits = compute_line_edits(source, &config);

        // Should have one edit for trailing spaces
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].new_text, "");
    }
}
