-- E2E tests for document highlight
-- Tests that hjkls correctly highlights all references to a symbol in the same file

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

T["document_highlight"] = MiniTest.new_set()

T["document_highlight"]["highlights function definition and calls"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Position on Hello function definition (line 6)
  child.cmd("normal! 6G")
  child.cmd("normal! 0f!w") -- Position on "Hello"

  local highlights = H.get_document_highlights(child)

  -- Should find at least 2 highlights (definition + calls)
  MiniTest.expect.equality(#highlights >= 2, true, "Expected at least 2 highlights for Hello function")

  -- Check that we have at least one WRITE (definition) highlight
  local has_write = false
  for _, hl in ipairs(highlights) do
    -- DocumentHighlightKind.Write = 3, Read = 2, Text = 1
    if hl.kind == 3 then
      has_write = true
      break
    end
  end
  MiniTest.expect.equality(has_write, true, "Expected at least one WRITE highlight for function definition")

  child.stop()
end

T["document_highlight"]["highlights variable definition and usages"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)

  -- Position on g:my_var definition (line 26)
  child.cmd("normal! 26G")
  child.cmd("normal! 0fg") -- Position on "g:my_var"

  local highlights = H.get_document_highlights(child)

  -- Should find at least 2 highlights (definition + usages)
  MiniTest.expect.equality(#highlights >= 2, true, "Expected at least 2 highlights for g:my_var")

  child.stop()
end

return T
