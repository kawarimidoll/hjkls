# hjkls

Language Server Protocol (LSP) implementation for Vim script, written in Rust.

## Features

- [x] Syntax error diagnostics (via tree-sitter-vim)
- [ ] Completion for built-in functions
- [ ] Go to definition
- [ ] Hover information

## Requirements

- Rust 1.85+
- Nix (optional, for development environment)

## Development

### Setup with Nix (recommended)

```bash
cp .envrc.sample .envrc
direnv allow
```

### Build

```bash
cargo build          # Debug build
cargo build --release # Release build
```

### Test with Neovim

```bash
nvim -u test/minimal_init.lua test/test.vim
```

### Lint

```bash
cargo clippy
cargo fmt --check
```

## Editor Setup

### Neovim 0.11+

```lua
vim.lsp.config("hjkls", {
  cmd = { "/path/to/hjkls" },
  filetypes = { "vim" },
  root_markers = { ".git" },
})

vim.lsp.enable("hjkls")
```

## License

MIT
