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
call Hello("alice")
call search("pattern")
call search("pattern", "n")
call search("pattern", "n", 100)

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
