# Formatting

hjkls provides automatic code formatting for Vim script files via the LSP `textDocument/formatting` request.

## Features

| Feature | Description |
|---------|-------------|
| **Trailing whitespace removal** | Removes trailing spaces/tabs from each line |
| **Final newline insertion** | Ensures files end with a newline |
| **Block indentation** | Automatically indents blocks (function/if/for/while/try/augroup) |
| **Line continuation indentation** | Indents continuation lines starting with `\` |
| **Space normalization** | Reduces multiple consecutive spaces to single space |

## Usage

### Neovim

```lua
-- Format current buffer
vim.lsp.buf.format()

-- Or bind to a key
vim.keymap.set('n', '<leader>f', vim.lsp.buf.format, { desc = 'Format buffer' })
```

### Vim

With a compatible LSP client:

```vim
" Format current buffer
call lsp#request('textDocument/formatting')
```

## Example

Before formatting:

```vim
function! MyFunc()
let x = 1
if x == 1
let y = 2
endif
endfunction
```

After formatting:

```vim
function! MyFunc()
  let x = 1
  if x == 1
    let y = 2
  endif
endfunction
```

## Indentation Rules

### Block Indentation

The formatter adds one level of indentation (default: 2 spaces) inside these block structures:

- `function` / `endfunction`
- `if` / `elseif` / `else` / `endif`
- `for` / `endfor`
- `while` / `endwhile`
- `try` / `catch` / `finally` / `endtry`
- `augroup` / `augroup END`

Nested blocks receive cumulative indentation:

```vim
function! Example()
  if condition
    for item in list
      call process(item)
    endfor
  endif
endfunction
```

### Line Continuation

Lines starting with `\` (Vim's line continuation character) receive additional indentation (default: 6 spaces = indent_width × 3):

```vim
let long_list = [
      \ 'item1',
      \ 'item2',
      \ 'item3',
      \ ]
```

### Space Normalization

Multiple consecutive spaces are reduced to a single space:

```vim
" Before
echo       'Hello'  ..          name

" After
echo 'Hello' .. name
```

**Note:** Spaces inside string literals and comments are preserved:

```vim
let msg = 'hello     world'   " Preserved inside string
" This   comment   is   also   preserved
```

## Configuration

Configure formatting in `.hjkls.toml`:

```toml
[format]
indent_width = 2                # Spaces per indent level (default: 2)
use_tabs = false                # Use tabs instead of spaces (default: false)
line_continuation_indent = 6    # Extra indent for \ lines (default: indent_width × 3)
trim_trailing_whitespace = true # Remove trailing whitespace (default: true)
insert_final_newline = true     # Add newline at end of file (default: true)
normalize_spaces = true         # Reduce multiple spaces to single (default: true)
```

### Tab Indentation

When `use_tabs = true`, the formatter uses tabs for full indent levels and spaces for any remainder:

```toml
[format]
indent_width = 4
use_tabs = true
```

Results in:
- 4-space indent → 1 tab
- 8-space indent → 2 tabs
- 6-space indent → 1 tab + 2 spaces

### Disabling Features

You can disable individual formatting features:

```toml
[format]
trim_trailing_whitespace = false  # Keep trailing whitespace
insert_final_newline = false      # Don't add final newline
normalize_spaces = false          # Keep multiple consecutive spaces
```

> **Note:** Changes to `.hjkls.toml` require restarting the LSP server to take effect.
