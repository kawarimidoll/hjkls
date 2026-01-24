//! Diagnostics module for hjkls
//!
//! This module provides lint diagnostics organized by severity:
//! - `correctness`: Error level - likely bugs or incorrect code
//! - `suspicious`: Warning level - code that may behave unexpectedly
//! - `style`: Hint level - style suggestions for better code

pub mod style;
pub mod suspicious;

// Re-export commonly used functions
pub use style::collect_style_hints;
pub use suspicious::collect_suspicious_warnings;
