//! Style hints for Vim script (DiagnosticSeverity::HINT)
//!
//! These rules suggest improvements for code style. They don't indicate bugs
//! but help maintain consistency and readability.

use tower_lsp_server::ls_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range};
use tree_sitter::Tree;

/// Collect all style hints from the syntax tree
pub fn collect_style_hints(tree: &Tree, source: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let root = tree.root_node();

    // double_dot: prefer `..` over `.` for string concatenation
    collect_double_dot_hints_recursive(&root, source, &mut diagnostics);

    // function_bang: s: functions don't need `!`
    collect_function_bang_hints_recursive(&root, source, &mut diagnostics);

    // abort: functions should have `abort` attribute
    collect_abort_hints_recursive(&root, source, &mut diagnostics);

    // single_quote: prefer single quotes when no escapes needed
    collect_single_quote_hints_recursive(&root, source, &mut diagnostics);

    // key_notation: normalize key notation to standard form
    collect_key_notation_hints_recursive(&root, source, &mut diagnostics);

    diagnostics
}

/// Collect hints for `.` string concatenation (should use `..`)
fn collect_double_dot_hints_recursive(
    node: &tree_sitter::Node,
    source: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if node.kind() == "binary_operation" {
        let mut cursor = node.walk();
        let children: Vec<_> = node.children(&mut cursor).collect();

        // Check if this is a `.` concatenation (not `..`)
        // In tree-sitter-vim, the operator is a child node with kind "." or ".."
        let has_single_dot = children.iter().any(|c| c.kind() == ".");

        if has_single_dot {
            let start = node.start_position();
            let end = node.end_position();
            let text = node.utf8_text(source.as_bytes()).unwrap_or(".");

            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position {
                        line: start.row as u32,
                        character: start.column as u32,
                    },
                    end: Position {
                        line: end.row as u32,
                        character: end.column as u32,
                    },
                },
                severity: Some(DiagnosticSeverity::HINT),
                source: Some("hjkls".to_string()),
                message: format!(
                    "Style: '{}' uses `.` for string concatenation. Use `..` instead. In Vim9 script, `..` is required.",
                    text.trim()
                ),
                code: Some(NumberOrString::String("hjkls/double_dot".to_string())),
                ..Default::default()
            });
        }
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_double_dot_hints_recursive(&child, source, diagnostics);
    }
}

/// Collect hints for `function!` with `s:` scope (bang is unnecessary)
fn collect_function_bang_hints_recursive(
    node: &tree_sitter::Node,
    source: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if node.kind() == "function_definition" {
        let mut cursor = node.walk();
        let children: Vec<_> = node.children(&mut cursor).collect();

        // Check if function has bang (!)
        let has_bang = children.iter().any(|c| c.kind() == "bang");

        if has_bang {
            // Find function_declaration to get the function name
            if let Some(decl) = children.iter().find(|c| c.kind() == "function_declaration") {
                let mut decl_cursor = decl.walk();
                let decl_children: Vec<_> = decl.children(&mut decl_cursor).collect();

                // Check if function name has s: scope
                let is_script_local = decl_children.iter().any(|c| {
                    if c.kind() == "scoped_identifier" {
                        let mut scope_cursor = c.walk();
                        c.children(&mut scope_cursor).any(|sc| {
                            sc.kind() == "scope"
                                && sc.utf8_text(source.as_bytes()).ok() == Some("s:")
                        })
                    } else {
                        false
                    }
                });

                if is_script_local {
                    let start = node.start_position();
                    // Get just the first line for cleaner message
                    let text = node.utf8_text(source.as_bytes()).unwrap_or("function!");
                    let first_line = text.lines().next().unwrap_or(text);

                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: start.row as u32,
                                character: start.column as u32,
                            },
                            end: Position {
                                line: start.row as u32,
                                character: start.column as u32 + first_line.len() as u32,
                            },
                        },
                        severity: Some(DiagnosticSeverity::HINT),
                        source: Some("hjkls".to_string()),
                        message: format!(
                            "Style: '{}' uses `function!` for script-local function. The `!` is unnecessary for `s:` functions.",
                            first_line.trim()
                        ),
                        code: Some(NumberOrString::String("hjkls/function_bang".to_string())),
                        ..Default::default()
                    });
                }
            }
        }
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_function_bang_hints_recursive(&child, source, diagnostics);
    }
}

/// Collect hints for functions without `abort` attribute
fn collect_abort_hints_recursive(
    node: &tree_sitter::Node,
    source: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if node.kind() == "function_definition" {
        let mut cursor = node.walk();
        let children: Vec<_> = node.children(&mut cursor).collect();

        // Check if function has `abort` attribute
        let has_abort = children.iter().any(|c| c.kind() == "abort");

        if !has_abort {
            let start = node.start_position();
            // Get just the first line for cleaner message
            let text = node.utf8_text(source.as_bytes()).unwrap_or("function");
            let first_line = text.lines().next().unwrap_or(text);

            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position {
                        line: start.row as u32,
                        character: start.column as u32,
                    },
                    end: Position {
                        line: start.row as u32,
                        character: start.column as u32 + first_line.len() as u32,
                    },
                },
                severity: Some(DiagnosticSeverity::HINT),
                source: Some("hjkls".to_string()),
                message: format!(
                    "Style: '{}' is missing `abort` attribute. Functions without `abort` continue execution after errors.",
                    first_line.trim()
                ),
                code: Some(NumberOrString::String("hjkls/abort".to_string())),
                ..Default::default()
            });
        }
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_abort_hints_recursive(&child, source, diagnostics);
    }
}

/// Collect hints for double-quoted strings that don't need escapes
fn collect_single_quote_hints_recursive(
    node: &tree_sitter::Node,
    source: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if node.kind() == "string_literal" {
        if let Ok(text) = node.utf8_text(source.as_bytes()) {
            // Only check double-quoted strings
            if text.starts_with('"') && text.ends_with('"') {
                // Check if the string contains any escape sequences
                // Vim escape sequences: \n, \r, \t, \e, \b, \\, \", \<xxx>, \x.., \u...., \U........, etc.
                let content = &text[1..text.len() - 1];
                let has_escape = content.contains('\\');
                // Also check for single quotes inside (would need escaping in single-quoted string)
                let has_single_quote = content.contains('\'');

                if !has_escape && !has_single_quote {
                    let start = node.start_position();
                    let end = node.end_position();

                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: start.row as u32,
                                character: start.column as u32,
                            },
                            end: Position {
                                line: end.row as u32,
                                character: end.column as u32,
                            },
                        },
                        severity: Some(DiagnosticSeverity::HINT),
                        source: Some("hjkls".to_string()),
                        message: format!(
                            "Style: {} can use single quotes. Double quotes are only needed for escape sequences.",
                            text
                        ),
                        code: Some(NumberOrString::String("hjkls/single_quote".to_string())),
                        ..Default::default()
                    });
                }
            }
        }
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_single_quote_hints_recursive(&child, source, diagnostics);
    }
}

/// Normalize key notation to standard Vim help format.
/// Returns None if already in correct format.
pub fn normalize_key_notation(key: &str) -> Option<String> {
    // Remove angle brackets for processing
    if !key.starts_with('<') || !key.ends_with('>') {
        return None;
    }
    let inner = &key[1..key.len() - 1];
    if inner.is_empty() {
        return None;
    }

    // Split modifiers and key name
    // Format: <C-S-...key> or just <key>
    // Modifiers: C (Ctrl), S (Shift), M (Meta), A (Alt), D (Cmd), T (Meta alt)
    let parts: Vec<&str> = inner.split('-').collect();
    let (modifiers, key_name) = if parts.len() > 1 {
        // Check how many leading parts are modifiers
        // The last part is always the key, so only check parts[0..len-1]
        let mut mod_end = 0;
        for (i, part) in parts[..parts.len() - 1].iter().enumerate() {
            // Modifier must be single character and one of C, S, M, A, D, T (case-insensitive)
            if part.len() == 1 {
                let ch = part.chars().next().unwrap().to_ascii_uppercase();
                if "CSMADT".contains(ch) {
                    mod_end = i + 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        if mod_end > 0 {
            (&parts[..mod_end], parts[mod_end..].join("-"))
        } else {
            (&[][..], inner.to_string())
        }
    } else {
        (&[][..], inner.to_string())
    };

    // Normalize modifiers (uppercase)
    let normalized_mods: Vec<String> = modifiers.iter().map(|m| m.to_uppercase()).collect();

    // Normalize key name based on standard Vim notation
    let normalized_key = match key_name.to_lowercase().as_str() {
        // Special keys (all uppercase)
        "cr" | "return" | "enter" => "CR",
        "nl" | "newline" | "linefeed" | "lf" => "NL",
        "tab" => "Tab",
        "esc" | "escape" => "Esc",
        "space" | "sp" => "Space",
        "bs" | "backspace" => "BS",
        "del" | "delete" => "Del",
        "insert" | "ins" => "Insert",
        "home" => "Home",
        "end" => "End",
        "pageup" | "pu" => "PageUp",
        "pagedown" | "pd" => "PageDown",
        "nul" | "null" => "Nul",
        "bar" => "Bar",
        "bslash" => "Bslash",
        "lt" => "lt",

        // Arrow keys (title case)
        "up" => "Up",
        "down" => "Down",
        "left" => "Left",
        "right" => "Right",

        // Function keys (F + number)
        k if k.starts_with('f') && k[1..].parse::<u32>().is_ok() => {
            let result = format!(
                "<{}{}>",
                if normalized_mods.is_empty() {
                    String::new()
                } else {
                    format!("{}-", normalized_mods.join("-"))
                },
                format!("F{}", &key_name[1..])
            );
            return if result == key { None } else { Some(result) };
        }

        // Keypad keys
        k if k.starts_with('k') && k.len() > 1 => {
            let keypad_key = &k[1..];
            let normalized = match keypad_key {
                "plus" | "add" => "kPlus",
                "minus" | "subtract" => "kMinus",
                "multiply" => "kMultiply",
                "divide" => "kDivide",
                "enter" => "kEnter",
                "point" | "decimal" => "kPoint",
                "home" => "kHome",
                "end" => "kEnd",
                "pageup" => "kPageUp",
                "pagedown" => "kPageDown",
                "insert" => "kInsert",
                "del" | "delete" => "kDel",
                d if d.len() == 1 && d.chars().next().unwrap().is_ascii_digit() => {
                    let result = format!(
                        "<{}{}>",
                        if normalized_mods.is_empty() {
                            String::new()
                        } else {
                            format!("{}-", normalized_mods.join("-"))
                        },
                        format!("k{}", d)
                    );
                    return if result == key { None } else { Some(result) };
                }
                _ => {
                    return None;
                }
            };
            let result = format!(
                "<{}{}>",
                if normalized_mods.is_empty() {
                    String::new()
                } else {
                    format!("{}-", normalized_mods.join("-"))
                },
                normalized
            );
            return if result == key { None } else { Some(result) };
        }

        // Special identifiers (title case)
        "leader" => "Leader",
        "localleader" => "LocalLeader",
        "plug" => "Plug",
        "sid" => "SID",
        "snr" => "SNR",
        "cmd" => "Cmd",
        "scrollwheelup" => "ScrollWheelUp",
        "scrollwheeldown" => "ScrollWheelDown",
        "scrollwheelleft" => "ScrollWheelLeft",
        "scrollwheelright" => "ScrollWheelRight",

        // Mouse events (title case words)
        "leftmouse" => "LeftMouse",
        "rightmouse" => "RightMouse",
        "middlemouse" => "MiddleMouse",
        "leftdrag" => "LeftDrag",
        "rightdrag" => "RightDrag",
        "leftrelease" => "LeftRelease",
        "rightrelease" => "RightRelease",
        "middlerelease" => "MiddleRelease",
        "x1mouse" => "X1Mouse",
        "x2mouse" => "X2Mouse",
        "x1drag" => "X1Drag",
        "x2drag" => "X2Drag",
        "x1release" => "X1Release",
        "x2release" => "X2Release",

        // Other special keys
        "help" => "Help",
        "undo" => "Undo",
        "ignore" => "Ignore",
        "drop" => "Drop",
        "focusgained" => "FocusGained",
        "focuslost" => "FocusLost",
        "cursorhold" => "CursorHold",

        // Unknown key - keep the key name as-is but normalize modifiers
        _ => {
            if normalized_mods.is_empty() {
                // No modifiers and unknown key - nothing to normalize
                return None;
            }
            // Has modifiers - construct result with normalized modifiers
            let result = format!("<{}-{}>", normalized_mods.join("-"), key_name);
            return if result == key { None } else { Some(result) };
        }
    };

    let result = format!(
        "<{}{}>",
        if normalized_mods.is_empty() {
            String::new()
        } else {
            format!("{}-", normalized_mods.join("-"))
        },
        normalized_key
    );

    // Only return if it's different from the original
    if result == key { None } else { Some(result) }
}

/// Collect hints for non-standard key notation
fn collect_key_notation_hints_recursive(
    node: &tree_sitter::Node,
    source: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if node.kind() == "keycode" {
        if let Ok(text) = node.utf8_text(source.as_bytes()) {
            if let Some(normalized) = normalize_key_notation(text) {
                let start = node.start_position();
                let end = node.end_position();

                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position {
                            line: start.row as u32,
                            character: start.column as u32,
                        },
                        end: Position {
                            line: end.row as u32,
                            character: end.column as u32,
                        },
                    },
                    severity: Some(DiagnosticSeverity::HINT),
                    source: Some("hjkls".to_string()),
                    message: format!(
                        "Style: {} should be written as {} (see :h key-notation)",
                        text, normalized
                    ),
                    code: Some(NumberOrString::String("hjkls/key_notation".to_string())),
                    ..Default::default()
                });
            }
        }
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_key_notation_hints_recursive(&child, source, diagnostics);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_notation_normalization() {
        // Already correct - should return None
        assert_eq!(normalize_key_notation("<CR>"), None);
        assert_eq!(normalize_key_notation("<Esc>"), None);
        assert_eq!(normalize_key_notation("<Up>"), None);
        assert_eq!(normalize_key_notation("<F1>"), None);
        assert_eq!(normalize_key_notation("<C-a>"), None);
        assert_eq!(normalize_key_notation("<Leader>"), None);

        // Needs normalization - should return Some
        assert_eq!(normalize_key_notation("<cr>"), Some("<CR>".to_string()));
        assert_eq!(normalize_key_notation("<esc>"), Some("<Esc>".to_string()));
        assert_eq!(normalize_key_notation("<ESC>"), Some("<Esc>".to_string()));
        assert_eq!(normalize_key_notation("<up>"), Some("<Up>".to_string()));
        assert_eq!(normalize_key_notation("<UP>"), Some("<Up>".to_string()));
        assert_eq!(normalize_key_notation("<f1>"), Some("<F1>".to_string()));
        assert_eq!(normalize_key_notation("<tab>"), Some("<Tab>".to_string()));
        assert_eq!(normalize_key_notation("<TAB>"), Some("<Tab>".to_string()));
        assert_eq!(
            normalize_key_notation("<space>"),
            Some("<Space>".to_string())
        );
        assert_eq!(normalize_key_notation("<bs>"), Some("<BS>".to_string()));

        // Modifiers
        assert_eq!(normalize_key_notation("<c-a>"), Some("<C-a>".to_string()));
        assert_eq!(normalize_key_notation("<C-A>"), None); // Already correct
        assert_eq!(normalize_key_notation("<C-a>"), None); // Already correct
        assert_eq!(
            normalize_key_notation("<s-tab>"),
            Some("<S-Tab>".to_string())
        );
        assert_eq!(
            normalize_key_notation("<c-s-f1>"),
            Some("<C-S-F1>".to_string())
        );

        // Special identifiers
        assert_eq!(
            normalize_key_notation("<leader>"),
            Some("<Leader>".to_string())
        );
        assert_eq!(normalize_key_notation("<plug>"), Some("<Plug>".to_string()));
        assert_eq!(normalize_key_notation("<sid>"), Some("<SID>".to_string()));

        // Unknown keys should return None (no suggestion)
        assert_eq!(normalize_key_notation("<unknown>"), None);
        assert_eq!(normalize_key_notation("<x>"), None);
    }
}
