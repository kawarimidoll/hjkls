-- Shared test helpers for E2E tests

local M = {}

--- Normalize vim.NIL to actual nil
---@param value any
---@return any
local function normalize(value)
  if value == vim.NIL then
    return nil
  end
  return value
end

--- Create a child Neovim process with hjkls configured
function M.create_child()
  local child = MiniTest.new_child_neovim()
  child.start({ "-u", _G.TEST_PATHS.child_init })
  return child
end

--- Wait for LSP to attach to the current buffer
---@param child table MiniTest child process
---@param timeout_ms number? Timeout in milliseconds (default: 5000)
function M.wait_for_lsp(child, timeout_ms)
  timeout_ms = timeout_ms or 5000
  child.lua("vim.wait(" .. timeout_ms .. ", function() return #vim.lsp.get_clients({ bufnr = 0 }) > 0 end, 100)")
end

--- Wait for diagnostics to populate
---@param child table MiniTest child process
---@param timeout_ms number? Timeout in milliseconds (default: 2000)
function M.wait_for_diagnostics(child, timeout_ms)
  timeout_ms = timeout_ms or 2000
  child.lua("vim.wait(" .. timeout_ms .. ", function() return #vim.diagnostic.get(0) > 0 end, 100)")
end

--- Get diagnostics for the current buffer
---@param child table MiniTest child process
---@return table Diagnostics list
function M.get_diagnostics(child)
  return child.lua_get("vim.diagnostic.get(0)")
end

--- Get completion items at current position
---@param child table MiniTest child process
---@return table List of completion item labels
function M.get_completions(child)
  -- Store result in global to avoid multiline lua_get issues
  child.lua([[
    local params = vim.lsp.util.make_position_params()
    local results = vim.lsp.buf_request_sync(0, 'textDocument/completion', params, 3000)
    _G._test_result = {}
    if results and results[1] and results[1].result then
      local items = results[1].result.items or results[1].result
      for _, item in ipairs(items) do
        table.insert(_G._test_result, item.label)
      end
    end
  ]])
  return child.lua_get("_G._test_result")
end

--- Get definition location at current cursor position
---@param child table MiniTest child process
---@return table|nil Definition location {uri, line} or nil
function M.get_definition(child)
  child.lua([[
    local params = vim.lsp.util.make_position_params()
    local results = vim.lsp.buf_request_sync(0, 'textDocument/definition', params, 3000)
    _G._test_result = nil
    if results and results[1] and results[1].result then
      local result = results[1].result
      -- Handle both Scalar (single Location) and Array responses
      local loc = result
      if vim.islist(result) and #result > 0 then
        loc = result[1]
      end
      if loc and (loc.uri or loc.targetUri) then
        _G._test_result = {
          uri = loc.uri or loc.targetUri,
          line = (loc.range or loc.targetRange).start.line,
        }
      end
    end
  ]])
  return normalize(child.lua_get("_G._test_result"))
end

--- Get hover content at current cursor position
---@param child table MiniTest child process
---@return string|nil Hover content or nil
function M.get_hover(child)
  child.lua([[
    local params = vim.lsp.util.make_position_params()
    local results = vim.lsp.buf_request_sync(0, 'textDocument/hover', params, 3000)
    _G._test_result = nil
    if results and results[1] and results[1].result then
      local hover = results[1].result
      if hover and hover.contents then
        if type(hover.contents) == "string" then
          _G._test_result = hover.contents
        elseif hover.contents.value then
          _G._test_result = hover.contents.value
        end
      end
    end
  ]])
  return normalize(child.lua_get("_G._test_result"))
end

--- Get references at current cursor position
---@param child table MiniTest child process
---@return table List of reference locations
function M.get_references(child)
  child.lua([[
    local params = vim.lsp.util.make_position_params()
    params.context = { includeDeclaration = true }
    local results = vim.lsp.buf_request_sync(0, 'textDocument/references', params, 3000)
    _G._test_result = {}
    if results and results[1] and results[1].result then
      for _, ref in ipairs(results[1].result) do
        table.insert(_G._test_result, { uri = ref.uri, line = ref.range.start.line })
      end
    end
  ]])
  return child.lua_get("_G._test_result")
end

--- Get document symbols
---@param child table MiniTest child process
---@return table List of symbol names
function M.get_document_symbols(child)
  child.lua([[
    local params = { textDocument = vim.lsp.util.make_text_document_params() }
    local results = vim.lsp.buf_request_sync(0, 'textDocument/documentSymbol', params, 3000)
    _G._test_result = {}
    if results and results[1] and results[1].result then
      for _, sym in ipairs(results[1].result) do
        table.insert(_G._test_result, sym.name)
      end
    end
  ]])
  return child.lua_get("_G._test_result")
end

--- Get workspace symbols
---@param child table MiniTest child process
---@param query string Search query
---@return table List of symbol names
function M.get_workspace_symbols(child, query)
  child.lua('_G._test_query = "' .. (query or "") .. '"')
  child.lua([[
    local params = { query = _G._test_query }
    local results = vim.lsp.buf_request_sync(0, 'workspace/symbol', params, 5000)
    _G._test_result = {}
    if results and results[1] and results[1].result then
      for _, sym in ipairs(results[1].result) do
        table.insert(_G._test_result, sym.name)
      end
    end
  ]])
  return child.lua_get("_G._test_result")
end

--- Get document highlights at current cursor position
---@param child table MiniTest child process
---@return table List of highlight info {line, kind}
function M.get_document_highlights(child)
  child.lua([[
    local params = vim.lsp.util.make_position_params()
    local results = vim.lsp.buf_request_sync(0, 'textDocument/documentHighlight', params, 3000)
    _G._test_result = {}
    if results and results[1] and results[1].result then
      for _, hl in ipairs(results[1].result) do
        table.insert(_G._test_result, { line = hl.range.start.line, kind = hl.kind })
      end
    end
  ]])
  return child.lua_get("_G._test_result")
end

--- Get signature help at current position
---@param child table MiniTest child process
---@return table|nil Signature help {label, active_parameter, parameters} or nil
function M.get_signature_help(child)
  child.lua([[
    local params = vim.lsp.util.make_position_params()
    local results = vim.lsp.buf_request_sync(0, 'textDocument/signatureHelp', params, 3000)
    _G._test_result = nil
    if results and results[1] and results[1].result then
      local sig = results[1].result
      if sig and sig.signatures and #sig.signatures > 0 then
        _G._test_result = {
          label = sig.signatures[1].label,
          active_parameter = sig.activeParameter,
        }
      end
    end
  ]])
  return normalize(child.lua_get("_G._test_result"))
end

return M
