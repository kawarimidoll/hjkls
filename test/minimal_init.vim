" Minimal Vim config for testing hjkls with yegappan/lsp
" Usage: vim -S test/minimal_init.vim test/fixtures/sample.vim
" Note: Using -S (not -u) to preserve Nix's vimrc which sets packpath for yegappan/lsp

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

" Configure yegappan/lsp for hjkls
let s:lspServers = [#{
      \   name: 'hjkls',
      \   filetype: ['vim'],
      \   path: s:hjkls_path,
      \   args: ['--log=' . s:log_path],
      \ }]

" yegappan/lsp options (must be set before LspAddServer)
let s:lspOptions = #{
      \   autoComplete: v:true,
      \   autoHighlight: v:true,
      \   showDiagOnStatusLine: v:true,
      \   showDiagWithVirtualText: v:true,
      \   showSignature: v:true,
      \ }

" Register server on VimEnter (after plugins are loaded)
autocmd VimEnter * ++once call s:setup_lsp()

function! s:setup_lsp() abort
  call LspOptionsSet(s:lspOptions)
  call LspAddServer(s:lspServers)
endfunction

" Key mappings (similar to Neovim defaults)
function! s:on_lsp_attached() abort
  setlocal signcolumn=yes

  " Completion: <C-x><C-o> for manual trigger
  setlocal omnifunc=lsp#lsp#OmniFunc
  setlocal completeopt=menuone,popup,noinsert,noselect

  " Navigation
  nnoremap <buffer> gd <Cmd>LspGotoDefinition<CR>
  nnoremap <buffer> <C-]> <Cmd>LspGotoDefinition<CR>
  nnoremap <buffer> grr <Cmd>LspShowReferences<CR>

  " Information
  nnoremap <buffer> K <Cmd>LspHover<CR>
  nnoremap <buffer> gs <Cmd>LspShowSignature<CR>
  nnoremap <buffer> gO <Cmd>LspDocumentSymbol<CR>

  " Actions
  nnoremap <buffer> grn <Cmd>LspRename<CR>
  nnoremap <buffer> gra <Cmd>LspCodeAction<CR>

  " Folding: use :LspFold to create folds from LSP
  " (zc=close, zo=open, zR=open all, zM=close all)
  nnoremap <buffer> zF <Cmd>LspFold<CR>
endfunction

augroup hjkls_lsp_attach
  autocmd!
  autocmd User LspAttached call s:on_lsp_attached()
augroup END

echowindow 'hjkls LSP configured (yegappan/lsp): ' . s:hjkls_path
