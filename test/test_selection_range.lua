-- E2E tests for selection range
-- Tests that hjkls correctly reports selection ranges for smart selection

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

T["selection_range"] = MiniTest.new_set()

T["selection_range"]["returns selection chain from identifier"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Position inside Hello function body (line 7, 0-indexed: 6)
  -- line 7: echo "Hello, " . a:name
  local ranges = H.get_selection_ranges(child, { { 6, 8 } })

  -- Should return at least one selection chain
  MiniTest.expect.equality(#ranges >= 1, true, "Expected at least 1 selection chain")

  -- The chain should have multiple levels (innermost to outermost)
  local chain = ranges[1]
  MiniTest.expect.equality(#chain >= 2, true, "Expected at least 2 levels in selection chain")

  -- First level should be smallest (innermost), last should be largest (root)
  local first = chain[1]
  local last = chain[#chain]

  -- The innermost range should be smaller or equal to the outermost
  local first_lines = first.end_line - first.start_line
  local last_lines = last.end_line - last.start_line
  MiniTest.expect.equality(first_lines <= last_lines, true, "Innermost range should be <= outermost")

  child.stop()
end

T["selection_range"]["expands through function body"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Position on "echo" inside Hello function body (line 7, char 2)
  -- line 7 (0-indexed: 6): echo "Hello, " . a:name
  local ranges = H.get_selection_ranges(child, { { 6, 2 } })

  MiniTest.expect.equality(#ranges >= 1, true, "Expected at least 1 selection chain")

  local chain = ranges[1]

  -- Should eventually reach the function definition (lines 5-7 in 0-indexed)
  local found_function = false
  for _, r in ipairs(chain) do
    -- Check if this range covers the entire function (starts at line 5)
    if r.start_line == 5 and r.end_line == 7 then
      found_function = true
      break
    end
  end
  MiniTest.expect.equality(found_function, true, "Expected chain to include function definition range")

  child.stop()
end

T["selection_range"]["handles multiple positions"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Request selection ranges for two positions
  local ranges = H.get_selection_ranges(child, {
    { 6, 2 },  -- Inside Hello function body
    { 30, 7 }, -- Inside UseVariables function (echo g:my_var)
  })

  -- Should return exactly 2 selection chains
  MiniTest.expect.equality(#ranges, 2, "Expected 2 selection chains for 2 positions")

  -- Each chain should have multiple levels
  MiniTest.expect.equality(#ranges[1] >= 2, true, "First chain should have at least 2 levels")
  MiniTest.expect.equality(#ranges[2] >= 2, true, "Second chain should have at least 2 levels")

  child.stop()
end

T["selection_range"]["outermost range covers entire file"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Position anywhere in the file
  local ranges = H.get_selection_ranges(child, { { 6, 2 } })

  MiniTest.expect.equality(#ranges >= 1, true, "Expected at least 1 selection chain")

  local chain = ranges[1]
  local outermost = chain[#chain]

  -- Outermost should start at line 0 (the root node covers entire file)
  MiniTest.expect.equality(outermost.start_line, 0, "Outermost range should start at line 0")
  MiniTest.expect.equality(outermost.start_char, 0, "Outermost range should start at character 0")

  child.stop()
end

return T
