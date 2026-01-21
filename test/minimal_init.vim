" Minimal Vim config for testing hjkls with vim-lsp
" Usage: vim -S test/minimal_init.vim test/fixtures/sample.vim
" Note: Using -S (not -u) to preserve Nix's vimrc which sets packpath for vim-lsp

" Disable unnecessary features
set noswapfile
set nobackup

" Remove user directories from runtimepath to avoid loading global config
set runtimepath-=$HOME/.vim
set runtimepath-=$HOME/.vim/after

" Enable syntax highlighting
syntax on

" Get the directory of this script
let s:script_dir = fnamemodify(resolve(expand('<sfile>:p')), ':h')
let s:repo_root = fnamemodify(s:script_dir, ':h')

" Add fixtures directory to runtimepath so autoload functions work
let s:fixtures_dir = s:script_dir . '/fixtures'
execute 'set runtimepath^=' . s:fixtures_dir

" Path to hjkls binary
let s:hjkls_path = s:repo_root . '/target/debug/hjkls'
let s:log_path = s:repo_root . '/logs/hjkls.log'

" Check if binary exists
if !executable(s:hjkls_path)
  echoerr 'hjkls binary not found at: ' . s:hjkls_path
  echomsg "Run 'cargo build' first"
  finish
endif

" Configure vim-lsp for hjkls
augroup hjkls_lsp
  autocmd!
  autocmd User lsp_setup call lsp#register_server({
        \ 'name': 'hjkls',
        \ 'cmd': [s:hjkls_path, '--log=' . s:log_path],
        \ 'allowlist': ['vim'],
        \ 'root_uri': {server_info->lsp#utils#path_to_uri(
        \   lsp#utils#find_nearest_parent_file_directory(
        \     lsp#utils#get_buffer_path(),
        \     ['.git']
        \   )
        \ )},
        \ })
augroup END

" Enable vim-lsp features
let g:lsp_diagnostics_enabled = 1
let g:lsp_diagnostics_echo_cursor = 1
let g:lsp_diagnostics_virtual_text_enabled = 1
let g:lsp_document_code_action_signs_enabled = 0
let g:lsp_log_verbose = 1
let g:lsp_log_file = s:repo_root . '/logs/vim-lsp.log'

" Key mappings (similar to Neovim defaults)
function! s:on_lsp_buffer_enabled() abort
  setlocal omnifunc=lsp#complete
  setlocal signcolumn=yes

  " Navigation
  nmap <buffer> gd <plug>(lsp-definition)
  nmap <buffer> <C-]> <plug>(lsp-definition)
  nmap <buffer> grr <plug>(lsp-references)

  " Information
  nmap <buffer> K <plug>(lsp-hover)
  nmap <buffer> gs <plug>(lsp-signature-help)
  nmap <buffer> gO <plug>(lsp-document-symbol)

  " Completion settings
  " Use <C-x><C-o> for omnifunc completion
  " NOTE: 'autocomplete' (Vim 9.1) causes E565 with vim-lsp, disabled
  setlocal complete=.,o
  setlocal completeopt=menuone,noselect

  " Folding (zc=close, zo=open, zR=open all, zM=close all)
  setlocal foldmethod=expr
  setlocal foldexpr=lsp#ui#vim#folding#foldexpr()
  setlocal foldtext=lsp#ui#vim#folding#foldtext()
  setlocal foldlevel=99
endfunction

augroup lsp_install
  autocmd!
  autocmd User lsp_buffer_enabled call s:on_lsp_buffer_enabled()
augroup END

echowindow 'hjkls LSP configured: ' . s:hjkls_path
