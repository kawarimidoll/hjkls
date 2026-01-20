-- E2E tests for find references
-- Tests that hjkls correctly finds all references to a symbol

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

T["references"] = MiniTest.new_set()

T["references"]["finds function definition and calls"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Wait for indexing
  child.lua("vim.wait(2000)")

  -- Position on Hello function definition (line 6)
  child.cmd("normal! 6G")
  child.cmd("normal! 0f!w") -- Position on "Hello"

  local refs = H.get_references(child)

  -- Hello is defined on line 6 and called on lines 17, 60, 64, 65
  -- At minimum, should find definition + at least one call
  MiniTest.expect.equality(#refs >= 2, true, "Expected at least 2 references for Hello function")

  child.stop()
end

T["references"]["finds variable definition and usages"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Wait for indexing
  child.lua("vim.wait(2000)")

  -- Position on g:my_var definition (line 26)
  child.cmd("normal! 26G")
  child.cmd("normal! 0fg") -- Position on "g:my_var"

  local refs = H.get_references(child)

  -- g:my_var is defined on line 26, used on lines 31, 39
  MiniTest.expect.equality(#refs >= 2, true, "Expected at least 2 references for g:my_var")

  child.stop()
end

T["references"]["finds script-local function references"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Wait for indexing
  child.lua("vim.wait(2000)")

  -- Position on s:PrivateHelper definition (line 10)
  child.cmd("normal! 10G")
  child.cmd("normal! 0f!w") -- Position on "s:PrivateHelper"

  local refs = H.get_references(child)

  -- s:PrivateHelper is defined on line 10, called on lines 20, 61
  MiniTest.expect.equality(#refs >= 2, true, "Expected at least 2 references for s:PrivateHelper")

  child.stop()
end

return T
