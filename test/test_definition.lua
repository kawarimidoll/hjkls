-- E2E tests for go to definition
-- Tests that hjkls correctly jumps to symbol definitions

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

T["definition"] = MiniTest.new_set()

T["definition"]["jumps to function definition in same file"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Wait for indexing
  child.lua("vim.wait(2000)")

  -- Go to line 17 where Hello is called: call Hello(a:who)
  child.cmd("normal! 17G")
  child.cmd("normal! 0f(b") -- Position on "Hello"

  local def = H.get_definition(child)

  MiniTest.expect.equality(def ~= nil, true, "Expected definition to be found")
  if def then
    -- Hello is defined on line 6 (0-indexed: 5)
    MiniTest.expect.equality(def.line, 5, "Expected definition on line 6 (0-indexed: 5)")
  end

  child.stop()
end

T["definition"]["jumps to variable definition"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Wait for indexing
  child.lua("vim.wait(2000)")

  -- Go to line 31 where g:my_var is used: echo g:my_var
  child.cmd("normal! 31G")
  child.cmd("normal! 0fg") -- Position on "g:my_var"

  local def = H.get_definition(child)

  MiniTest.expect.equality(def ~= nil, true, "Expected definition to be found for g:my_var")
  if def then
    -- g:my_var is defined on line 26 (0-indexed: 25)
    MiniTest.expect.equality(def.line, 25, "Expected definition on line 26 (0-indexed: 25)")
  end

  child.stop()
end

T["definition"]["jumps to autoload function in different file"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Wait for indexing
  child.lua("vim.wait(2000)")

  -- Go to line 47 where autoload function is called: call myplugin#util#helper()
  child.cmd("normal! 47G")
  child.cmd("normal! 0fm") -- Position on "myplugin"

  local def = H.get_definition(child)

  MiniTest.expect.equality(def ~= nil, true, "Expected definition to be found for autoload function")
  if def then
    -- Should point to autoload/myplugin/util.vim
    local is_autoload_file = def.uri:match("autoload/myplugin/util%.vim") ~= nil
    MiniTest.expect.equality(is_autoload_file, true, "Expected definition in autoload file")
  end

  child.stop()
end

T["definition"]["returns nil for builtin functions"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Go to line 38 where strlen is used: let g:length = strlen("test")
  child.cmd("normal! 38G")
  child.cmd("normal! 0fs") -- Position on "strlen"

  local def = H.get_definition(child)

  -- Builtin functions don't have a definition location
  MiniTest.expect.equality(def, nil, "Expected no definition for builtin function")

  child.stop()
end

return T
