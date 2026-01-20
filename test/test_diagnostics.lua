-- E2E tests for diagnostics
-- Tests that hjkls reports correct diagnostics for various code patterns

local T = MiniTest.new_set({
  hooks = {
    pre_case = function()
      -- Ensure hjkls binary exists
      local hjkls = _G.TEST_PATHS.hjkls_binary
      if vim.fn.executable(hjkls) == 0 then
        error("hjkls binary not found. Run 'cargo build' first.")
      end
    end,
  },
})

-- Helper: create a child Neovim process with hjkls configured
local function create_child()
  local child = MiniTest.new_child_neovim()
  child.start({ "-u", _G.TEST_PATHS.child_init })
  return child
end

-- Helper: wait for LSP to attach and return diagnostics
local function get_diagnostics(child, timeout_ms)
  timeout_ms = timeout_ms or 5000

  -- Wait for LSP to attach
  child.lua(string.format(
    [[
    vim.wait(%d, function()
      return #vim.lsp.get_clients({ bufnr = 0 }) > 0
    end, 100)
  ]],
    timeout_ms
  ))

  -- Wait for diagnostics to populate
  child.lua([[
    vim.wait(2000, function()
      return #vim.diagnostic.get(0) > 0
    end, 100)
  ]])

  return child.lua_get("vim.diagnostic.get(0)")
end

-- Helper: find diagnostic at specific line (0-indexed)
local function find_diagnostic_at_line(diagnostics, line)
  for _, d in ipairs(diagnostics) do
    if d.lnum == line then
      return d
    end
  end
  return nil
end

-- Helper: count diagnostics at specific line
local function count_diagnostics_at_line(diagnostics, line)
  local count = 0
  for _, d in ipairs(diagnostics) do
    if d.lnum == line then
      count = count + 1
    end
  end
  return count
end

T["diagnostics"] = MiniTest.new_set()

T["diagnostics"]["detects syntax errors"] = function()
  local child = create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")

  local diagnostics = get_diagnostics(child)

  -- Line 73: function! Broken( - unclosed parenthesis
  local broken_func = find_diagnostic_at_line(diagnostics, 73)
  MiniTest.expect.equality(broken_func ~= nil, true, "Expected syntax error on line 74 (0-indexed: 73)")

  -- Line 77: if 1 without endif
  local missing_endif = find_diagnostic_at_line(diagnostics, 77)
  MiniTest.expect.equality(missing_endif ~= nil, true, "Expected syntax error on line 78 (0-indexed: 77)")

  child.stop()
end

T["diagnostics"]["detects argument count errors"] = function()
  local child = create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")

  local diagnostics = get_diagnostics(child)

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
  local child = create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")

  local diagnostics = get_diagnostics(child)

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
  local child = create_child()
  child.cmd("edit " .. _G.TEST_PATHS.fixtures_dir .. "/sample.vim")

  local diagnostics = get_diagnostics(child)

  -- Should have multiple diagnostics total
  MiniTest.expect.equality(#diagnostics >= 5, true, "Expected at least 5 diagnostics in sample.vim")

  child.stop()
end

return T
