//! Diagnostics module for hjkls
//!
//! This module provides lint diagnostics organized by severity:
//! - `correctness`: Error level - likely bugs or incorrect code
//! - `suspicious`: Warning level - code that may behave unexpectedly
//! - `style`: Hint level - style suggestions for better code
//!
//! Diagnostics can be suppressed using inline comments:
//! - `hjkls:ignore <rules>` - ignore to end of file
//! - `hjkls:ignore-next-line <rules>` - ignore next line only

pub mod ignore;
pub mod style;
pub mod suspicious;

use tower_lsp_server::ls_types::Diagnostic;

use crate::config::Config;

// Re-export commonly used functions
pub use ignore::{filter_diagnostics, parse_ignore_directives};
pub use style::collect_style_hints;
pub use suspicious::collect_suspicious_warnings;

/// Map a diagnostic code to its category
///
/// Returns the category name for a given rule code (e.g., "hjkls/normal_bang" -> "suspicious")
fn get_rule_category(code: &str) -> Option<&'static str> {
    // Strip "hjkls/" prefix if present
    let rule_name = code.strip_prefix("hjkls/").unwrap_or(code);

    // Map rule names to categories
    match rule_name {
        // Correctness rules
        "autoload_missing" | "arity_mismatch" | "scope_violation" | "undefined_function" => {
            Some("correctness")
        }
        // Suspicious rules
        "normal_bang"
        | "match_case"
        | "autocmd_group"
        | "set_compatible"
        | "vim9script_position" => Some("suspicious"),
        // Style rules
        "double_dot" | "function_bang" | "abort" | "single_quote" | "key_notation"
        | "plug_noremap" => Some("style"),
        _ => None,
    }
}

/// Filter diagnostics based on configuration settings
///
/// Removes diagnostics for rules that are disabled in the config.
pub fn filter_by_config(diagnostics: Vec<Diagnostic>, config: &Config) -> Vec<Diagnostic> {
    diagnostics
        .into_iter()
        .filter(|diag| {
            // Get the diagnostic code
            let Some(code) = diag.code.as_ref() else {
                // Diagnostics without code (e.g., syntax errors) are always shown
                return true;
            };

            let code_str = match code {
                tower_lsp_server::ls_types::NumberOrString::String(s) => s.as_str(),
                tower_lsp_server::ls_types::NumberOrString::Number(_) => {
                    // Numeric codes are always shown
                    return true;
                }
            };

            // Get the rule name (strip "hjkls/" prefix)
            let rule_name = code_str.strip_prefix("hjkls/").unwrap_or(code_str);

            // Get the category for this rule
            let Some(category) = get_rule_category(code_str) else {
                // Unknown rules are always shown
                return true;
            };

            // Check if the rule is enabled
            config.is_rule_enabled(category, rule_name)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower_lsp_server::ls_types::{NumberOrString, Position, Range};

    fn make_diagnostic(code: &str) -> Diagnostic {
        Diagnostic {
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
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
    fn test_get_rule_category() {
        assert_eq!(get_rule_category("hjkls/normal_bang"), Some("suspicious"));
        assert_eq!(get_rule_category("hjkls/double_dot"), Some("style"));
        assert_eq!(
            get_rule_category("hjkls/undefined_function"),
            Some("correctness")
        );
        assert_eq!(get_rule_category("hjkls/unknown_rule"), None);
        // Without prefix
        assert_eq!(get_rule_category("normal_bang"), Some("suspicious"));
    }

    #[test]
    fn test_filter_by_config_default() {
        let config = Config::default();

        let diagnostics = vec![
            make_diagnostic("hjkls/undefined_function"), // correctness: enabled by default
            make_diagnostic("hjkls/normal_bang"),        // suspicious: enabled by default
            make_diagnostic("hjkls/double_dot"),         // style: disabled by default
        ];

        let filtered = filter_by_config(diagnostics, &config);
        assert_eq!(filtered.len(), 2);
        assert!(
            filtered
                .iter()
                .any(|d| d.code == Some(NumberOrString::String("hjkls/undefined_function".into())))
        );
        assert!(
            filtered
                .iter()
                .any(|d| d.code == Some(NumberOrString::String("hjkls/normal_bang".into())))
        );
    }

    #[test]
    fn test_filter_by_config_category_override() {
        let config = Config::parse(
            r#"
            [lint]
            style = true
            suspicious = false
            "#,
        )
        .unwrap();

        let diagnostics = vec![
            make_diagnostic("hjkls/normal_bang"), // suspicious: now disabled
            make_diagnostic("hjkls/double_dot"),  // style: now enabled
        ];

        let filtered = filter_by_config(diagnostics, &config);
        assert_eq!(filtered.len(), 1);
        assert!(
            filtered
                .iter()
                .any(|d| d.code == Some(NumberOrString::String("hjkls/double_dot".into())))
        );
    }

    #[test]
    fn test_filter_by_config_rule_override() {
        let config = Config::parse(
            r#"
            [lint]
            suspicious = true

            [lint.rules.suspicious]
            normal_bang = "off"
            "#,
        )
        .unwrap();

        let diagnostics = vec![
            make_diagnostic("hjkls/normal_bang"), // individually disabled
            make_diagnostic("hjkls/match_case"),  // still enabled (category is on)
        ];

        let filtered = filter_by_config(diagnostics, &config);
        assert_eq!(filtered.len(), 1);
        assert!(
            filtered
                .iter()
                .any(|d| d.code == Some(NumberOrString::String("hjkls/match_case".into())))
        );
    }
}
