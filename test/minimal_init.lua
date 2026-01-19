-- Minimal Neovim config for testing hjkls
-- Usage: nvim -u test/minimal_init.lua test/test.vim

-- Disable unnecessary features for faster startup
vim.opt.loadplugins = false
vim.opt.swapfile = false
vim.opt.backup = false

-- Get the directory of this script
local script_dir = vim.fn.fnamemodify(debug.getinfo(1, "S").source:sub(2), ":h")
local repo_root = vim.fn.fnamemodify(script_dir, ":h")

-- Path to hjkls binary
local hjkls_path = repo_root .. "/target/debug/hjkls"

-- Check if binary exists
if vim.fn.executable(hjkls_path) == 0 then
  vim.notify("hjkls binary not found at: " .. hjkls_path, vim.log.levels.ERROR)
  vim.notify("Run 'cargo build' first", vim.log.levels.INFO)
  return
end

-- Configure LSP
vim.lsp.config("hjkls", {
  cmd = { hjkls_path },
  filetypes = { "vim" },
  root_markers = { ".git" },
})

vim.lsp.enable("hjkls")

-- Show diagnostics in virtual text
vim.diagnostic.config({
  virtual_text = true,
  signs = true,
  underline = true,
})

vim.notify("hjkls LSP configured: " .. hjkls_path, vim.log.levels.INFO)
