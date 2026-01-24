mod backend;
mod builtins;
mod completion;
mod db;
mod diagnostics;
mod logger;
mod symbols;

use std::path::PathBuf;

use tower_lsp_server::{LspService, Server};

use backend::Backend;
use builtins::EditorMode;

fn print_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

fn print_help() {
    println!(
        "{} - {}

Usage: {} [OPTIONS]

Options:
  -V, --version          Show version information
      --vim-only         Show only Vim-compatible functions in completion
      --neovim-only      Show only Neovim-compatible functions in completion
      --vimruntime=<PATH> Override $VIMRUNTIME path for autoload resolution
      --log=<PATH>       Enable debug logging to specified file
  -h, --help             Show this help message

This is an LSP server for Vim script. It communicates via stdin/stdout
using the Language Server Protocol.",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_NAME")
    );
}

#[tokio::main]
async fn main() {
    // Parse CLI arguments
    let args: Vec<String> = std::env::args().collect();

    let mut vim_only = false;
    let mut neovim_only = false;

    for arg in &args[1..] {
        match arg.as_str() {
            "-V" | "--version" => {
                print_version();
                return;
            }
            "-h" | "--help" => {
                print_help();
                return;
            }
            "--vim-only" => vim_only = true,
            "--neovim-only" => neovim_only = true,
            _ => {}
        }
    }

    // Check for conflicting options
    if vim_only && neovim_only {
        eprintln!("error: --vim-only and --neovim-only cannot be used together");
        std::process::exit(1);
    }

    let editor_mode = if vim_only {
        EditorMode::VimOnly
    } else if neovim_only {
        EditorMode::NeovimOnly
    } else {
        EditorMode::Both
    };

    // Parse --log=PATH argument
    let log_path = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--log=").map(String::from));
    logger::init(log_path);

    // Parse --vimruntime=PATH or get from environment
    let vimruntime: Option<PathBuf> = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--vimruntime=").map(PathBuf::from))
        .or_else(|| std::env::var("VIMRUNTIME").ok().map(PathBuf::from))
        .filter(|p| p.exists());

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) =
        LspService::new(|client| Backend::new(client, editor_mode, vimruntime.clone()));
    Server::new(stdin, stdout, socket).serve(service).await;
}
