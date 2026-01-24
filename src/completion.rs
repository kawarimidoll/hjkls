//! Completion context detection for Vim script
//!
//! This module provides context-aware completion by analyzing cursor position
//! to determine what kind of completion candidates should be offered.

/// Completion context based on cursor position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionContext {
    /// Line start or command position -> Ex commands
    Command,
    /// After autocmd -> event names
    AutocmdEvent,
    /// After set/setlocal -> option names
    Option,
    /// After map command, typing <... -> map options
    MapOption,
    /// Inside has('...') -> feature names
    HasFeature,
    /// Expression/function call context -> functions and variables
    Function,
}

/// Find the start position of a completion token, including scope prefix.
/// For Vim script, this includes scope prefixes like s:, g:, l:, a:, b:, w:, t:, v:
/// e.g., for "call s:Priv|" (| is cursor), returns the position of 's'
pub fn find_completion_token_start(line: &str, cursor_col: usize) -> usize {
    let bytes: Vec<char> = line.chars().collect();
    let col = cursor_col.min(bytes.len());

    if col == 0 {
        return 0;
    }

    // First, find the start of the identifier part (alphanumeric, underscore, #)
    let mut start = col;
    while start > 0 {
        let ch = bytes[start - 1];
        if ch.is_alphanumeric() || ch == '_' || ch == '#' {
            start -= 1;
        } else {
            break;
        }
    }

    // Then, check if there's a scope prefix (s:, g:, etc.) before the identifier
    if start >= 2 && bytes[start - 1] == ':' {
        let scope_char = bytes[start - 2];
        if matches!(scope_char, 's' | 'g' | 'l' | 'a' | 'b' | 'w' | 't' | 'v') {
            // Verify this is actually a scope prefix (not part of another word)
            if start < 3 || !bytes[start - 3].is_alphanumeric() {
                start -= 2; // Include the scope prefix
            }
        }
    }

    start
}

/// Determine what kind of completion is appropriate based on cursor context
pub fn get_completion_context(line: &str, col: usize) -> CompletionContext {
    let before_cursor = &line[..col.min(line.len())];
    let trimmed = before_cursor.trim_start();

    // Empty line or only whitespace -> command context
    if trimmed.is_empty() {
        return CompletionContext::Command;
    }

    // Check for specific command patterns
    // autocmd [group] EVENT -> autocmd event completion
    if let Some(rest) = trimmed
        .strip_prefix("autocmd")
        .or_else(|| trimmed.strip_prefix("au "))
    {
        let rest = rest.trim_start();
        // Skip optional group name (if it doesn't look like an event)
        // Events are typically CamelCase, groups can be anything
        let parts: Vec<&str> = rest.split_whitespace().collect();
        // If we have 0 or 1 parts, we're likely typing the event (or group then event)
        if parts.len() <= 1 {
            return CompletionContext::AutocmdEvent;
        }
        // If 2+ parts, check if cursor is still in the "event" area
        // This is a simplification - we assume event comes after optional group
    }

    // set/setlocal/setglobal OPTION -> option completion
    if trimmed.starts_with("set ")
        || trimmed.starts_with("setlocal ")
        || trimmed.starts_with("setglobal ")
        || trimmed.starts_with("se ")
        || trimmed.starts_with("setl ")
        || trimmed.starts_with("setg ")
    {
        return CompletionContext::Option;
    }

    // map commands with < suggesting map option
    let map_commands = [
        "map", "nmap", "vmap", "xmap", "smap", "imap", "cmap", "omap", "lmap", "tmap", "noremap",
        "nnoremap", "vnoremap", "xnoremap", "snoremap", "inoremap", "cnoremap", "onoremap",
        "lnoremap", "tnoremap",
    ];
    for cmd in &map_commands {
        if let Some(rest) = trimmed.strip_prefix(cmd) {
            if rest.starts_with(' ') || rest.is_empty() {
                let rest = rest.trim_start();
                // If typing <... it's a map option
                if rest.ends_with('<')
                    || rest
                        .split_whitespace()
                        .last()
                        .is_some_and(|s| s.starts_with('<'))
                {
                    return CompletionContext::MapOption;
                }
            }
        }
    }

    // has('... -> feature completion
    if before_cursor.contains("has('") || before_cursor.contains("has(\"") {
        // Check if we're inside the has() call
        let last_has = before_cursor.rfind("has(");
        if let Some(pos) = last_has {
            let after_has = &before_cursor[pos..];
            // If there's an opening quote but no closing quote, we're inside
            if (after_has.contains('\'') && after_has.matches('\'').count() == 1)
                || (after_has.contains('"') && after_has.matches('"').count() == 1)
            {
                return CompletionContext::HasFeature;
            }
        }
    }

    // Check if line starts with a command (no = or function call pattern)
    // This is a heuristic: if the line doesn't have = and doesn't look like an expression
    let first_word = trimmed.split_whitespace().next().unwrap_or("");
    if !trimmed.contains('=') && !trimmed.contains('(') && !first_word.is_empty() {
        // If typing the first word, it's likely a command
        let first_word_end = trimmed.find(char::is_whitespace).unwrap_or(trimmed.len());
        if col <= before_cursor.len() - trimmed.len() + first_word_end {
            return CompletionContext::Command;
        }
    }

    // Default: function/expression context
    CompletionContext::Function
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_line_returns_command() {
        assert_eq!(get_completion_context("", 0), CompletionContext::Command);
        assert_eq!(
            get_completion_context("    ", 4),
            CompletionContext::Command
        );
    }

    #[test]
    fn test_autocmd_event_context() {
        // "autocmd " followed by typing event name
        assert_eq!(
            get_completion_context("autocmd Buf", 11),
            CompletionContext::AutocmdEvent
        );
        assert_eq!(
            get_completion_context("autocmd ", 8),
            CompletionContext::AutocmdEvent
        );
        // "au " shorthand
        assert_eq!(
            get_completion_context("au FileType", 11),
            CompletionContext::AutocmdEvent
        );
    }

    #[test]
    fn test_set_option_context() {
        // "set " followed by option name
        assert_eq!(
            get_completion_context("set nu", 6),
            CompletionContext::Option
        );
        assert_eq!(
            get_completion_context("setlocal expandtab", 18),
            CompletionContext::Option
        );
        assert_eq!(
            get_completion_context("setg ", 5),
            CompletionContext::Option
        );
    }

    #[test]
    fn test_map_option_context() {
        // Map commands with <...> options
        assert_eq!(
            get_completion_context("nnoremap <silent", 16),
            CompletionContext::MapOption
        );
        assert_eq!(
            get_completion_context("nmap <buf", 9),
            CompletionContext::MapOption
        );
        assert_eq!(
            get_completion_context("inoremap <", 10),
            CompletionContext::MapOption
        );
    }

    #[test]
    fn test_has_feature_context() {
        // Inside has('...') call
        assert_eq!(
            get_completion_context("if has('nvi", 11),
            CompletionContext::HasFeature
        );
        assert_eq!(
            get_completion_context("if has(\"py", 10),
            CompletionContext::HasFeature
        );
        assert_eq!(
            get_completion_context("  has('", 7),
            CompletionContext::HasFeature
        );
    }

    #[test]
    fn test_command_context() {
        // Line start with command
        assert_eq!(get_completion_context("ech", 3), CompletionContext::Command);
        assert_eq!(get_completion_context("let", 3), CompletionContext::Command);
    }

    #[test]
    fn test_function_context() {
        // Expression context
        assert_eq!(
            get_completion_context("let x = str", 11),
            CompletionContext::Function
        );
        assert_eq!(
            get_completion_context("call MyFunc(arg", 15),
            CompletionContext::Function
        );
        assert_eq!(
            get_completion_context("return strlen(s", 15),
            CompletionContext::Function
        );
    }

    #[test]
    fn test_operator_not_confused_with_command() {
        // Operators should not trigger Command context
        // `<` as comparison operator, not Ex command
        assert_eq!(
            get_completion_context("if a < b", 6),
            CompletionContext::Function
        );
        // `<` after `=` assignment
        assert_eq!(
            get_completion_context("let x = <", 9),
            CompletionContext::Function
        );
        // `>` as comparison operator
        assert_eq!(
            get_completion_context("if a > b", 6),
            CompletionContext::Function
        );
        // `<` at line start IS a valid Ex command (shift left)
        assert_eq!(get_completion_context("<", 1), CompletionContext::Command);
        assert_eq!(get_completion_context(">", 1), CompletionContext::Command);
    }
}
