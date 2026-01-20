-- E2E tests for signature help
-- Tests that hjkls provides parameter hints during function calls

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

T["signature_help"] = MiniTest.new_set()

T["signature_help"]["shows builtin function parameters"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Go to line 38 where strlen is used: let g:length = strlen("test")
  -- Position cursor inside the parentheses
  child.cmd("normal! 38G")
  child.cmd('normal! 0f"') -- Position on the first quote inside strlen()

  local sig = H.get_signature_help(child)

  MiniTest.expect.equality(sig ~= nil, true, "Expected signature help for strlen")
  if sig then
    local has_strlen = sig.label:match("strlen") ~= nil
    MiniTest.expect.equality(has_strlen, true, "Expected strlen in signature label")
  end

  child.stop()
end

T["signature_help"]["shows user-defined function parameters"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Wait for indexing
  child.lua("vim.wait(2000)")

  -- Go to line 17 where Hello is called: call Hello(a:who)
  -- Position cursor inside the parentheses
  child.cmd("normal! 17G")
  child.cmd("normal! 0f:") -- Position on the colon in a:who

  local sig = H.get_signature_help(child)

  MiniTest.expect.equality(sig ~= nil, true, "Expected signature help for Hello")
  if sig then
    local has_hello = sig.label:match("Hello") ~= nil
    local has_name = sig.label:match("name") ~= nil
    MiniTest.expect.equality(has_hello, true, "Expected Hello in signature label")
    MiniTest.expect.equality(has_name, true, "Expected 'name' parameter in signature")
  end

  child.stop()
end

T["signature_help"]["returns nil outside function call"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Position on a regular line, not inside a function call
  child.cmd("normal! 1G")
  child.cmd("normal! 0")

  local sig = H.get_signature_help(child)

  MiniTest.expect.equality(sig, nil, "Expected no signature help outside function call")

  child.stop()
end

return T
