//! Configuration file support for hjkls
//!
//! Reads `.hjkls.toml` from the workspace root to configure lint and format rules.
//!
//! # Example configuration
//!
//! ```toml
//! [lint]
//! correctness = true   # default: true
//! suspicious = true    # default: true
//! style = false        # default: false
//!
//! [lint.rules.suspicious]
//! normal_bang = "off"
//!
//! [lint.rules.style]
//! double_dot = "warn"
//!
//! [format]
//! indent_width = 2                # default: 2
//! use_tabs = false                # default: false
//! line_continuation_indent = 6    # default: indent_width * 3
//! trim_trailing_whitespace = true # default: true
//! insert_final_newline = true     # default: true
//! normalize_spaces = true         # default: true
//! space_around_operators = true   # default: true
//! space_after_comma = true        # default: true
//! ```

use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

/// The configuration file name
pub const CONFIG_FILE_NAME: &str = ".hjkls.toml";

/// Rule state: enabled or disabled
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleState {
    /// Rule is disabled
    Off,
    /// Rule is enabled (default for most rules)
    #[default]
    Warn,
}

impl RuleState {
    pub fn is_enabled(self) -> bool {
        matches!(self, RuleState::Warn)
    }
}

/// Format configuration section
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct FormatConfig {
    /// Indent width (shiftwidth), default: 2
    pub indent_width: usize,
    /// Use tabs instead of spaces, default: false
    pub use_tabs: bool,
    /// Line continuation indent (default: indent_width * 3 = 6)
    /// When None, uses indent_width * 3
    pub line_continuation_indent: Option<usize>,
    /// Trim trailing whitespace, default: true
    pub trim_trailing_whitespace: bool,
    /// Insert final newline, default: true
    pub insert_final_newline: bool,
    /// Normalize multiple consecutive spaces to single space, default: true
    pub normalize_spaces: bool,
    /// Add/normalize spaces around operators, default: true
    pub space_around_operators: bool,
    /// Add space after commas, default: true
    pub space_after_comma: bool,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            indent_width: 2,
            use_tabs: false,
            line_continuation_indent: None,
            trim_trailing_whitespace: true,
            insert_final_newline: true,
            normalize_spaces: true,
            space_around_operators: true,
            space_after_comma: true,
        }
    }
}

impl FormatConfig {
    /// Get effective line continuation indent (default: indent_width * 3)
    pub fn effective_line_continuation_indent(&self) -> usize {
        self.line_continuation_indent
            .unwrap_or(self.indent_width * 3)
    }
}

/// Lint configuration section
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct LintConfig {
    /// Enable/disable correctness category (default: true)
    pub correctness: Option<bool>,
    /// Enable/disable suspicious category (default: true)
    pub suspicious: Option<bool>,
    /// Enable/disable style category (default: false)
    pub style: Option<bool>,
    /// Per-rule overrides
    pub rules: RulesConfig,
}

/// Per-category rule overrides
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct RulesConfig {
    /// Correctness rule overrides
    pub correctness: HashMap<String, RuleState>,
    /// Suspicious rule overrides
    pub suspicious: HashMap<String, RuleState>,
    /// Style rule overrides
    pub style: HashMap<String, RuleState>,
}

/// Root configuration structure
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Lint configuration
    pub lint: LintConfig,
    /// Format configuration
    pub format: FormatConfig,
}

impl Config {
    /// Load configuration from a file path
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path).map_err(ConfigError::Io)?;
        Self::parse(&content)
    }

    /// Parse configuration from TOML string
    pub fn parse(content: &str) -> Result<Self, ConfigError> {
        toml::from_str(content).map_err(ConfigError::Parse)
    }

    /// Find and load configuration from workspace roots
    ///
    /// Searches for `.hjkls.toml` in each workspace root, returning the first found.
    pub fn find_in_workspace(roots: &[std::path::PathBuf]) -> Option<Self> {
        for root in roots {
            let config_path = root.join(CONFIG_FILE_NAME);
            if config_path.exists() {
                match Self::load(&config_path) {
                    Ok(config) => return Some(config),
                    Err(e) => {
                        // Log error but continue searching
                        // Note: eprintln is not visible to LSP clients, but log_debug
                        // requires the logger module which would create a circular dependency.
                        // Users will notice issues when their config doesn't take effect.
                        let _ = e; // Suppress unused warning
                    }
                }
            }
        }
        None
    }

    /// Check if a rule is enabled
    ///
    /// Priority: per-rule override > category setting > default
    pub fn is_rule_enabled(&self, category: &str, rule: &str) -> bool {
        // Check per-rule override first
        let rule_override = match category {
            "correctness" => self.lint.rules.correctness.get(rule),
            "suspicious" => self.lint.rules.suspicious.get(rule),
            "style" => self.lint.rules.style.get(rule),
            _ => None,
        };

        if let Some(state) = rule_override {
            return state.is_enabled();
        }

        // Check category setting
        match category {
            "correctness" => self.lint.correctness.unwrap_or(true),
            "suspicious" => self.lint.suspicious.unwrap_or(true),
            "style" => self.lint.style.unwrap_or(false),
            _ => true,
        }
    }
}

/// Configuration error types
#[derive(Debug)]
pub enum ConfigError {
    /// IO error reading the file
    Io(std::io::Error),
    /// TOML parse error
    Parse(toml::de::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "IO error: {}", e),
            ConfigError::Parse(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_config() {
        let config = Config::parse("").unwrap();
        // Defaults
        assert!(config.is_rule_enabled("correctness", "undefined_function"));
        assert!(config.is_rule_enabled("suspicious", "normal_bang"));
        assert!(!config.is_rule_enabled("style", "double_dot")); // style default: false
    }

    #[test]
    fn test_parse_category_settings() {
        let config = Config::parse(
            r#"
            [lint]
            correctness = true
            suspicious = false
            style = true
            "#,
        )
        .unwrap();

        assert!(config.is_rule_enabled("correctness", "undefined_function"));
        assert!(!config.is_rule_enabled("suspicious", "normal_bang"));
        assert!(config.is_rule_enabled("style", "double_dot"));
    }

    #[test]
    fn test_parse_rule_overrides() {
        let config = Config::parse(
            r#"
            [lint]
            suspicious = true
            style = false

            [lint.rules.suspicious]
            normal_bang = "off"

            [lint.rules.style]
            double_dot = "warn"
            "#,
        )
        .unwrap();

        // Category enabled, but rule disabled
        assert!(!config.is_rule_enabled("suspicious", "normal_bang"));
        // Other rules in category still enabled
        assert!(config.is_rule_enabled("suspicious", "match_case"));

        // Category disabled, but rule enabled
        assert!(config.is_rule_enabled("style", "double_dot"));
        // Other rules in category still disabled
        assert!(!config.is_rule_enabled("style", "function_bang"));
    }

    #[test]
    fn test_rule_state_deserialization() {
        let config = Config::parse(
            r#"
            [lint.rules.suspicious]
            normal_bang = "off"
            match_case = "warn"
            "#,
        )
        .unwrap();

        assert!(!config.is_rule_enabled("suspicious", "normal_bang"));
        assert!(config.is_rule_enabled("suspicious", "match_case"));
    }
}
