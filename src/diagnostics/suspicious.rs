//! Suspicious warnings for Vim script (DiagnosticSeverity::WARNING)
//!
//! These rules identify patterns that may behave unexpectedly or cause issues.
//! While not necessarily bugs, they often indicate potential problems.

use tower_lsp_server::ls_types::{Diagnostic, DiagnosticSeverity, Position, Range};
use tree_sitter::Tree;

/// Collect all suspicious warnings from the syntax tree
pub fn collect_suspicious_warnings(tree: &Tree, source: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let root = tree.root_node();

    // normal_bang: prefer `normal!` over `normal`
    collect_normal_bang_warnings_recursive(&root, source, &mut diagnostics);

    // match_case: prefer `=~#` or `=~?` over `=~`
    collect_match_case_warnings_recursive(&root, source, &mut diagnostics);

    // autocmd_group: autocmd outside augroup is risky
    let _ = collect_autocmd_group_warnings_recursive(&root, source, false, &mut diagnostics);

    // set_compatible: `set compatible` enables Vi-compatible mode (rarely intended)
    collect_set_compatible_warnings_recursive(&root, source, &mut diagnostics);

    // vim9script_position: `vim9script` must be at the start of the file
    collect_vim9script_position_warnings(&root, source, &mut diagnostics);

    diagnostics
}

/// Collect warnings for `normal` without `!` (should use `normal!`)
fn collect_normal_bang_warnings_recursive(
    node: &tree_sitter::Node,
    source: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if node.kind() == "normal_statement" {
        // Check if there's a bang child node
        let mut cursor = node.walk();
        let has_bang = node.children(&mut cursor).any(|c| c.kind() == "bang");

        if !has_bang {
            let start = node.start_position();
            let end = node.end_position();
            let text = node.utf8_text(source.as_bytes()).unwrap_or("normal");

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
                severity: Some(DiagnosticSeverity::WARNING),
                source: Some("hjkls".to_string()),
                message: format!(
                    "Suspicious: '{}' uses `normal` without `!`. User mappings may interfere. Use `normal!` instead.",
                    text.trim()
                ),
                ..Default::default()
            });
        }
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_normal_bang_warnings_recursive(&child, source, diagnostics);
    }
}

/// Collect warnings for `=~` without case modifier (should use `=~#` or `=~?`)
fn collect_match_case_warnings_recursive(
    node: &tree_sitter::Node,
    source: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if node.kind() == "binary_operation" {
        let mut cursor = node.walk();
        let children: Vec<_> = node.children(&mut cursor).collect();

        // Check if this is a =~ operation
        let has_match_op = children.iter().any(|c| c.kind() == "=~");

        if has_match_op {
            // Check if there's a match_case modifier
            let has_case_modifier = children.iter().any(|c| c.kind() == "match_case");

            if !has_case_modifier {
                let start = node.start_position();
                let end = node.end_position();
                let text = node.utf8_text(source.as_bytes()).unwrap_or("=~");

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
                    severity: Some(DiagnosticSeverity::WARNING),
                    source: Some("hjkls".to_string()),
                    message: format!(
                        "Suspicious: '{}' uses `=~` without case modifier. Behavior depends on 'ignorecase' option. Use `=~#` (case-sensitive) or `=~?` (case-insensitive) instead.",
                        text.trim()
                    ),
                    ..Default::default()
                });
            }
        }
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_match_case_warnings_recursive(&child, source, diagnostics);
    }
}

/// Collect warnings for `autocmd` outside of `augroup`
///
/// In tree-sitter-vim, augroup_statement and autocmd_statement are siblings,
/// not parent-child. So we need to track state across siblings.
fn collect_autocmd_group_warnings_recursive(
    node: &tree_sitter::Node,
    source: &str,
    inside_augroup: bool,
    diagnostics: &mut Vec<Diagnostic>,
) -> bool {
    // If this is an augroup statement, update the state
    if node.kind() == "augroup_statement" {
        let mut cursor = node.walk();
        let children: Vec<_> = node.children(&mut cursor).collect();

        if let Some(name_node) = children.iter().find(|c| c.kind() == "augroup_name") {
            if let Ok(name) = name_node.utf8_text(source.as_bytes()) {
                // Return new state: true if entering augroup, false if exiting (END)
                return !name.eq_ignore_ascii_case("END");
            }
        }
        return inside_augroup;
    }

    // Check autocmd statements
    if node.kind() == "autocmd_statement" && !inside_augroup {
        let mut cursor = node.walk();
        let children: Vec<_> = node.children(&mut cursor).collect();

        // Check if this autocmd has an event list (actual autocmd registration)
        // autocmd! (with bang and no events) is just clearing, which is OK
        let has_events = children.iter().any(|c| c.kind() == "au_event_list");

        // Check if this autocmd has an inline group name (e.g., autocmd MyGroup BufRead ...)
        // This is valid even outside augroup block
        let has_inline_group = children.iter().any(|c| c.kind() == "augroup_name");

        if has_events && !has_inline_group {
            let start = node.start_position();
            let end = node.end_position();
            let text = node.utf8_text(source.as_bytes()).unwrap_or("autocmd");

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
                severity: Some(DiagnosticSeverity::WARNING),
                source: Some("hjkls".to_string()),
                message: format!(
                    "Suspicious: '{}' is defined outside of an augroup. This may cause duplicate autocmds on reload. Wrap in `augroup` with `autocmd!` to clear.",
                    text.lines().next().unwrap_or(text).trim()
                ),
                ..Default::default()
            });
        }
    }

    // Recurse into children, tracking augroup state across siblings
    let mut cursor = node.walk();
    let mut current_in_augroup = inside_augroup;
    for child in node.children(&mut cursor) {
        current_in_augroup = collect_autocmd_group_warnings_recursive(
            &child,
            source,
            current_in_augroup,
            diagnostics,
        );
    }

    // Return the current state (for sibling propagation at parent level)
    inside_augroup
}

/// Collect warnings for `set compatible` / `set cp`
///
/// Vi-compatible mode disables many Vim features and is rarely intended.
fn collect_set_compatible_warnings_recursive(
    node: &tree_sitter::Node,
    source: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if node.kind() == "set_statement" {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "set_item" {
                let mut item_cursor = child.walk();
                for item_child in child.children(&mut item_cursor) {
                    // Look for option_name directly under set_item (not under no_option)
                    // set compatible -> set_item -> option_name
                    // set nocompatible -> set_item -> no_option -> option_name
                    if item_child.kind() == "option_name" {
                        if let Ok(opt_name) = item_child.utf8_text(source.as_bytes()) {
                            if opt_name == "compatible" || opt_name == "cp" {
                                let start = node.start_position();
                                let end = node.end_position();
                                let text = node
                                    .utf8_text(source.as_bytes())
                                    .unwrap_or("set compatible");

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
                                    severity: Some(DiagnosticSeverity::WARNING),
                                    source: Some("hjkls".to_string()),
                                    message: format!(
                                        "Suspicious: '{}' enables Vi-compatible mode, which disables many Vim features. Is this intended?",
                                        text.trim()
                                    ),
                                    ..Default::default()
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_set_compatible_warnings_recursive(&child, source, diagnostics);
    }
}

/// Collect warnings for `vim9script` not at the start of the file
///
/// vim9script must be the very first statement in the file (even before comments).
/// tree-sitter-vim parses vim9script as unknown_builtin_statement with
/// unknown_command_name="vim" and arguments containing "9script".
fn collect_vim9script_position_warnings(
    root: &tree_sitter::Node,
    source: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    // Only check script_file nodes
    if root.kind() != "script_file" {
        return;
    }

    let mut cursor = root.walk();
    let mut is_first_statement = true;

    for child in root.children(&mut cursor) {
        // Check if this is a vim9script declaration
        if child.kind() == "unknown_builtin_statement" {
            let mut child_cursor = child.walk();
            let children: Vec<_> = child.children(&mut child_cursor).collect();

            // Check for vim9script pattern: unknown_command_name="vim" + arguments="9script"
            let is_vim9script = children.iter().any(|c| {
                c.kind() == "unknown_command_name"
                    && c.utf8_text(source.as_bytes()).unwrap_or("") == "vim"
            }) && children.iter().any(|c| {
                c.kind() == "arguments"
                    && c.utf8_text(source.as_bytes())
                        .unwrap_or("")
                        .contains("9script")
            });

            if is_vim9script {
                // vim9script found - warn if it's not the first statement
                if !is_first_statement {
                    let start = child.start_position();
                    let end = child.end_position();

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
                        severity: Some(DiagnosticSeverity::WARNING),
                        source: Some("hjkls".to_string()),
                        message:
                            "Suspicious: `vim9script` must be at the very first line of the file."
                                .to_string(),
                        ..Default::default()
                    });
                }
                // Stop checking after finding vim9script
                return;
            }
        }

        // Any statement (including comments) before vim9script is a problem
        is_first_statement = false;
    }
}
