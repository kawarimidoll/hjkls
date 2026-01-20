-- E2E tests for hover
-- Tests that hjkls provides correct hover information

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

T["hover"] = MiniTest.new_set()

T["hover"]["shows builtin function signature"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Go to line 38 where strlen is used
  child.cmd("normal! 38G")
  child.cmd("normal! 0fs") -- Position on "strlen"

  local hover = H.get_hover(child)

  MiniTest.expect.equality(hover ~= nil, true, "Expected hover content for strlen")
  if hover then
    local has_signature = hover:match("strlen") ~= nil
    MiniTest.expect.equality(has_signature, true, "Expected strlen in hover content")
  end

  child.stop()
end

T["hover"]["shows user-defined function signature"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Wait for indexing
  child.lua("vim.wait(2000)")

  -- Go to line 17 where Hello is called
  child.cmd("normal! 17G")
  child.cmd("normal! 0f(b") -- Position on "Hello"

  local hover = H.get_hover(child)

  MiniTest.expect.equality(hover ~= nil, true, "Expected hover content for Hello function")
  if hover then
    -- Should show the function signature with parameter
    local has_name = hover:match("Hello") ~= nil
    MiniTest.expect.equality(has_name, true, "Expected Hello in hover content")
  end

  child.stop()
end

T["hover"]["shows autoload function info"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Wait for indexing
  child.lua("vim.wait(2000)")

  -- Go to line 47 where autoload function is called
  child.cmd("normal! 47G")
  child.cmd("normal! 0fm") -- Position on "myplugin"

  local hover = H.get_hover(child)

  MiniTest.expect.equality(hover ~= nil, true, "Expected hover content for autoload function")
  if hover then
    -- Should show the autoload function info
    local has_info = hover:match("myplugin") ~= nil or hover:match("helper") ~= nil
    MiniTest.expect.equality(has_info, true, "Expected autoload info in hover content")
  end

  child.stop()
end

return T
