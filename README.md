# hjkls

Language Server Protocol (LSP) implementation for Vim script, written in Rust.

## Features

- [x] Diagnostics (syntax errors + [lint rules](LINTING.md))
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
- [x] Selection range (smart expand selection via syntax tree)
- [x] Code actions (quick fixes for lint rules)
- [x] Formatting ([configuration](FORMATTING.md))

## Builtin Function Coverage

**Total: 786 functions**

| Category    | Count | Description                                                     |
| ----------- | ----- | --------------------------------------------------------------- |
| Common      | ~420  | Vim/Neovim shared functions (`strlen`, `expand`, `bufnr`, etc.) |
| Vim-only    | ~130  | `popup_*`, `ch_*`, `job_*`, `term_*`, `test_*`, etc.            |
| Neovim-only | ~170  | `nvim_*` API functions                                          |

## Builtin Variable Coverage

**Total: 149 variables**

| Scope | Count | Examples                                      |
| ----- | ----- | --------------------------------------------- |
| `v:`  | 126   | `v:version`, `v:errmsg`, `v:true`, `v:false`  |
| `b:`  | 2     | `b:changedtick`, `b:current_syntax`           |
| `g:`  | 21    | `g:colors_name`, `g:mapleader`, `g:clipboard` |

Only variables with help documentation tags are supported. Runtime-defined variables (e.g., `g:markdown_*` syntax options) are excluded due to dynamic naming and noise.

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

### Vim 9.0+ (yegappan/lsp)

[yegappan/lsp](https://github.com/yegappan/lsp) is a Vim9 script-based LSP client.

```vim
let lspServers = [#{
      \   name: 'hjkls',
      \   filetype: ['vim'],
      \   path: '/path/to/hjkls',
      \   args: [],
      \ }]

let lspOptions = #{
      \   autoComplete: v:true,
      \   autoHighlight: v:true,
      \   showDiagWithVirtualText: v:true,
      \   showSignature: v:true,
      \ }

autocmd VimEnter * ++once call LspOptionsSet(lspOptions) | call LspAddServer(lspServers)
```

### Vim 8.0+ (vim-lsp)

[vim-lsp](https://github.com/prabirshrestha/vim-lsp) is a popular async LSP client.

```vim
augroup hjkls_lsp
  autocmd!
  autocmd User lsp_setup call lsp#register_server({
        \ 'name': 'hjkls',
        \ 'cmd': ['/path/to/hjkls'],
        \ 'allowlist': ['vim'],
        \ })
augroup END

let g:lsp_diagnostics_enabled = 1
let g:lsp_diagnostics_virtual_text_enabled = 1
```

## Neovim Optional Settings

### Autocompletion

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

### Document Highlight

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

### LSP-based Folding

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

## Known Issues

### `<Cmd>` mapping false positives

[tree-sitter-vim](https://github.com/tree-sitter-grammars/tree-sitter-vim) (v0.4.0) cannot correctly parse `<Cmd>...<CR>` style mappings (e.g., `nmap qu <Cmd>quit<CR>`). The grammar's `command_argument: /\S+/` greedily consumes `<CR>`, preventing `_map_rhs_statement` from recognizing its closing token. This causes the parser to emit ERROR or MISSING nodes.

hjkls includes a workaround to detect and suppress these false positives so that valid `<Cmd>` mappings do not appear as syntax errors.

## Related Projects

- [tower-lsp-server](https://github.com/tombi-toml/tower-lsp-server) - LSP server framework for Rust
- [texter](https://github.com/airblast-dev/texter) - Text management library with tree-sitter integration
- [tree-sitter-vim](https://github.com/tree-sitter-grammars/tree-sitter-vim) - Vim script grammar for tree-sitter
- [salsa](https://github.com/salsa-rs/salsa) - Incremental computation framework
- [vim-language-server](https://github.com/iamcco/vim-language-server) - Prior art: Vim script LSP in TypeScript

## License

MIT
