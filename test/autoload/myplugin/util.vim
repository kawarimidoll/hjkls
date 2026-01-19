" Autoload functions for myplugin#util#*

function! myplugin#util#helper()
  return "Hello from autoload!"
endfunction

function! myplugin#util#greet(name)
  echo "Hello, " . a:name
endfunction

let s:private_var = 42
