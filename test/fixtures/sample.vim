" Test file for hjkls
" This file contains examples for testing LSP features

" === Functions (goto_definition, hover) ===

function! Hello(name = 'World')
  echo "Hello, " . a:name
endfunction

function! s:PrivateHelper()
  return 42
endfunction

function! Greet(who)
  " Hover over 'Hello' to see signature
  " Press gd on 'Hello' to jump to definition
  call Hello(a:who)

  " Hover over 's:PrivateHelper' to see signature
  let l:result = s:PrivateHelper()
  return l:result
endfunction

" === Variables (goto_definition, hover) ===

let g:my_var = 42
let s:script_var = "hello"

function! UseVariables()
  " Press gd on 'g:my_var' to jump to definition
  echo g:my_var
  echo s:script_var
endfunction

" === Built-in functions (hover) ===

" Hover over 'strlen', 'exists', 'empty' to see built-in docs
let g:length = strlen("test")
if exists('g:my_var') && !empty(g:my_var)
  echo "Variable exists and is not empty"
endif

" === Autoload functions (hover, goto_definition) ===

" Hover over autoload function calls to see expected file path
" Press gd to jump to the definition in autoload/myplugin/util.vim
call myplugin#util#helper()
call myplugin#util#greet("World")

" These don't exist - hover shows path, gd won't work
call nonexistent#module#func()

" === Invalid argument count (should show warnings) ===

" Too few arguments
call strlen()
call search()

" Too many arguments
call Hello("alice", "bob")
call s:PrivateHelper("extra")
call empty(1, 2, 3)

" Valid: optional arguments
call Hello()
call Hello('alice')
call search('pattern')
call search('pattern', 'n')
call search('pattern', 'n', 100)

" === Scope violations (should show warnings) ===

" l: and a: are only valid inside functions
" These should trigger scope violation warnings:
let l:invalid_local = 1
echo a:invalid_arg

" Valid: l: inside function
function! ValidLocalScope()
  let l:valid = 1
  return l:valid
endfunction

" === Undefined function calls (should show warnings) ===

" Undefined script-local function
call s:NonExistentHelper()

" Undefined global function (capital)
call UndefinedGlobalFunc()

" Undefined lowercase function (not a built-in)
call notabuiltin()

" Valid: defined functions should NOT show warnings
call Hello()
call s:PrivateHelper()

" Valid: built-in functions should NOT show warnings
call strlen("test")
call empty([])

" === Suspicious patterns (should show warnings) ===

" normal_bang: should warn
normal j
normal k

" normal_bang: valid (has !)
normal! j

" match_case: should warn
if g:my_var =~ 'pattern'
endif

" match_case: valid
if g:my_var =~# 'pattern'
endif
if g:my_var =~? 'pattern'
endif

" autocmd_group: should warn
autocmd FileType vim echo "standalone"

" autocmd_group: valid (augroup block)
augroup MyTestGroup
  autocmd!
  autocmd BufRead * echo "in group"
augroup END

" autocmd_group: valid (inline group)
autocmd MyTestGroup BufEnter * echo "inline group"

" set_compatible: should warn
set compatible
set cp

" double_dot: should hint (use .. instead of .)
let g:concat_single = "hello" . "world"
let g:concat_chain = "a" . "b" . "c"

" double_dot: valid (already using ..)
let g:concat_double = 'hello' .. 'world'

" function_bang: should hint (s: functions don't need !)
function! s:ScriptLocalWithBang()
  return 1
endfunction

" function_bang: valid (s: without bang)
function s:ScriptLocalNoBang()
  return 2
endfunction

" function_bang: valid (global function with bang is OK)
function! GlobalFuncWithBang()
  return 3
endfunction

" abort: should hint (missing abort)
function! NoAbortFunc()
  return 1
endfunction

" abort: valid (has abort)
function! HasAbortFunc() abort
  return 2
endfunction

" single_quote: should hint (no escapes needed)
let g:double_simple = "hello"

" single_quote: valid (has escape sequence)
let g:double_escape = "hello\nworld"

" single_quote: valid (already single quote)
let g:single_simple = 'hello'

" single_quote: valid (contains single quote)
let g:double_with_quote = "it's a test"

" === Dynamic function calls (should NOT show undefined warnings) ===

" Lambda assigned to variable
let Lambda1 = { x -> x + 1 }
echo Lambda1(5)

" Funcref assigned to variable
let Funcref1 = function('strlen')
echo Funcref1('hello')

" Dictionary function
let dict1 = {}
function! dict1.method() abort
  return 'method called'
endfunction
echo dict1.method()

" Dictionary method with self
let obj1 = {}
function! obj1.greet() abort dict
  return 'hello'
endfunction
echo obj1.greet()

" Calling via a: scope (callback pattern)
function! s:RunCallback(callback) abort
  return a:callback()
endfunction
call s:RunCallback({ -> 'result' })

" self.method() call inside dict function
function! dict1.chain() abort dict
  return self.method()
endfunction

" Dictionary subscript call (a:args['callback']())
function! s:RunWithOptions(opts) abort
  return a:opts['callback']()
endfunction

" Local scope function reference (l:Func())
function! s:TestLocalFuncref() abort
  let l:Callback = { -> 'test' }
  return l:Callback()
endfunction

" === Key notation (should show hints) ===

" key_notation: should hint (lowercase)
nnoremap <cr> :echo 'hello'<CR>
nnoremap <esc> <Esc>

" key_notation: should hint (all caps arrow)
nnoremap <UP> k
nnoremap <DOWN> j

" key_notation: should hint (lowercase modifier)
nnoremap <c-a> <C-a>

" key_notation: valid (already correct)
nnoremap <CR> :quit<CR>
nnoremap <Esc> <Esc>
nnoremap <Up> k
nnoremap <C-a> <C-a>
