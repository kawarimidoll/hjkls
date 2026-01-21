# hjkls

Language Server Protocol (LSP) implementation for Vim script, written in Rust.

## Features

- [x] Diagnostics (syntax errors, scope violations via tree-sitter-vim)
- [x] Completion (built-in functions + user-defined symbols with scope support)
- [x] Go to definition (same file + cross-file autoload support)
- [x] Hover information (function signatures, autoload file paths)
- [x] Find references (same file + cross-file)
- [x] Document symbols (outline)
- [x] Rename (cross-file support)
- [x] Signature help (parameter info on function calls)
- [x] Workspace symbols (project-wide symbol search)
- [x] Document highlight (highlight symbol under cursor)
- [x] Folding range (function/if/for/while/try/augroup)

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

## Installation

### Try without installing

```bash
nix run github:kawarimidoll/hjkls
```

### Home Manager (Flakes)

Add to your `flake.nix` inputs:

```nix
{
  inputs = {
    hjkls.url = "github:kawarimidoll/hjkls";
    # ...
  };
}
```

Then add to your home configuration:

```nix
{ inputs, pkgs, ... }:
{
  home.packages = [
    inputs.hjkls.packages.${pkgs.system}.default
  ];
}
```

### Cargo

```bash
cargo install --git https://github.com/kawarimidoll/hjkls
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

Neovim 0.11:

```lua
vim.api.nvim_create_autocmd("LspAttach", {
  callback = function(args)
    vim.opt.completeopt = { "menuone", "noselect", "fuzzy" }
    local client = vim.lsp.get_client_by_id(args.data.client_id)
    if client and client:supports_method("textDocument/completion") then
      vim.lsp.completion.enable(true, client.id, args.buf, { autotrigger = true })
    end
  end,
})
```

Neovim 0.12+ (nightly):

```lua
vim.api.nvim_create_autocmd("LspAttach", {
  callback = function(args)
    -- 'complete' with 'o' flag: include omnifunc (LSP) in CTRL-N/P completion
    -- 'autocomplete': trigger completion automatically as you type
    vim.bo[args.buf].complete = ".,o"
    vim.bo[args.buf].autocomplete = true
    vim.opt.completeopt = { "menuone", "noselect", "fuzzy" }
  end,
})
```

#### Optional: Enable document highlight

```lua
-- Highlight references to the symbol under cursor
vim.api.nvim_create_autocmd({ "CursorHold", "CursorHoldI" }, {
  callback = function()
    if #vim.lsp.get_clients({ bufnr = 0 }) > 0 then
      vim.lsp.buf.document_highlight()
    end
  end,
})
vim.api.nvim_create_autocmd({ "CursorMoved", "CursorMovedI" }, {
  callback = function()
    vim.lsp.buf.clear_references()
  end,
})
```

#### Optional: Enable LSP-based folding

```lua
-- Enable LSP folding (zc=close, zo=open, zR=open all, zM=close all)
vim.api.nvim_create_autocmd("LspAttach", {
  callback = function(args)
    local client = vim.lsp.get_client_by_id(args.data.client_id)
    if client and client:supports_method("textDocument/foldingRange") then
      vim.wo[args.buf].foldmethod = "expr"
      vim.wo[args.buf].foldexpr = "v:lua.vim.lsp.foldexpr()"
      vim.wo[args.buf].foldlevel = 99 -- start with all folds open
    end
  end,
})
```

## Development

### Setup

```bash
cp .envrc.sample .envrc
direnv allow
```

### Commands

```bash
just          # Show available commands
just build    # Build debug binary
just release  # Build release binary
just check    # Run clippy and check
just fmt      # Format code (Rust, Markdown, YAML, Nix)
just test     # Run tests
just dev-nvim # Open sample file in Neovim for manual testing
just dev-vim  # Open sample file in Vim for manual testing
```

## License

MIT
