" Test file for hjkls
" This file contains examples for testing LSP features

" === Functions (goto_definition, hover) ===

function! Hello(name)
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

" === Invalid syntax (should show errors) ===

" Unclosed function
function! Broken(
endfunction

" Missing endif
if 1
  echo "oops"

" Invalid let statement
let = "no variable name"

" === More valid syntax ===

autocmd BufRead *.txt echo "Reading text file"

command! -nargs=1 Greet echo "Hello, " . <q-args>
