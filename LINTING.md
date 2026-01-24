# Lint Rules

hjkls provides lint diagnostics beyond syntax errors. Rules are organized into categories by severity.

## Categories

| Category | Default | Severity | Description |
|----------|---------|----------|-------------|
| **correctness** | Enabled | Error | Likely bugs or incorrect code |
| **suspicious** | Enabled | Warning | Code that may behave unexpectedly |
| **style** | Enabled | Hint | Style suggestions for better code |

## Correctness Rules (Error)

These rules detect code that is likely incorrect.

### `undefined_function`

Warns when calling a function that is not defined.

```vim
" Warning: Undefined function
call NonExistentFunc()
```

**Checks against:**
- Built-in functions (786 functions)
- User-defined functions in the same file
- Global functions in the workspace
- Autoload functions (via `$VIMRUNTIME`)

### `scope_violation`

Warns when using `l:` (local) or `a:` (argument) scope outside of a function.

```vim
" Warning: l: is only valid inside functions
let l:invalid = 1

function! Valid()
  let l:valid = 1  " OK
endfunction
```

### `argument_count_mismatch`

Warns when calling a function with wrong number of arguments.

```vim
" Warning: strlen() expects 1 argument
echo strlen()

" Warning: empty() expects 1 argument
echo empty(1, 2, 3)
```

Supports optional arguments (e.g., `search(pattern [, flags [, stopline]])`).

## Suspicious Rules (Warning)

These rules detect code that may behave unexpectedly. Inspired by [vint](https://github.com/Vimjas/vint).

### `normal_bang`

**Origin:** vint (`ProhibitCommandRelyOnUser`)

Warns when using `normal` without `!`. User mappings can interfere with `normal` commands.

```vim
" Warning: use normal! instead
normal j

" OK
normal! j
```

### `match_case`

**Origin:** vint (`ProhibitEqualTildeOperator`)

Warns when using `=~` without case modifier. Behavior depends on `'ignorecase'` option.

```vim
" Warning: depends on 'ignorecase'
if str =~ 'pattern'

" OK: explicit case handling
if str =~# 'pattern'  " case-sensitive
if str =~? 'pattern'  " case-insensitive
```

### `autocmd_group`

**Origin:** vint (`ProhibitAutocmdWithNoGroup`)

Warns when defining `autocmd` outside of an `augroup`. Re-sourcing the script will duplicate the autocmd.

```vim
" Warning: autocmd outside augroup
autocmd BufRead * echo "hello"

" OK: wrapped in augroup
augroup MyGroup
  autocmd!
  autocmd BufRead * echo "hello"
augroup END
```

### `set_compatible`

**Origin:** vint (`ProhibitSetNoCompatible`)

Warns when enabling Vi-compatible mode. This resets many options and is rarely intended.

```vim
" Warning: enables Vi-compatible mode
set compatible
set cp
```

### `vim9script_position`

**Origin:** hjkls original

Warns when `vim9script` is not at the very first line of the file.

```vim
" Warning: vim9script must be first
let g:foo = 1
vim9script  " Too late!
```

## Style Rules (Hint)

These rules suggest improvements for code style. They don't indicate bugs but help maintain consistency.

### `double_dot`

**Origin:** hjkls original

Suggests using `..` instead of `.` for string concatenation. In Vim9 script, `..` is required.

```vim
" Hint: use .. instead
let s = "hello" . "world"

" OK
let s = "hello" .. "world"
```

### `function_bang`

**Origin:** hjkls original

Suggests removing `!` from script-local function definitions. The `!` allows overwriting existing functions, but `s:` functions are only visible within the same script, making `!` unnecessary.

```vim
" Hint: ! is unnecessary for s: functions
function! s:MyHelper()
  return 1
endfunction

" OK
function s:MyHelper()
  return 1
endfunction

" OK: global functions may need ! to avoid E122
function! GlobalFunc()
  return 2
endfunction
```

### `abort`

**Origin:** vint (`ProhibitMissingAbort`)

Suggests adding `abort` attribute to function definitions. Without `abort`, functions continue execution after an error occurs, which can lead to unexpected behavior.

```vim
" Hint: missing abort attribute
function! MyFunc()
  return 1
endfunction

" OK: has abort
function! MyFunc() abort
  return 1
endfunction
```

### `single_quote`

**Origin:** hjkls original

Suggests using single quotes for strings that don't require escape sequences. Single-quoted strings are simpler and more readable when no special characters are needed.

```vim
" Hint: use single quotes
let s = "hello"

" OK: already single-quoted
let s = 'hello'

" OK: contains escape sequence
let s = "hello\nworld"

" OK: contains single quote
let s = "it's a test"
```

### `key_notation`

**Origin:** hjkls original

Normalizes key notation to match Vim's standard help notation (`:h key-notation`). This improves readability and consistency.

```vim
" Hint: use standard notation
nnoremap <cr> :echo "hello"<CR>
nnoremap <esc> <ESC>
nnoremap <UP> k
nnoremap <c-a> <C-a>

" OK: already standard
nnoremap <CR> :quit<CR>
nnoremap <Esc> <Esc>
nnoremap <Up> k
nnoremap <C-a> <C-a>
```

**Standard notations:**
- Special keys: `<CR>`, `<Tab>`, `<Esc>`, `<Space>`, `<BS>`, `<Del>`, etc.
- Arrow keys: `<Up>`, `<Down>`, `<Left>`, `<Right>`
- Function keys: `<F1>` through `<F12>`
- Modifiers: `<C-...>`, `<S-...>`, `<M-...>`, `<A-...>` (uppercase modifier letter)

### `plug_noremap`

**Origin:** hjkls original

Suggests using `noremap` variants (`nnoremap`, `vnoremap`, etc.) for `<Plug>` mappings. According to `:help noremap`, `<Plug>` is always remapped even when using `noremap`, so using `nmap` vs `nnoremap` makes no difference for `<Plug>` mappings. However, `noremap` variants are preferred for consistency and to avoid accidentally remapping other keys in the same mapping.

```vim
" Hint: use nnoremap for <Plug> mapping
nmap a <Plug>(some-function)

" OK
nnoremap a <Plug>(some-function)
```

## Suppressing Diagnostics

You can suppress diagnostics using inline comments.

### `hjkls:ignore-next-line`

Suppresses diagnostics on the next line only.

```vim
" hjkls:ignore-next-line suspicious#normal_bang
normal j  " No warning on this line

normal k  " Warning appears here (not suppressed)
```

### `hjkls:ignore`

Suppresses diagnostics from this line to the end of the file.

```vim
" hjkls:ignore suspicious#normal_bang
normal j  " No warning
normal k  " No warning
```

### Rule Specification

Rules are specified as `category#rule_name`:

```vim
" Suppress a specific rule
" hjkls:ignore suspicious#normal_bang

" Suppress multiple rules
" hjkls:ignore suspicious#normal_bang, style#double_dot

" Suppress all diagnostics (no rules specified)
" hjkls:ignore
```

### Comment Styles

Both legacy Vim script and Vim9 script comment styles are supported:

```vim
" Legacy Vim script
" hjkls:ignore suspicious#normal_bang

# Vim9 script
# hjkls:ignore suspicious#normal_bang
```

### Limitations

The comment detection uses a simple heuristic (whitespace-preceded `"` or `#`) and may produce false positives for these characters inside string literals. In practice, this is rarely an issue since `hjkls:ignore` is an unusual string to appear in code.
