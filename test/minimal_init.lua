-- Minimal Neovim config for testing hjkls
-- Usage: nvim -u test/minimal_init.lua test/fixtures/sample.vim

-- Disable unnecessary features for faster startup
vim.opt.loadplugins = false
vim.opt.swapfile = false
vim.opt.backup = false

-- Get the directory of this script
local script_path = debug.getinfo(1, "S").source:sub(2)
local script_dir = vim.fs.dirname(script_path)
local repo_root = vim.fs.dirname(script_dir)

-- Add fixtures directory to runtimepath so autoload functions work
local fixtures_dir = script_dir .. "/fixtures"
vim.opt.runtimepath:prepend(fixtures_dir)

-- Path to hjkls binary
local hjkls_path = repo_root .. "/target/debug/hjkls"

-- Check if binary exists
if vim.fn.executable(hjkls_path) == 0 then
  vim.notify("hjkls binary not found at: " .. hjkls_path, vim.log.levels.ERROR)
  vim.notify("Run 'cargo build' first", vim.log.levels.INFO)
  return
end

-- Configure LSP with debug logging
local log_path = repo_root .. "/logs/hjkls.log"

vim.lsp.config("hjkls", {
  cmd = { hjkls_path, "--log=" .. log_path },
  cmd_env = { RUST_BACKTRACE = "1" },
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

-- Custom LSP keymaps
-- Neovim defaults: grr=references, gO=document_symbol, K=hover, <C-]>=definition
-- Neovim 0.11+: <C-s> (insert) = signature_help
-- See :h lsp-defaults for full list
vim.api.nvim_create_autocmd("LspAttach", {
  callback = function(args)
    local opts = { buffer = args.buf }
    vim.keymap.set("n", "gd", vim.lsp.buf.definition, opts) -- shortcut (default: <C-]>)
    vim.keymap.set("n", "gs", vim.lsp.buf.signature_help, opts) -- normal mode shortcut

    -- Enable autocompletion (Neovim 0.11+)
    vim.opt.completeopt = { "menuone", "noselect", "fuzzy" }
    local client = vim.lsp.get_client_by_id(args.data.client_id)
    if client and client:supports_method("textDocument/completion") then
      vim.lsp.completion.enable(true, client.id, args.buf, { autotrigger = true })
    end
  end,
})

-- Document Highlight: highlight references to the symbol under cursor
vim.api.nvim_create_autocmd({ "CursorHold", "CursorHoldI" }, {
  callback = function()
    if #vim.lsp.get_clients({ bufnr = 0 }) > 0 then
      vim.lsp.buf.document_highlight()
    end
  end,
})
vim.api.nvim_create_autocmd({ "CursorMoved", "CursorMovedI" }, {
  callback = function()
    vim.lsp.buf.clear_references()
  end,
})

vim.notify("hjkls LSP configured: " .. hjkls_path, vim.log.levels.INFO)
