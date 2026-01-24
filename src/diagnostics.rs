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

// Re-export commonly used functions
pub use ignore::{filter_diagnostics, parse_ignore_directives};
pub use style::collect_style_hints;
pub use suspicious::collect_suspicious_warnings;
