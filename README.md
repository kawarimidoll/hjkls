# hjkls

Language Server Protocol (LSP) implementation for Vim script, written in Rust.

## Features

- [x] Syntax error diagnostics (via tree-sitter-vim)
- [x] Completion (built-in functions + user-defined symbols with scope support)
- [x] Go to definition (same file + cross-file autoload support)
- [x] Hover information (function signatures, autoload file paths)
- [x] Find references (same file + cross-file)
- [x] Document symbols (outline)
- [x] Rename (cross-file support)
- [x] Signature help (parameter info on function calls)

## Builtin Function Coverage

**Total: 600 functions**

| Category | Count | Description |
|----------|-------|-------------|
| Common | ~420 | Vim/Neovim shared functions (`strlen`, `expand`, `bufnr`, etc.) |
| Vim-only | ~90 | `popup_*`, `ch_*`, `job_*`, `term_*`, etc. |
| Neovim-only | ~56 | `nvim_*` API functions |

### Not Supported

| Category | Reason |
|----------|--------|
| `test_*` functions (~25) | Internal testing functions, not used in plugin development |

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

#### Optional: Enable autocompletion

```lua
vim.api.nvim_create_autocmd("LspAttach", {
  callback = function(args)
    vim.bo[args.buf].complete = ".,o"
    vim.bo[args.buf].autocomplete = true
    vim.opt.completeopt = { "menuone", "noselect" }
  end,
})
```

## License

MIT
