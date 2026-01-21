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
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")
  H.wait_for_lsp(child)
  H.wait_for_diagnostics(child)

  local diagnostics = H.get_diagnostics(child)

  -- Line 107: function! Broken( - unclosed parenthesis (0-indexed: 106)
  local broken_func = find_diagnostic_at_line(diagnostics, 106)
  MiniTest.expect.equality(broken_func ~= nil, true, "Expected syntax error on line 107 (0-indexed: 106)")

  -- Line 111: if 1 without endif (0-indexed: 110)
  local missing_endif = find_diagnostic_at_line(diagnostics, 110)
  MiniTest.expect.equality(missing_endif ~= nil, true, "Expected syntax error on line 111 (0-indexed: 110)")

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

return T
