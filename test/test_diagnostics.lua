-- E2E tests for diagnostics
-- Tests that hjkls reports correct diagnostics for various code patterns

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

-- Helper: find diagnostic at specific line (0-indexed)
local function find_diagnostic_at_line(diagnostics, line)
  for _, d in ipairs(diagnostics) do
    if d.lnum == line then
      return d
    end
  end
  return nil
end

T["diagnostics"] = MiniTest.new_set()

T["diagnostics"]["detects syntax errors"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/syntax_errors.vim")
  H.wait_for_lsp(child)
  H.wait_for_diagnostics(child)

  local diagnostics = H.get_diagnostics(child)

  -- Line 5: function! Broken( - unclosed parenthesis (0-indexed: 4)
  local broken_func = find_diagnostic_at_line(diagnostics, 4)
  MiniTest.expect.equality(broken_func ~= nil, true, "Expected syntax error on line 5 (0-indexed: 4)")

  -- Line 9: if 1 without endif (0-indexed: 8)
  local missing_endif = find_diagnostic_at_line(diagnostics, 8)
  MiniTest.expect.equality(missing_endif ~= nil, true, "Expected syntax error on line 9 (0-indexed: 8)")

  child.stop()
end

T["diagnostics"]["detects argument count errors"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)
  H.wait_for_diagnostics(child)

  local diagnostics = H.get_diagnostics(child)

  -- Line 55: strlen() - too few arguments (expects 1)
  local strlen_err = find_diagnostic_at_line(diagnostics, 55)
  MiniTest.expect.equality(strlen_err ~= nil, true, "Expected error for strlen() with no arguments")

  -- Line 59: Hello("alice", "bob") - too many arguments
  local hello_err = find_diagnostic_at_line(diagnostics, 59)
  MiniTest.expect.equality(hello_err ~= nil, true, "Expected error for Hello() with too many arguments")

  -- Line 61: empty(1, 2, 3) - too many arguments
  local empty_err = find_diagnostic_at_line(diagnostics, 61)
  MiniTest.expect.equality(empty_err ~= nil, true, "Expected error for empty() with too many arguments")

  child.stop()
end

T["diagnostics"]["allows valid optional arguments"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)
  H.wait_for_diagnostics(child)

  local diagnostics = H.get_diagnostics(child)

  -- Lines 64-68: valid calls with optional arguments should NOT have errors
  -- Line 64: Hello() - valid (has default)
  -- Line 65: Hello("alice") - valid
  -- Line 66-68: search() with 1-3 args - all valid

  -- Count errors in the valid section (lines 64-68, 0-indexed: 63-67)
  local valid_section_errors = 0
  for _, d in ipairs(diagnostics) do
    if d.lnum >= 63 and d.lnum <= 67 then
      valid_section_errors = valid_section_errors + 1
    end
  end

  MiniTest.expect.equality(valid_section_errors, 0, "Expected no errors in valid optional arguments section")

  child.stop()
end

T["diagnostics"]["reports multiple errors"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)
  H.wait_for_diagnostics(child)

  local diagnostics = H.get_diagnostics(child)

  -- Should have multiple diagnostics total
  MiniTest.expect.equality(#diagnostics >= 5, true, "Expected at least 5 diagnostics in sample.vim")

  child.stop()
end

T["diagnostics"]["detects scope violations"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)
  H.wait_for_diagnostics(child)

  local diagnostics = H.get_diagnostics(child)

  -- Line 75: let l:invalid_local = 1 (0-indexed: 74)
  local local_err = find_diagnostic_at_line(diagnostics, 74)
  MiniTest.expect.equality(local_err ~= nil, true, "Expected scope violation for l: outside function")

  -- Line 76: echo a:invalid_arg (0-indexed: 75)
  local arg_err = find_diagnostic_at_line(diagnostics, 75)
  MiniTest.expect.equality(arg_err ~= nil, true, "Expected scope violation for a: outside function")

  -- Line 80-81: l:valid inside function should NOT have error (0-indexed: 79-80)
  local valid_local_79 = find_diagnostic_at_line(diagnostics, 79)
  local valid_local_80 = find_diagnostic_at_line(diagnostics, 80)
  -- These should be nil (no error for valid usage inside function)
  local has_false_positive = (valid_local_79 ~= nil and valid_local_79.message:match("scope")) or
                             (valid_local_80 ~= nil and valid_local_80.message:match("scope"))
  MiniTest.expect.equality(has_false_positive, false, "Expected no scope violation for l: inside function")

  child.stop()
end

T["diagnostics"]["detects undefined function calls"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)
  H.wait_for_diagnostics(child)

  local diagnostics = H.get_diagnostics(child)

  -- Line 87: call s:NonExistentHelper() (0-indexed: 86)
  local script_func_err = find_diagnostic_at_line(diagnostics, 86)
  MiniTest.expect.equality(script_func_err ~= nil, true, "Expected undefined function warning for s:NonExistentHelper")
  if script_func_err then
    MiniTest.expect.equality(script_func_err.message:match("Undefined function") ~= nil, true)
  end

  -- Line 90: call UndefinedGlobalFunc() (0-indexed: 89)
  local global_func_err = find_diagnostic_at_line(diagnostics, 89)
  MiniTest.expect.equality(global_func_err ~= nil, true, "Expected undefined function warning for UndefinedGlobalFunc")
  if global_func_err then
    MiniTest.expect.equality(global_func_err.message:match("Undefined function") ~= nil, true)
  end

  -- Line 93: call notabuiltin() - lowercase undefined function (0-indexed: 92)
  local lowercase_func_err = find_diagnostic_at_line(diagnostics, 92)
  MiniTest.expect.equality(lowercase_func_err ~= nil, true, "Expected undefined function warning for notabuiltin")
  if lowercase_func_err then
    MiniTest.expect.equality(lowercase_func_err.message:match("Undefined function") ~= nil, true)
  end

  -- Lines 96-97: valid user-defined functions should NOT have undefined warnings
  local valid_hello = find_diagnostic_at_line(diagnostics, 95)
  local valid_private = find_diagnostic_at_line(diagnostics, 96)
  local has_user_false_positive = (valid_hello ~= nil and valid_hello.message:match("Undefined")) or
                                  (valid_private ~= nil and valid_private.message:match("Undefined"))
  MiniTest.expect.equality(has_user_false_positive, false, "Expected no undefined warning for defined functions")

  -- Lines 100-101: built-in functions should NOT have undefined warnings
  local valid_strlen = find_diagnostic_at_line(diagnostics, 99)
  local valid_empty = find_diagnostic_at_line(diagnostics, 100)
  local has_builtin_false_positive = (valid_strlen ~= nil and valid_strlen.message:match("Undefined")) or
                                     (valid_empty ~= nil and valid_empty.message:match("Undefined"))
  MiniTest.expect.equality(has_builtin_false_positive, false, "Expected no undefined warning for built-in functions")

  child.stop()
end

T["diagnostics"]["detects suspicious normal without bang"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)
  H.wait_for_diagnostics(child)

  local diagnostics = H.get_diagnostics(child)

  -- Line 106: normal j (0-indexed: 105)
  local normal_j = find_diagnostic_at_line(diagnostics, 105)
  MiniTest.expect.equality(normal_j ~= nil, true, "Expected warning for 'normal j' without bang")
  if normal_j then
    MiniTest.expect.equality(normal_j.message:match("normal") ~= nil, true)
    MiniTest.expect.equality(normal_j.message:match("normal!") ~= nil, true)
  end

  -- Line 107: normal k (0-indexed: 106)
  local normal_k = find_diagnostic_at_line(diagnostics, 106)
  MiniTest.expect.equality(normal_k ~= nil, true, "Expected warning for 'normal k' without bang")

  -- Line 110: normal! j should NOT have warning (0-indexed: 109)
  local normal_bang = find_diagnostic_at_line(diagnostics, 109)
  local has_normal_false_positive = normal_bang ~= nil and normal_bang.message:match("normal")
  MiniTest.expect.equality(has_normal_false_positive, false, "Expected no warning for 'normal!' with bang")

  child.stop()
end

T["diagnostics"]["detects suspicious match without case modifier"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)
  H.wait_for_diagnostics(child)

  local diagnostics = H.get_diagnostics(child)

  -- Line 113: if g:my_var =~ 'pattern' (0-indexed: 112)
  local match_warn = find_diagnostic_at_line(diagnostics, 112)
  MiniTest.expect.equality(match_warn ~= nil, true, "Expected warning for '=~' without case modifier")
  if match_warn then
    MiniTest.expect.equality(match_warn.message:match("=~") ~= nil, true)
    MiniTest.expect.equality(match_warn.message:match("ignorecase") ~= nil, true)
  end

  -- Line 117: =~# should NOT have warning (0-indexed: 116)
  local match_hash = find_diagnostic_at_line(diagnostics, 116)
  local has_hash_false_positive = match_hash ~= nil and match_hash.message:match("=~")
  MiniTest.expect.equality(has_hash_false_positive, false, "Expected no warning for '=~#'")

  -- Line 119: =~? should NOT have warning (0-indexed: 118)
  local match_question = find_diagnostic_at_line(diagnostics, 118)
  local has_question_false_positive = match_question ~= nil and match_question.message:match("=~")
  MiniTest.expect.equality(has_question_false_positive, false, "Expected no warning for '=~?'")

  child.stop()
end

T["diagnostics"]["detects suspicious autocmd outside augroup"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)
  H.wait_for_diagnostics(child)

  local diagnostics = H.get_diagnostics(child)

  -- Line 123: autocmd FileType vim (0-indexed: 122)
  local autocmd_standalone = find_diagnostic_at_line(diagnostics, 122)
  MiniTest.expect.equality(autocmd_standalone ~= nil, true, "Expected warning for autocmd outside augroup (line 123)")
  if autocmd_standalone then
    MiniTest.expect.equality(autocmd_standalone.message:match("augroup") ~= nil, true)
  end

  -- Line 128: autocmd inside augroup should NOT have warning (0-indexed: 127)
  local autocmd_in_group = find_diagnostic_at_line(diagnostics, 127)
  local has_augroup_false_positive = autocmd_in_group ~= nil and autocmd_in_group.message:match("augroup") ~= nil
  MiniTest.expect.equality(has_augroup_false_positive, false, "Expected no warning for autocmd inside augroup")

  -- Line 132: autocmd with inline group should NOT have warning (0-indexed: 131)
  local autocmd_inline_group = find_diagnostic_at_line(diagnostics, 131)
  local has_inline_false_positive = autocmd_inline_group ~= nil and autocmd_inline_group.message:match("augroup") ~= nil
  MiniTest.expect.equality(has_inline_false_positive, false, "Expected no warning for autocmd with inline group")

  child.stop()
end

T["diagnostics"]["detects suspicious set compatible"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)
  H.wait_for_diagnostics(child)

  local diagnostics = H.get_diagnostics(child)

  -- Line 135: set compatible (0-indexed: 134)
  local compat = find_diagnostic_at_line(diagnostics, 134)
  MiniTest.expect.equality(compat ~= nil, true, "Expected warning for 'set compatible'")
  if compat then
    MiniTest.expect.equality(compat.message:match("Vi%-compatible") ~= nil, true)
  end

  -- Line 136: set cp (0-indexed: 135)
  local cp = find_diagnostic_at_line(diagnostics, 135)
  MiniTest.expect.equality(cp ~= nil, true, "Expected warning for 'set cp'")
  if cp then
    MiniTest.expect.equality(cp.message:match("Vi%-compatible") ~= nil, true)
  end

  child.stop()
end

T["diagnostics"]["detects vim9script not at start"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/vim9script_errors.vim")
  H.wait_for_lsp(child)
  H.wait_for_diagnostics(child)

  local diagnostics = H.get_diagnostics(child)

  -- Line 5: vim9script after other code (0-indexed: 4)
  local vim9script_warn = find_diagnostic_at_line(diagnostics, 4)
  MiniTest.expect.equality(vim9script_warn ~= nil, true, "Expected warning for vim9script not at start")
  if vim9script_warn then
    MiniTest.expect.equality(vim9script_warn.message:match("vim9script") ~= nil, true)
    MiniTest.expect.equality(vim9script_warn.message:match("first") ~= nil, true)
  end

  child.stop()
end

T["diagnostics"]["hints style double_dot"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)
  H.wait_for_diagnostics(child)

  local diagnostics = H.get_diagnostics(child)

  -- Line 139: "hello" . "world" (0-indexed: 138)
  local single_dot = find_diagnostic_at_line(diagnostics, 138)
  MiniTest.expect.equality(single_dot ~= nil, true, "Expected hint for '.' string concatenation")
  if single_dot then
    MiniTest.expect.equality(single_dot.severity, vim.diagnostic.severity.HINT)
    MiniTest.expect.equality(single_dot.message:match("%.%.") ~= nil, true, "Message should mention '..'")
    MiniTest.expect.equality(single_dot.message:match("Vim9") ~= nil, true, "Message should mention Vim9")
  end

  -- Line 140: "a" . "b" . "c" - chained (0-indexed: 139)
  local chained_dot = find_diagnostic_at_line(diagnostics, 139)
  MiniTest.expect.equality(chained_dot ~= nil, true, "Expected hint for chained '.' string concatenation")

  -- Line 143: "hello" .. "world" should NOT have hint (0-indexed: 142)
  local double_dot = find_diagnostic_at_line(diagnostics, 142)
  local has_double_dot_false_positive = double_dot ~= nil and double_dot.message:match("Style") ~= nil
  MiniTest.expect.equality(has_double_dot_false_positive, false, "Expected no hint for '..' concatenation")

  child.stop()
end

T["diagnostics"]["hints style function_bang"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)
  H.wait_for_diagnostics(child)

  local diagnostics = H.get_diagnostics(child)

  -- Line 146: function! s:ScriptLocalWithBang() (0-indexed: 145)
  local script_local_bang = find_diagnostic_at_line(diagnostics, 145)
  MiniTest.expect.equality(script_local_bang ~= nil, true, "Expected hint for 'function!' with s: scope")
  if script_local_bang then
    MiniTest.expect.equality(script_local_bang.severity, vim.diagnostic.severity.HINT)
    MiniTest.expect.equality(script_local_bang.message:match("s:") ~= nil, true, "Message should mention s:")
  end

  -- Line 151: function s:ScriptLocalNoBang() should NOT have function_bang hint (0-indexed: 150)
  -- Note: It may have abort hint, so check specifically for "unnecessary" in message
  local script_local_no_bang = find_diagnostic_at_line(diagnostics, 150)
  local has_no_bang_false_positive = script_local_no_bang ~= nil and script_local_no_bang.message:match("unnecessary") ~= nil
  MiniTest.expect.equality(has_no_bang_false_positive, false, "Expected no function_bang hint for 'function' without bang")

  -- Line 156: function! GlobalFuncWithBang() should NOT have function_bang hint (0-indexed: 155)
  -- Note: It may have abort hint, so check specifically for "unnecessary" in message
  local global_bang = find_diagnostic_at_line(diagnostics, 155)
  local has_global_false_positive = global_bang ~= nil and global_bang.message:match("unnecessary") ~= nil
  MiniTest.expect.equality(has_global_false_positive, false, "Expected no function_bang hint for global function with bang")

  child.stop()
end

T["diagnostics"]["hints style abort"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)
  H.wait_for_diagnostics(child)

  local diagnostics = H.get_diagnostics(child)

  -- Line 161: function! NoAbortFunc() (0-indexed: 160)
  local no_abort = find_diagnostic_at_line(diagnostics, 160)
  MiniTest.expect.equality(no_abort ~= nil, true, "Expected hint for function without abort")
  if no_abort then
    MiniTest.expect.equality(no_abort.severity, vim.diagnostic.severity.HINT)
    MiniTest.expect.equality(no_abort.message:match("abort") ~= nil, true, "Message should mention abort")
  end

  -- Line 166: function! HasAbortFunc() abort should NOT have hint (0-indexed: 165)
  local has_abort = find_diagnostic_at_line(diagnostics, 165)
  local has_abort_false_positive = has_abort ~= nil and has_abort.message:match("missing.*abort")
  MiniTest.expect.equality(has_abort_false_positive, false, "Expected no hint for function with abort")

  child.stop()
end

T["diagnostics"]["hints style single_quote"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)
  H.wait_for_diagnostics(child)

  local diagnostics = H.get_diagnostics(child)

  -- Line 171: "hello" (0-indexed: 170)
  local double_simple = find_diagnostic_at_line(diagnostics, 170)
  MiniTest.expect.equality(double_simple ~= nil, true, "Expected hint for double-quoted string without escapes")
  if double_simple then
    MiniTest.expect.equality(double_simple.severity, vim.diagnostic.severity.HINT)
    MiniTest.expect.equality(double_simple.message:match("single quotes") ~= nil, true, "Message should mention single quotes")
  end

  -- Line 174: "hello\nworld" should NOT have hint (0-indexed: 173)
  local double_escape = find_diagnostic_at_line(diagnostics, 173)
  local has_escape_false_positive = double_escape ~= nil and double_escape.message:match("single quotes")
  MiniTest.expect.equality(has_escape_false_positive, false, "Expected no hint for string with escape sequence")

  -- Line 177: 'hello' should NOT have hint (0-indexed: 176)
  local single_simple = find_diagnostic_at_line(diagnostics, 176)
  local has_single_false_positive = single_simple ~= nil and single_simple.message:match("single quotes")
  MiniTest.expect.equality(has_single_false_positive, false, "Expected no hint for single-quoted string")

  -- Line 180: "it's a test" should NOT have hint (0-indexed: 179)
  local double_with_quote = find_diagnostic_at_line(diagnostics, 179)
  local has_quote_false_positive = double_with_quote ~= nil and double_with_quote.message:match("single quotes")
  MiniTest.expect.equality(has_quote_false_positive, false, "Expected no hint for string containing single quote")

  child.stop()
end

T["diagnostics"]["hints style key_notation"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)
  H.wait_for_diagnostics(child)

  local diagnostics = H.get_diagnostics(child)

  -- Line 185: <cr> should be <CR> (0-indexed: 184)
  local cr_hint = find_diagnostic_at_line(diagnostics, 184)
  MiniTest.expect.equality(cr_hint ~= nil, true, "Expected hint for '<cr>' key notation")
  if cr_hint then
    MiniTest.expect.equality(cr_hint.severity, vim.diagnostic.severity.HINT)
    MiniTest.expect.equality(cr_hint.message:match("<CR>") ~= nil, true, "Message should mention <CR>")
  end

  -- Line 189: <UP> should be <Up> (0-indexed: 188)
  local up_hint = find_diagnostic_at_line(diagnostics, 188)
  MiniTest.expect.equality(up_hint ~= nil, true, "Expected hint for '<UP>' key notation")
  if up_hint then
    MiniTest.expect.equality(up_hint.message:match("<Up>") ~= nil, true, "Message should mention <Up>")
  end

  -- Line 193: <c-a> should be <C-a> (0-indexed: 192)
  local ctrl_hint = find_diagnostic_at_line(diagnostics, 192)
  MiniTest.expect.equality(ctrl_hint ~= nil, true, "Expected hint for '<c-a>' key notation")
  if ctrl_hint then
    MiniTest.expect.equality(ctrl_hint.message:match("<C%-a>") ~= nil, true, "Message should mention <C-a>")
  end

  -- Lines 196-199: correct notations should NOT have hints
  -- Line 196: <CR> (0-indexed: 195)
  local valid_cr = find_diagnostic_at_line(diagnostics, 195)
  local has_cr_false_positive = valid_cr ~= nil and valid_cr.message:match("key%-notation") ~= nil
  MiniTest.expect.equality(has_cr_false_positive, false, "Expected no hint for correct '<CR>' notation")

  -- Line 198: <Up> (0-indexed: 197)
  local valid_up = find_diagnostic_at_line(diagnostics, 197)
  local has_up_false_positive = valid_up ~= nil and valid_up.message:match("key%-notation") ~= nil
  MiniTest.expect.equality(has_up_false_positive, false, "Expected no hint for correct '<Up>' notation")

  child.stop()
end

-- ============================================================================
-- Inline ignore directive tests
-- ============================================================================

T["diagnostics"]["hjkls:ignore-next-line suppresses warning on next line"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/ignore_test.vim")
  H.wait_for_lsp(child)
  H.wait_for_diagnostics(child)

  local diagnostics = H.get_diagnostics(child)

  -- Line 4 (0-indexed: 3): normal j - should have warning (not ignored)
  local line3_warning = find_diagnostic_at_line(diagnostics, 3)
  local has_warning_line3 = line3_warning ~= nil and line3_warning.message:match("normal") ~= nil
  MiniTest.expect.equality(
    has_warning_line3,
    true,
    "Expected normal_bang warning on line 4 (not ignored)"
  )

  -- Line 7 (0-indexed: 6): normal k - should NOT have warning (ignored by hjkls:ignore-next-line on line 6)
  local line6_warning = find_diagnostic_at_line(diagnostics, 6)
  local has_normal_warning_line6 = line6_warning ~= nil and line6_warning.message:match("normal") ~= nil
  MiniTest.expect.equality(
    has_normal_warning_line6,
    false,
    "Expected no warning on line 7 (ignored by hjkls:ignore-next-line)"
  )

  -- Line 10 (0-indexed: 9): normal l - should have warning (ignore-next-line only affects line 7)
  local line9_warning = find_diagnostic_at_line(diagnostics, 9)
  local has_warning_line9 = line9_warning ~= nil and line9_warning.message:match("normal") ~= nil
  MiniTest.expect.equality(
    has_warning_line9,
    true,
    "Expected normal_bang warning on line 10 (ignore-next-line doesn't affect this)"
  )

  child.stop()
end

T["diagnostics"]["hjkls:ignore suppresses warnings to end of file"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/ignore_test.vim")
  H.wait_for_lsp(child)
  H.wait_for_diagnostics(child)

  local diagnostics = H.get_diagnostics(child)

  -- Lines 13-14 (0-indexed: 12-13): normal m, normal n - should NOT have warning
  -- (ignored by hjkls:ignore suspicious#normal_bang on line 12)
  local line12_warning = find_diagnostic_at_line(diagnostics, 12)
  local has_normal_warning_line12 = line12_warning ~= nil and line12_warning.message:match("normal") ~= nil
  MiniTest.expect.equality(
    has_normal_warning_line12,
    false,
    "Expected no normal_bang warning on line 13 (ignored to end of file)"
  )

  local line13_warning = find_diagnostic_at_line(diagnostics, 13)
  local has_normal_warning_line13 = line13_warning ~= nil and line13_warning.message:match("normal") ~= nil
  MiniTest.expect.equality(
    has_normal_warning_line13,
    false,
    "Expected no normal_bang warning on line 14 (ignored to end of file)"
  )

  child.stop()
end

T["diagnostics"]["hjkls:ignore only affects specified rule"] = function()
  local child = H.create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/ignore_test.vim")
  H.wait_for_lsp(child)
  H.wait_for_diagnostics(child)

  local diagnostics = H.get_diagnostics(child)

  -- Line 17 (0-indexed: 16): echo "hello" . "world" - should have double_dot hint
  -- (hjkls:ignore suspicious#normal_bang doesn't affect style#double_dot)
  local line16_warning = find_diagnostic_at_line(diagnostics, 16)
  local has_double_dot_line16 = line16_warning ~= nil and line16_warning.message:match("%.%.") ~= nil
  MiniTest.expect.equality(
    has_double_dot_line16,
    true,
    "Expected double_dot hint on line 17 (different rule not ignored)"
  )

  -- Line 20 (0-indexed: 19): echo "foo" . "bar" - should NOT have double_dot hint
  -- (ignored by hjkls:ignore style#double_dot on line 19)
  local line19_warning = find_diagnostic_at_line(diagnostics, 19)
  local has_double_dot_line19 = line19_warning ~= nil and line19_warning.message:match("%.%.") ~= nil
  MiniTest.expect.equality(
    has_double_dot_line19,
    false,
    "Expected no double_dot hint on line 20 (ignored by hjkls:ignore)"
  )

  child.stop()
end

return T
