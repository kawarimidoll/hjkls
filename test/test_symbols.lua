-- E2E tests for document and workspace symbols
-- Tests that hjkls correctly lists symbols

local T = MiniTest.new_set({
  hooks = {
    pre_case = function()
      local hjkls = _G.TEST_PATHS.hjkls_binary
      if vim.fn.executable(hjkls) == 0 then
        error("hjkls binary not found. Run 'cargo build' first.")
      end
    end,
  },
})

T["document_symbol"] = MiniTest.new_set()

T["document_symbol"]["lists all functions and variables"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Wait for parsing
  child.lua("vim.wait(1000)")

  local symbols = H.get_document_symbols(child)

  -- Should include functions defined in sample.vim
  local has_hello = vim.tbl_contains(symbols, "Hello")
  local has_greet = vim.tbl_contains(symbols, "Greet")
  local has_private = false
  for _, name in ipairs(symbols) do
    if name:match("PrivateHelper") then
      has_private = true
      break
    end
  end

  MiniTest.expect.equality(has_hello, true, "Expected Hello in document symbols")
  MiniTest.expect.equality(has_greet, true, "Expected Greet in document symbols")
  MiniTest.expect.equality(has_private, true, "Expected s:PrivateHelper in document symbols")

  child.stop()
end

T["document_symbol"]["includes variables"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Wait for parsing
  child.lua("vim.wait(1000)")

  local symbols = H.get_document_symbols(child)

  -- Should include variables
  local has_my_var = false
  local has_script_var = false
  for _, name in ipairs(symbols) do
    if name:match("my_var") then
      has_my_var = true
    end
    if name:match("script_var") then
      has_script_var = true
    end
  end

  MiniTest.expect.equality(has_my_var, true, "Expected g:my_var in document symbols")
  MiniTest.expect.equality(has_script_var, true, "Expected s:script_var in document symbols")

  child.stop()
end

T["workspace_symbol"] = MiniTest.new_set()

T["workspace_symbol"]["finds symbols across files"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Wait for indexing to complete
  child.lua("vim.wait(3000)")

  local symbols = H.get_workspace_symbols(child, "helper")

  -- Should find myplugin#util#helper from autoload file
  local has_helper = false
  for _, name in ipairs(symbols) do
    if name:match("helper") then
      has_helper = true
      break
    end
  end

  MiniTest.expect.equality(has_helper, true, "Expected helper function in workspace symbols")

  child.stop()
end

T["workspace_symbol"]["filters by query"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Wait for indexing
  child.lua("vim.wait(3000)")

  -- Search for "Hello" specifically
  local symbols = H.get_workspace_symbols(child, "Hello")

  -- Should find Hello function
  local has_hello = vim.tbl_contains(symbols, "Hello")

  MiniTest.expect.equality(has_hello, true, "Expected Hello in filtered workspace symbols")

  -- Should NOT include unrelated symbols
  local has_greet = vim.tbl_contains(symbols, "Greet")
  MiniTest.expect.equality(has_greet, false, "Expected Greet NOT in filtered workspace symbols")

  child.stop()
end

return T
