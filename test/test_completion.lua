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

  -- Position cursor after "str" to trigger completion (function context)
  -- Use nvim_buf_set_lines + nvim_win_set_cursor for reliable text insertion
  child.cmd("normal! G")
  child.lua([[
    local line_count = vim.api.nvim_buf_line_count(0)
    vim.api.nvim_buf_set_lines(0, line_count, line_count, false, {"let x = str"})
    vim.api.nvim_win_set_cursor(0, {line_count + 1, 11})
  ]])

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

  -- Position cursor and type to trigger completion for user function (expression context)
  child.cmd("normal! G")
  child.lua([[
    local line_count = vim.api.nvim_buf_line_count(0)
    vim.api.nvim_buf_set_lines(0, line_count, line_count, false, {"let x = Hel"})
    vim.api.nvim_win_set_cursor(0, {line_count + 1, 11})
  ]])

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

-- Context-aware completion tests

T["completion"]["context: command at line start"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Type at line start (command context)
  child.cmd("normal! G")
  child.cmd("normal! o")
  child.type_keys("ech")

  local completions = H.get_completions(child)

  -- Should include Ex commands like echo
  local has_echo = vim.tbl_contains(completions, "echo")
  -- Should NOT include functions like strlen (they belong to function context)
  local has_strlen = vim.tbl_contains(completions, "strlen")

  MiniTest.expect.equality(has_echo, true, "Expected 'echo' command in completions at line start")
  MiniTest.expect.equality(has_strlen, false, "Expected NO 'strlen' function in command context")

  child.stop()
end

T["completion"]["context: function in expression"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Type in expression context (after =)
  child.cmd("normal! G")
  child.lua([[
    local line_count = vim.api.nvim_buf_line_count(0)
    vim.api.nvim_buf_set_lines(0, line_count, line_count, false, {"let x = str"})
    vim.api.nvim_win_set_cursor(0, {line_count + 1, 11})
  ]])

  local completions = H.get_completions(child)

  -- Should include functions like strlen (not Ex commands)
  local has_strlen = vim.tbl_contains(completions, "strlen")

  MiniTest.expect.equality(has_strlen, true, "Expected 'strlen' function in expression context")

  child.stop()
end

T["completion"]["context: autocmd events"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Type after autocmd
  child.cmd("normal! G")
  child.lua([[
    local line_count = vim.api.nvim_buf_line_count(0)
    vim.api.nvim_buf_set_lines(0, line_count, line_count, false, {"autocmd Buf"})
    vim.api.nvim_win_set_cursor(0, {line_count + 1, 11})
  ]])

  local completions = H.get_completions(child)

  -- Should include autocmd events
  local has_bufread = vim.tbl_contains(completions, "BufRead")
  local has_bufenter = vim.tbl_contains(completions, "BufEnter")

  MiniTest.expect.equality(has_bufread or has_bufenter, true, "Expected autocmd events in completions")

  child.stop()
end

T["completion"]["context: set options"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Type after set
  child.cmd("normal! G")
  child.lua([[
    local line_count = vim.api.nvim_buf_line_count(0)
    vim.api.nvim_buf_set_lines(0, line_count, line_count, false, {"set nu"})
    vim.api.nvim_win_set_cursor(0, {line_count + 1, 6})
  ]])

  local completions = H.get_completions(child)

  -- Should include options
  local has_number = vim.tbl_contains(completions, "number")

  MiniTest.expect.equality(has_number, true, "Expected 'number' option in completions after 'set'")

  child.stop()
end

T["completion"]["context: map options"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Use nvim_put to insert text with < directly
  child.cmd("normal! G")
  child.cmd("normal! o")
  child.lua([[vim.api.nvim_put({"nnoremap <si"}, 'c', true, true)]])

  local completions = H.get_completions(child)

  -- Should include map options
  local has_silent = vim.tbl_contains(completions, "<silent>")
  local has_buffer = vim.tbl_contains(completions, "<buffer>")

  MiniTest.expect.equality(has_silent or has_buffer, true, "Expected map options like <silent> or <buffer>")

  child.stop()
end

T["completion"]["context: has() features"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Type inside has() - use lua to insert text directly to avoid quote escaping issues
  child.cmd("normal! G")
  child.cmd("normal! o")
  child.lua([[vim.api.nvim_put({"if has('nvi"}, 'c', true, true)]])

  local completions = H.get_completions(child)

  -- Should include has() features
  local has_nvim = vim.tbl_contains(completions, "nvim")

  MiniTest.expect.equality(has_nvim, true, "Expected 'nvim' feature in completions inside has()")

  child.stop()
end

return T
