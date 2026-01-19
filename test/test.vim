" Test file for hjkls syntax diagnostics
" This file contains both valid and invalid Vim script

" === Valid syntax ===

function! Hello(name)
  echo "Hello, " . a:name
endfunction

let g:my_var = 42

if exists('g:my_var')
  echo "Variable exists"
endif

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
