-- E2E tests for completion
-- Tests that hjkls provides correct completion candidates

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

T["completion"] = MiniTest.new_set()

T["completion"]["completes builtin functions"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Position cursor after "str" to trigger completion
  child.cmd("normal! gg")
  child.cmd("normal! o")
  child.type_keys("call str")

  local completions = H.get_completions(child)

  -- Should include builtin string functions
  local has_strlen = vim.tbl_contains(completions, "strlen")
  local has_strpart = vim.tbl_contains(completions, "strpart")

  MiniTest.expect.equality(has_strlen or has_strpart, true, "Expected builtin string functions in completions")

  child.stop()
end

T["completion"]["completes user-defined functions"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Wait for indexing
  child.lua("vim.wait(2000)")

  -- Position cursor and type to trigger completion for user function
  child.cmd("normal! G")
  child.cmd("normal! o")
  child.type_keys("call Hel")

  local completions = H.get_completions(child)

  -- Should include the Hello function defined in sample.vim
  local has_hello = vim.tbl_contains(completions, "Hello")

  MiniTest.expect.equality(has_hello, true, "Expected user-defined Hello function in completions")

  child.stop()
end

T["completion"]["completes variables"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Wait for indexing
  child.lua("vim.wait(2000)")

  -- Type variable prefix
  child.cmd("normal! G")
  child.cmd("normal! o")
  child.type_keys("echo g:my")

  local completions = H.get_completions(child)

  -- Should include g:my_var
  local has_my_var = vim.tbl_contains(completions, "g:my_var")

  MiniTest.expect.equality(has_my_var, true, "Expected g:my_var in completions")

  child.stop()
end

return T
