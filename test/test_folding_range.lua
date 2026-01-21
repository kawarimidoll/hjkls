-- E2E tests for folding range
-- Tests that hjkls correctly reports foldable regions in Vim script

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

T["folding_range"] = MiniTest.new_set()

T["folding_range"]["finds function folds"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  local ranges = H.get_folding_ranges(child)

  -- Should find at least 4 function folds in sample.vim
  -- Hello (6-8), s:PrivateHelper (10-12), Greet (14-22), UseVariables (29-33)
  MiniTest.expect.equality(#ranges >= 4, true, "Expected at least 4 foldable regions")

  -- Check that Hello function fold exists (0-indexed: lines 5-7)
  local found_hello = false
  for _, r in ipairs(ranges) do
    if r.start_line == 5 and r.end_line == 7 then
      found_hello = true
      break
    end
  end
  MiniTest.expect.equality(found_hello, true, "Expected fold for Hello function (lines 6-8)")

  child.stop()
end

T["folding_range"]["finds if statement folds"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  local ranges = H.get_folding_ranges(child)

  -- Check that if statement fold exists (0-indexed: lines 38-40)
  local found_if = false
  for _, r in ipairs(ranges) do
    if r.start_line == 38 and r.end_line == 40 then
      found_if = true
      break
    end
  end
  MiniTest.expect.equality(found_if, true, "Expected fold for if statement (lines 39-41)")

  child.stop()
end

T["folding_range"]["all folds have region kind"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  local ranges = H.get_folding_ranges(child)

  -- All folds should have "region" kind
  for _, r in ipairs(ranges) do
    MiniTest.expect.equality(r.kind, "region", "Expected fold kind to be 'region'")
  end

  child.stop()
end

return T
