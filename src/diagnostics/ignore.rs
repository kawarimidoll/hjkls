//! Inline comment parsing for diagnostic suppression
//!
//! Supports two directive forms:
//! - `hjkls:ignore <rules>` - ignore diagnostics for the rest of the file
//! - `hjkls:ignore-next-line <rules>` - ignore diagnostics for the next line only
//!
//! Rules are specified as `category#rule_name`, separated by commas.
//! Example: `hjkls:ignore suspicious#normal_bang, style#double_dot`

use tower_lsp_server::ls_types::Diagnostic;

/// Type of ignore directive
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IgnoreKind {
    /// Ignore from this line to end of file
    ToEndOfFile,
    /// Ignore only the next line
    NextLine,
}

/// A parsed ignore directive from a comment
#[derive(Debug, Clone)]
pub struct IgnoreDirective {
    /// The line number where the directive appears (0-indexed)
    pub line: u32,
    /// The rules to ignore (e.g., ["suspicious#normal_bang", "style#double_dot"])
    /// Empty means ignore all rules
    pub rules: Vec<String>,
    /// The kind of ignore directive
    pub kind: IgnoreKind,
}

/// Parse ignore directives from source code
///
/// Looks for comments containing:
/// - `hjkls:ignore <rules>` - ignore to end of file
/// - `hjkls:ignore-next-line <rules>` - ignore next line only
///
/// Both `"` (legacy) and `#` (Vim9) comment styles are supported.
pub fn parse_ignore_directives(source: &str) -> Vec<IgnoreDirective> {
    let mut directives = Vec::new();

    for (line_num, line) in source.lines().enumerate() {
        // Find comment start (either " or #)
        let comment_start = find_comment_start(line);
        let Some(comment_pos) = comment_start else {
            continue;
        };

        let comment_text = &line[comment_pos..];

        // Check for hjkls:ignore-next-line first (longer match)
        if let Some(rules_start) = comment_text.find("hjkls:ignore-next-line") {
            let rules_text = &comment_text[rules_start + "hjkls:ignore-next-line".len()..];
            let rules = parse_rules(rules_text);
            directives.push(IgnoreDirective {
                line: line_num as u32,
                rules,
                kind: IgnoreKind::NextLine,
            });
        } else if let Some(rules_start) = comment_text.find("hjkls:ignore") {
            let rules_text = &comment_text[rules_start + "hjkls:ignore".len()..];
            let rules = parse_rules(rules_text);
            directives.push(IgnoreDirective {
                line: line_num as u32,
                rules,
                kind: IgnoreKind::ToEndOfFile,
            });
        }
    }

    directives
}

/// Find the start position of a comment in a line
///
/// Returns the byte offset of the comment character (`"` or `#`),
/// or None if no comment is found.
///
/// # Limitations
///
/// This uses a simple heuristic (whitespace-preceded `"` or `#`) and may
/// produce false positives for these characters inside string literals.
/// In practice, this is rarely an issue since `hjkls:ignore` is an unusual
/// string to appear in code. For more accurate parsing, tree-sitter's
/// comment nodes could be used.
fn find_comment_start(line: &str) -> Option<usize> {
    // Simple heuristic: find first " or # that starts a comment
    let trimmed = line.trim_start();

    // Line starts with comment character
    if trimmed.starts_with('"') || trimmed.starts_with('#') {
        return Some(line.len() - trimmed.len());
    }

    // Look for comment after code
    // For simplicity, we look for " or # preceded by whitespace
    for (i, c) in line.char_indices() {
        if c == '"' || c == '#' {
            // Check if this looks like a comment (preceded by whitespace or at line start)
            if i == 0 {
                return Some(i);
            }
            let prev_char = line[..i].chars().last();
            if prev_char.is_none_or(|c| c.is_whitespace()) {
                return Some(i);
            }
        }
    }

    None
}

/// Parse rule names from the text after the directive
///
/// Rules are comma-separated: `suspicious#normal_bang, style#double_dot`
fn parse_rules(text: &str) -> Vec<String> {
    text.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// Filter diagnostics based on ignore directives
///
/// A diagnostic is filtered out if:
/// - There's an `ignore-next-line` directive on the line before it that matches its code
/// - There's an `ignore` directive on any line before it that matches its code
pub fn filter_diagnostics(
    diagnostics: Vec<Diagnostic>,
    directives: &[IgnoreDirective],
) -> Vec<Diagnostic> {
    if directives.is_empty() {
        return diagnostics;
    }

    diagnostics
        .into_iter()
        .filter(|diag| !should_ignore(diag, directives))
        .collect()
}

/// Check if a diagnostic should be ignored based on directives
fn should_ignore(diag: &Diagnostic, directives: &[IgnoreDirective]) -> bool {
    let diag_line = diag.range.start.line;
    let diag_code = get_diagnostic_code(diag);

    for directive in directives {
        match directive.kind {
            IgnoreKind::NextLine => {
                // Directive on line N ignores diagnostics on line N+1
                if directive.line + 1 == diag_line && matches_rules(&directive.rules, &diag_code) {
                    return true;
                }
            }
            IgnoreKind::ToEndOfFile => {
                // Directive on line N ignores diagnostics on lines > N
                if directive.line < diag_line && matches_rules(&directive.rules, &diag_code) {
                    return true;
                }
            }
        }
    }

    false
}

/// Extract the diagnostic code as a string (e.g., "hjkls/normal_bang")
fn get_diagnostic_code(diag: &Diagnostic) -> Option<String> {
    diag.code.as_ref().map(|code| match code {
        tower_lsp_server::ls_types::NumberOrString::String(s) => s.clone(),
        tower_lsp_server::ls_types::NumberOrString::Number(n) => n.to_string(),
    })
}

/// Check if a diagnostic code matches any of the specified rules
///
/// If rules is empty, matches all diagnostics.
/// Rule format: `category#rule_name` (e.g., `suspicious#normal_bang`)
/// Diagnostic code format: `hjkls/rule_name` (e.g., `hjkls/normal_bang`)
fn matches_rules(rules: &[String], diag_code: &Option<String>) -> bool {
    // Empty rules means match all
    if rules.is_empty() {
        return true;
    }

    let Some(code) = diag_code else {
        return false;
    };

    // Extract rule_name from diagnostic code (hjkls/rule_name -> rule_name)
    let rule_name = code.strip_prefix("hjkls/").unwrap_or(code);

    for rule in rules {
        // Rule format: category#rule_name
        // We match if the rule_name part matches
        if let Some((_category, name)) = rule.split_once('#') {
            if name == rule_name {
                return true;
            }
        } else {
            // If no category specified, match by rule name directly
            if rule == rule_name {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower_lsp_server::ls_types::{NumberOrString, Position, Range};

    fn make_diagnostic(line: u32, code: &str) -> Diagnostic {
        Diagnostic {
            range: Range {
                start: Position { line, character: 0 },
                end: Position {
                    line,
                    character: 10,
                },
            },
            severity: None,
            code: Some(NumberOrString::String(code.to_string())),
            code_description: None,
            source: Some("hjkls".to_string()),
            message: "test".to_string(),
            related_information: None,
            tags: None,
            data: None,
        }
    }

    #[test]
    fn test_parse_ignore_directives_legacy_comment() {
        let source = r#"" hjkls:ignore suspicious#normal_bang
normal j"#;
        let directives = parse_ignore_directives(source);
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].line, 0);
        assert_eq!(directives[0].rules, vec!["suspicious#normal_bang"]);
        assert_eq!(directives[0].kind, IgnoreKind::ToEndOfFile);
    }

    #[test]
    fn test_parse_ignore_directives_vim9_comment() {
        let source = r#"vim9script
# hjkls:ignore-next-line suspicious#normal_bang
normal j"#;
        let directives = parse_ignore_directives(source);
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].line, 1);
        assert_eq!(directives[0].rules, vec!["suspicious#normal_bang"]);
        assert_eq!(directives[0].kind, IgnoreKind::NextLine);
    }

    #[test]
    fn test_parse_ignore_directives_multiple_rules() {
        let source = r#"" hjkls:ignore suspicious#normal_bang, style#double_dot"#;
        let directives = parse_ignore_directives(source);
        assert_eq!(directives.len(), 1);
        assert_eq!(
            directives[0].rules,
            vec!["suspicious#normal_bang", "style#double_dot"]
        );
    }

    #[test]
    fn test_parse_ignore_directives_no_rules() {
        let source = r#"" hjkls:ignore"#;
        let directives = parse_ignore_directives(source);
        assert_eq!(directives.len(), 1);
        assert!(directives[0].rules.is_empty());
    }

    #[test]
    fn test_filter_diagnostics_ignore_next_line() {
        let directives = vec![IgnoreDirective {
            line: 0,
            rules: vec!["suspicious#normal_bang".to_string()],
            kind: IgnoreKind::NextLine,
        }];

        let diagnostics = vec![
            make_diagnostic(1, "hjkls/normal_bang"), // Should be filtered
            make_diagnostic(2, "hjkls/normal_bang"), // Should NOT be filtered
            make_diagnostic(1, "hjkls/double_dot"),  // Should NOT be filtered (different rule)
        ];

        let filtered = filter_diagnostics(diagnostics, &directives);
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].range.start.line, 2);
        assert_eq!(filtered[1].range.start.line, 1);
    }

    #[test]
    fn test_filter_diagnostics_ignore_to_end() {
        let directives = vec![IgnoreDirective {
            line: 5,
            rules: vec!["suspicious#normal_bang".to_string()],
            kind: IgnoreKind::ToEndOfFile,
        }];

        let diagnostics = vec![
            make_diagnostic(3, "hjkls/normal_bang"), // Should NOT be filtered (before directive)
            make_diagnostic(10, "hjkls/normal_bang"), // Should be filtered
            make_diagnostic(10, "hjkls/double_dot"), // Should NOT be filtered (different rule)
        ];

        let filtered = filter_diagnostics(diagnostics, &directives);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_filter_diagnostics_ignore_all() {
        let directives = vec![IgnoreDirective {
            line: 0,
            rules: vec![], // Empty means all rules
            kind: IgnoreKind::ToEndOfFile,
        }];

        let diagnostics = vec![
            make_diagnostic(5, "hjkls/normal_bang"),
            make_diagnostic(5, "hjkls/double_dot"),
        ];

        let filtered = filter_diagnostics(diagnostics, &directives);
        assert!(filtered.is_empty());
    }

    #[test]
    fn test_find_comment_start() {
        assert_eq!(find_comment_start("\" comment"), Some(0));
        assert_eq!(find_comment_start("# vim9 comment"), Some(0));
        assert_eq!(find_comment_start("  \" indented"), Some(2));
        assert_eq!(find_comment_start("code \" comment"), Some(5));
        assert_eq!(find_comment_start("no comment here"), None);
    }
}
