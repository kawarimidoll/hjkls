" Test file for syntax errors
" This file intentionally contains syntax errors for testing diagnostics

" Unclosed function (line 5)
function! Broken(
endfunction

" Missing endif (line 9)
if 1
  echo "oops"

" Invalid let statement (line 13)
let = "no variable name"
