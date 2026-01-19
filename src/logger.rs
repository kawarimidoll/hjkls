//! Simple file-based logger for debugging
//!
//! Usage: hjkls --log=/path/to/hjkls.log

use std::fs::OpenOptions;
use std::io::Write;
use std::sync::OnceLock;

static LOG_PATH: OnceLock<Option<String>> = OnceLock::new();

/// Initialize the logger with the given path
pub fn init(path: Option<String>) {
    LOG_PATH.get_or_init(|| path);
}

/// Log a message to the file if logging is enabled
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::logger::log(&format!($($arg)*))
    };
}

/// Write a log message to the file
pub fn log(message: &str) {
    let Some(Some(path)) = LOG_PATH.get() else {
        return;
    };

    let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) else {
        return;
    };

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let _ = writeln!(file, "[{timestamp}] {message}");
}
