-- E2E test runner init file for mini.test
-- Usage: nvim --headless -u test/e2e_init.lua

-- Disable unnecessary features
vim.opt.loadplugins = false
vim.opt.swapfile = false
vim.opt.backup = false

-- Get paths
local script_path = debug.getinfo(1, "S").source:sub(2)
local test_dir = vim.fs.dirname(script_path)
local repo_root = vim.fs.dirname(test_dir)

-- Store paths for tests to access (before setup so tests can use them)
_G.TEST_PATHS = {
  repo_root = repo_root,
  test_dir = test_dir,
  fixtures_dir = test_dir .. "/fixtures",
  hjkls_binary = repo_root .. "/target/debug/hjkls",
  child_init = test_dir .. "/minimal_init.lua",
}

-- Setup mini.test (loaded via Nix)
require("mini.test").setup({
  collect = {
    -- Look for test files in test/ directory instead of default tests/
    find_files = function()
      return vim.fn.glob(test_dir .. "/test_*.lua", false, true)
    end,
  },
  execute = {
    reporter = require("mini.test").gen_reporter.stdout({ group_depth = 2 }),
  },
})

-- Execute tests and exit
MiniTest.run()
vim.cmd("qa!")
