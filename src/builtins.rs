//! Vim script built-in functions database

/// Function availability in Vim/Neovim
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Availability {
    /// Available in both Vim and Neovim
    Common,
    /// Vim only (e.g., ch_*, job_*, popup_*, term_*)
    VimOnly,
    /// Neovim only (e.g., nvim_*, api_info, stdpath)
    NeovimOnly,
}

/// Editor mode for filtering completions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditorMode {
    /// Show all functions (default)
    #[default]
    Both,
    /// Show only Vim-compatible functions
    VimOnly,
    /// Show only Neovim-compatible functions
    NeovimOnly,
}

impl Availability {
    /// Get label suffix for completion items
    pub fn label_suffix(&self) -> &'static str {
        match self {
            Availability::Common => "",
            Availability::VimOnly => " [Vim only]",
            Availability::NeovimOnly => " [Neovim only]",
        }
    }

    /// Check if this availability is compatible with the given editor mode
    pub fn is_compatible(&self, mode: EditorMode) -> bool {
        match (mode, self) {
            (EditorMode::Both, _) => true,
            (EditorMode::VimOnly, Availability::NeovimOnly) => false,
            (EditorMode::NeovimOnly, Availability::VimOnly) => false,
            _ => true,
        }
    }
}

/// Information about a built-in function
pub struct BuiltinFunction {
    pub name: &'static str,
    pub signature: &'static str,
    pub description: &'static str,
    pub availability: Availability,
}

/// List of Vim built-in functions
/// Reference: :help function-list
pub static BUILTIN_FUNCTIONS: &[BuiltinFunction] = &[
    // String functions
    BuiltinFunction {
        name: "strlen",
        signature: "strlen({string})",
        description: "Return the number of bytes in {string}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "strchars",
        signature: "strchars({string} [, {skipcc}])",
        description: "Return the number of characters in {string}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "strwidth",
        signature: "strwidth({string})",
        description: "Return the display width of {string}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "strdisplaywidth",
        signature: "strdisplaywidth({string} [, {col}])",
        description: "Return the display width of {string} starting at {col}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "substitute",
        signature: "substitute({string}, {pat}, {sub}, {flags})",
        description: "Replace {pat} with {sub} in {string}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "submatch",
        signature: "submatch({nr} [, {list}])",
        description: "Return a specific match in substitute",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "strpart",
        signature: "strpart({string}, {start} [, {len} [, {chars}]])",
        description: "Return part of a string",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "stridx",
        signature: "stridx({haystack}, {needle} [, {start}])",
        description: "Return index of {needle} in {haystack}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "strridx",
        signature: "strridx({haystack}, {needle} [, {start}])",
        description: "Return last index of {needle} in {haystack}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "split",
        signature: "split({string} [, {pattern} [, {keepempty}]])",
        description: "Split {string} into a List",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "join",
        signature: "join({list} [, {sep}])",
        description: "Join {list} items into a string",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "trim",
        signature: "trim({string} [, {mask} [, {dir}]])",
        description: "Remove characters from {string}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "tolower",
        signature: "tolower({string})",
        description: "Convert {string} to lowercase",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "toupper",
        signature: "toupper({string})",
        description: "Convert {string} to uppercase",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "tr",
        signature: "tr({string}, {fromstr}, {tostr})",
        description: "Translate characters in {string}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "printf",
        signature: "printf({fmt}, {expr1}...)",
        description: "Format a string like sprintf()",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "escape",
        signature: "escape({string}, {chars})",
        description: "Escape {chars} in {string} with backslash",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "shellescape",
        signature: "shellescape({string} [, {special}])",
        description: "Escape {string} for use as shell argument",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "fnameescape",
        signature: "fnameescape({string})",
        description: "Escape {string} for use as file name",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "match",
        signature: "match({string}, {pattern} [, {start} [, {count}]])",
        description: "Return index of {pattern} match in {string}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "matchend",
        signature: "matchend({string}, {pattern} [, {start} [, {count}]])",
        description: "Return end index of {pattern} match",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "matchstr",
        signature: "matchstr({string}, {pattern} [, {start} [, {count}]])",
        description: "Return matched string",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "matchlist",
        signature: "matchlist({string}, {pattern} [, {start} [, {count}]])",
        description: "Return match and submatches as List",
        availability: Availability::Common,
    },
    // List functions
    BuiltinFunction {
        name: "len",
        signature: "len({expr})",
        description: "Return the length of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "empty",
        signature: "empty({expr})",
        description: "Return TRUE if {expr} is empty",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "get",
        signature: "get({list}, {idx} [, {default}])",
        description: "Get item {idx} from {list}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "add",
        signature: "add({list}, {expr})",
        description: "Append {expr} to {list}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "insert",
        signature: "insert({list}, {item} [, {idx}])",
        description: "Insert {item} into {list}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "remove",
        signature: "remove({list}, {idx} [, {end}])",
        description: "Remove items from {list}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "copy",
        signature: "copy({expr})",
        description: "Make a shallow copy of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "deepcopy",
        signature: "deepcopy({expr} [, {noref}])",
        description: "Make a deep copy of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "extend",
        signature: "extend({list1}, {list2} [, {idx}])",
        description: "Append {list2} to {list1}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "filter",
        signature: "filter({expr}, {func})",
        description: "Filter items in {expr} using {func}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "map",
        signature: "map({expr}, {func})",
        description: "Transform items in {expr} using {func}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "sort",
        signature: "sort({list} [, {func} [, {dict}]])",
        description: "Sort {list} in-place",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "reverse",
        signature: "reverse({list})",
        description: "Reverse {list} in-place",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "uniq",
        signature: "uniq({list} [, {func} [, {dict}]])",
        description: "Remove duplicate adjacent items",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "index",
        signature: "index({list}, {expr} [, {start} [, {ic}]])",
        description: "Return index of {expr} in {list}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "count",
        signature: "count({list}, {expr} [, {ic} [, {max}]])",
        description: "Count occurrences of {expr} in {list}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "range",
        signature: "range({expr} [, {max} [, {stride}]])",
        description: "Return a List of numbers",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "repeat",
        signature: "repeat({expr}, {count})",
        description: "Repeat {expr} {count} times",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "flatten",
        signature: "flatten({list} [, {maxdepth}])",
        description: "Flatten nested lists",
        availability: Availability::Common,
    },
    // Dictionary functions
    BuiltinFunction {
        name: "keys",
        signature: "keys({dict})",
        description: "Return List of keys in {dict}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "values",
        signature: "values({dict})",
        description: "Return List of values in {dict}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "items",
        signature: "items({dict})",
        description: "Return List of [key, value] pairs",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "has_key",
        signature: "has_key({dict}, {key})",
        description: "Return TRUE if {dict} has {key}",
        availability: Availability::Common,
    },
    // Type checking
    BuiltinFunction {
        name: "type",
        signature: "type({expr})",
        description: "Return the type of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "typename",
        signature: "typename({expr})",
        description: "Return the type name of {expr}",
        availability: Availability::Common,
    },
    // Buffer/Window/Tab functions
    BuiltinFunction {
        name: "bufnr",
        signature: "bufnr([{expr} [, {create}]])",
        description: "Return buffer number",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "bufname",
        signature: "bufname([{expr}])",
        description: "Return buffer name",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "bufexists",
        signature: "bufexists({expr})",
        description: "Return TRUE if buffer exists",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "buflisted",
        signature: "buflisted({expr})",
        description: "Return TRUE if buffer is listed",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "bufloaded",
        signature: "bufloaded({expr})",
        description: "Return TRUE if buffer is loaded",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getbufline",
        signature: "getbufline({buf}, {lnum} [, {end}])",
        description: "Return lines from buffer",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setbufline",
        signature: "setbufline({buf}, {lnum}, {text})",
        description: "Set lines in buffer",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "appendbufline",
        signature: "appendbufline({buf}, {lnum}, {text})",
        description: "Append lines to buffer",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "deletebufline",
        signature: "deletebufline({buf}, {first} [, {last}])",
        description: "Delete lines from buffer",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "winnr",
        signature: "winnr([{arg}])",
        description: "Return window number",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "winbufnr",
        signature: "winbufnr({nr})",
        description: "Return buffer number of window {nr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "tabpagenr",
        signature: "tabpagenr([{arg}])",
        description: "Return tab page number",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "tabpagebuflist",
        signature: "tabpagebuflist([{arg}])",
        description: "Return List of buffer numbers in tab",
        availability: Availability::Common,
    },
    // Cursor/Position functions
    BuiltinFunction {
        name: "line",
        signature: "line({expr} [, {winid}])",
        description: "Return line number of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "col",
        signature: "col({expr} [, {winid}])",
        description: "Return column number of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "virtcol",
        signature: "virtcol({expr} [, {list} [, {winid}]])",
        description: "Return screen column of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getpos",
        signature: "getpos({expr})",
        description: "Return position of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setpos",
        signature: "setpos({expr}, {list})",
        description: "Set position of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "cursor",
        signature: "cursor({lnum}, {col} [, {off}])",
        description: "Move cursor to position",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getcurpos",
        signature: "getcurpos([{winnr}])",
        description: "Return cursor position",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getline",
        signature: "getline({lnum} [, {end}])",
        description: "Return line(s) from current buffer",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setline",
        signature: "setline({lnum}, {text})",
        description: "Set line {lnum} to {text}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "append",
        signature: "append({lnum}, {text})",
        description: "Append {text} after line {lnum}",
        availability: Availability::Common,
    },
    // Search functions
    BuiltinFunction {
        name: "search",
        signature: "search({pattern} [, {flags} [, {stopline} [, {timeout} [, {skip}]]]])",
        description: "Search for {pattern}, return line number of match",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "searchpos",
        signature: "searchpos({pattern} [, {flags} [, {stopline} [, {timeout} [, {skip}]]]])",
        description: "Search for {pattern}, return [lnum, col] of match",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "searchpair",
        signature: "searchpair({start}, {middle}, {end} [, {flags} [, {skip} [, {stopline} [, {timeout}]]]])",
        description: "Search for matching pair of start/end patterns",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "searchpairpos",
        signature: "searchpairpos({start}, {middle}, {end} [, {flags} [, {skip} [, {stopline} [, {timeout}]]]])",
        description: "Search for matching pair, return [lnum, col]",
        availability: Availability::Common,
    },
    // File functions
    BuiltinFunction {
        name: "expand",
        signature: "expand({string} [, {nosuf} [, {list}]])",
        description: "Expand wildcards and special keywords",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "glob",
        signature: "glob({expr} [, {nosuf} [, {list} [, {alllinks}]]])",
        description: "Expand file wildcards",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "globpath",
        signature: "globpath({path}, {expr} [, {nosuf} [, {list} [, {alllinks}]]])",
        description: "Expand file wildcards in {path}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "filereadable",
        signature: "filereadable({file})",
        description: "Return TRUE if {file} is readable",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "filewritable",
        signature: "filewritable({file})",
        description: "Return TRUE if {file} is writable",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "isdirectory",
        signature: "isdirectory({directory})",
        description: "Return TRUE if {directory} is a directory",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "fnamemodify",
        signature: "fnamemodify({fname}, {mods})",
        description: "Modify file name according to {mods}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "readfile",
        signature: "readfile({fname} [, {type} [, {max}]])",
        description: "Read file into a List",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "writefile",
        signature: "writefile({list}, {fname} [, {flags}])",
        description: "Write List to file",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "delete",
        signature: "delete({fname} [, {flags}])",
        description: "Delete file or directory",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "rename",
        signature: "rename({from}, {to})",
        description: "Rename file",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "mkdir",
        signature: "mkdir({name} [, {path} [, {prot}]])",
        description: "Create directory",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getcwd",
        signature: "getcwd([{winnr} [, {tabnr}]])",
        description: "Return current working directory",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "chdir",
        signature: "chdir({dir})",
        description: "Change current directory",
        availability: Availability::Common,
    },
    // System functions
    BuiltinFunction {
        name: "system",
        signature: "system({cmd} [, {input}])",
        description: "Execute shell command and return output",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "systemlist",
        signature: "systemlist({cmd} [, {input} [, {keepempty}]])",
        description: "Execute shell command and return List",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "executable",
        signature: "executable({expr})",
        description: "Return TRUE if {expr} is executable",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "exepath",
        signature: "exepath({expr})",
        description: "Return full path to executable",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "environ",
        signature: "environ()",
        description: "Return Dict of environment variables",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getenv",
        signature: "getenv({name})",
        description: "Return environment variable value",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setenv",
        signature: "setenv({name}, {val})",
        description: "Set environment variable",
        availability: Availability::Common,
    },
    // Misc functions
    BuiltinFunction {
        name: "exists",
        signature: "exists({expr})",
        description: "Return TRUE if {expr} exists",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "has",
        signature: "has({feature} [, {check}])",
        description: "Return TRUE if feature is supported",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "eval",
        signature: "eval({string})",
        description: "Evaluate {string} as expression",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "execute",
        signature: "execute({command} [, {silent}])",
        description: "Execute Ex command and return output",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "input",
        signature: "input({prompt} [, {text} [, {completion}]])",
        description: "Get input from user",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "confirm",
        signature: "confirm({msg} [, {choices} [, {default} [, {type}]]])",
        description: "Show confirmation dialog",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "feedkeys",
        signature: "feedkeys({string} [, {mode}])",
        description: "Add keys to input buffer",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "mode",
        signature: "mode([{expr}])",
        description: "Return current mode",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "visualmode",
        signature: "visualmode([{expr}])",
        description: "Return last visual mode",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "echo",
        signature: "echo {expr1} ..",
        description: "Echo expressions",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "echomsg",
        signature: "echomsg {expr1} ..",
        description: "Echo as message",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "echoerr",
        signature: "echoerr {expr1} ..",
        description: "Echo as error message",
        availability: Availability::Common,
    },
    // Function-related
    BuiltinFunction {
        name: "call",
        signature: "call({func}, {arglist} [, {dict}])",
        description: "Call {func} with arguments from {arglist}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "function",
        signature: "function({name} [, {arglist}] [, {dict}])",
        description: "Return Funcref to function {name}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "funcref",
        signature: "funcref({name} [, {arglist}] [, {dict}])",
        description: "Return Funcref like function()",
        availability: Availability::Common,
    },
    // JSON
    BuiltinFunction {
        name: "json_encode",
        signature: "json_encode({expr})",
        description: "Encode {expr} as JSON",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "json_decode",
        signature: "json_decode({string})",
        description: "Decode JSON {string}",
        availability: Availability::Common,
    },
    // Timer
    BuiltinFunction {
        name: "timer_start",
        signature: "timer_start({time}, {callback} [, {options}])",
        description: "Create a timer",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "timer_stop",
        signature: "timer_stop({timer})",
        description: "Stop a timer",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "timer_stopall",
        signature: "timer_stopall()",
        description: "Stop all timers",
        availability: Availability::Common,
    },
    // Math functions
    BuiltinFunction {
        name: "abs",
        signature: "abs({expr})",
        description: "Return absolute value of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "acos",
        signature: "acos({expr})",
        description: "Return arc cosine of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "asin",
        signature: "asin({expr})",
        description: "Return arc sine of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "atan",
        signature: "atan({expr})",
        description: "Return arc tangent of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "atan2",
        signature: "atan2({expr1}, {expr2})",
        description: "Return arc tangent of {expr1}/{expr2}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "ceil",
        signature: "ceil({expr})",
        description: "Return smallest integer >= {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "cos",
        signature: "cos({expr})",
        description: "Return cosine of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "cosh",
        signature: "cosh({expr})",
        description: "Return hyperbolic cosine of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "exp",
        signature: "exp({expr})",
        description: "Return e to the power of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "floor",
        signature: "floor({expr})",
        description: "Return largest integer <= {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "fmod",
        signature: "fmod({expr1}, {expr2})",
        description: "Return remainder of {expr1}/{expr2}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "log",
        signature: "log({expr})",
        description: "Return natural logarithm of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "log10",
        signature: "log10({expr})",
        description: "Return base-10 logarithm of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "pow",
        signature: "pow({x}, {y})",
        description: "Return {x} to the power of {y}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "round",
        signature: "round({expr})",
        description: "Return {expr} rounded to nearest integer",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "sin",
        signature: "sin({expr})",
        description: "Return sine of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "sinh",
        signature: "sinh({expr})",
        description: "Return hyperbolic sine of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "sqrt",
        signature: "sqrt({expr})",
        description: "Return square root of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "tan",
        signature: "tan({expr})",
        description: "Return tangent of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "tanh",
        signature: "tanh({expr})",
        description: "Return hyperbolic tangent of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "trunc",
        signature: "trunc({expr})",
        description: "Return integer part of {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "float2nr",
        signature: "float2nr({expr})",
        description: "Convert Float to Number",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "str2float",
        signature: "str2float({string})",
        description: "Convert String to Float",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "str2nr",
        signature: "str2nr({string} [, {base}])",
        description: "Convert String to Number",
        availability: Availability::Common,
    },
    // Character/byte conversion
    BuiltinFunction {
        name: "char2nr",
        signature: "char2nr({string} [, {utf8}])",
        description: "Return number value of first char in {string}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "nr2char",
        signature: "nr2char({expr} [, {utf8}])",
        description: "Return character with number value {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "byteidx",
        signature: "byteidx({expr}, {nr} [, {utf16}])",
        description: "Return byte index of {nr}th char in {expr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "byteidxcomp",
        signature: "byteidxcomp({expr}, {nr} [, {utf16}])",
        description: "Like byteidx() but count composing chars",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "charidx",
        signature: "charidx({string}, {idx} [, {countcc} [, {utf16}]])",
        description: "Return char index of byte {idx} in {string}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "strgetchar",
        signature: "strgetchar({str}, {index})",
        description: "Return char at {index} in {str}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "strcharpart",
        signature: "strcharpart({str}, {start} [, {len} [, {skipcc}]])",
        description: "Return part of {str} by char index",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "strcharlen",
        signature: "strcharlen({string})",
        description: "Return number of chars in {string}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "str2list",
        signature: "str2list({string} [, {utf8}])",
        description: "Return List of character numbers",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "list2str",
        signature: "list2str({list} [, {utf8}])",
        description: "Return String from List of numbers",
        availability: Availability::Common,
    },
    // Window functions
    BuiltinFunction {
        name: "winheight",
        signature: "winheight({nr})",
        description: "Return height of window {nr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "winwidth",
        signature: "winwidth({nr})",
        description: "Return width of window {nr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "winline",
        signature: "winline()",
        description: "Return window line of cursor",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "wincol",
        signature: "wincol()",
        description: "Return window column of cursor",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "winsaveview",
        signature: "winsaveview()",
        description: "Return Dict with current window view",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "winrestview",
        signature: "winrestview({dict})",
        description: "Restore window view from {dict}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "win_getid",
        signature: "win_getid([{win} [, {tab}]])",
        description: "Return window ID",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "win_gotoid",
        signature: "win_gotoid({id})",
        description: "Go to window with {id}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "win_id2win",
        signature: "win_id2win({id})",
        description: "Return window number of {id}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "win_id2tabwin",
        signature: "win_id2tabwin({id})",
        description: "Return [tabnr, winnr] of {id}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "win_findbuf",
        signature: "win_findbuf({bufnr})",
        description: "Return window IDs for {bufnr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "win_gettype",
        signature: "win_gettype([{nr}])",
        description: "Return type of window {nr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "win_screenpos",
        signature: "win_screenpos({nr})",
        description: "Return screen position of window {nr}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "win_execute",
        signature: "win_execute({id}, {command} [, {silent}])",
        description: "Execute {command} in window {id}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "win_splitmove",
        signature: "win_splitmove({nr}, {target} [, {options}])",
        description: "Move window {nr} to split of {target}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "winlayout",
        signature: "winlayout([{tabnr}])",
        description: "Return layout of windows in tab",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "winrestcmd",
        signature: "winrestcmd()",
        description: "Return command to restore window sizes",
        availability: Availability::Common,
    },
    // Buffer info functions
    BuiltinFunction {
        name: "getbufinfo",
        signature: "getbufinfo([{buf}])",
        description: "Return List of buffer information",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getbufvar",
        signature: "getbufvar({buf}, {varname} [, {def}])",
        description: "Return variable from buffer",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setbufvar",
        signature: "setbufvar({buf}, {varname}, {val})",
        description: "Set variable in buffer",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "bufadd",
        signature: "bufadd({name})",
        description: "Add buffer {name} to buffer list",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "bufload",
        signature: "bufload({buf})",
        description: "Load buffer {buf} if not loaded",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getwininfo",
        signature: "getwininfo([{winid}])",
        description: "Return List of window information",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getwinvar",
        signature: "getwinvar({winnr}, {varname} [, {def}])",
        description: "Return variable from window",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setwinvar",
        signature: "setwinvar({winnr}, {varname}, {val})",
        description: "Set variable in window",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "gettabinfo",
        signature: "gettabinfo([{tabnr}])",
        description: "Return List of tab page information",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "gettabvar",
        signature: "gettabvar({tabnr}, {varname} [, {def}])",
        description: "Return variable from tab page",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "settabvar",
        signature: "settabvar({tabnr}, {varname}, {val})",
        description: "Set variable in tab page",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "gettabwinvar",
        signature: "gettabwinvar({tabnr}, {winnr}, {varname} [, {def}])",
        description: "Return variable from window in tab",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "settabwinvar",
        signature: "settabwinvar({tabnr}, {winnr}, {varname}, {val})",
        description: "Set variable in window of tab",
        availability: Availability::Common,
    },
    // Time functions
    BuiltinFunction {
        name: "localtime",
        signature: "localtime()",
        description: "Return current time in seconds",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "strftime",
        signature: "strftime({format} [, {time}])",
        description: "Format time as string",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "strptime",
        signature: "strptime({format}, {timestring})",
        description: "Parse time string",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "reltime",
        signature: "reltime([{start} [, {end}]])",
        description: "Return relative time",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "reltimestr",
        signature: "reltimestr({time})",
        description: "Return string representation of reltime",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "reltimefloat",
        signature: "reltimefloat({time})",
        description: "Return Float from reltime",
        availability: Availability::Common,
    },
    // System info
    BuiltinFunction {
        name: "getpid",
        signature: "getpid()",
        description: "Return process ID of Vim",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "hostname",
        signature: "hostname()",
        description: "Return name of host machine",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "tempname",
        signature: "tempname()",
        description: "Return name of a temp file",
        availability: Availability::Common,
    },
    // Input functions
    BuiltinFunction {
        name: "getchar",
        signature: "getchar([{expr}])",
        description: "Get one character from user",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getcharstr",
        signature: "getcharstr([{expr}])",
        description: "Get one character as string",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getcharmod",
        signature: "getcharmod()",
        description: "Return modifiers for last getchar()",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "inputlist",
        signature: "inputlist({textlist})",
        description: "Let user pick from a list",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "inputsecret",
        signature: "inputsecret({prompt} [, {text}])",
        description: "Get input without showing it",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "inputsave",
        signature: "inputsave()",
        description: "Save typeahead",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "inputrestore",
        signature: "inputrestore()",
        description: "Restore typeahead",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "inputdialog",
        signature: "inputdialog({prompt} [, {text} [, {cancelreturn}]])",
        description: "Like input() but in a GUI dialog",
        availability: Availability::Common,
    },
    // Match functions
    BuiltinFunction {
        name: "matchadd",
        signature: "matchadd({group}, {pattern} [, {priority} [, {id} [, {dict}]]])",
        description: "Add match highlighting",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "matchaddpos",
        signature: "matchaddpos({group}, {pos} [, {priority} [, {id} [, {dict}]]])",
        description: "Add match at positions",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "matcharg",
        signature: "matcharg({nr})",
        description: "Return arguments of :match",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "matchdelete",
        signature: "matchdelete({id} [, {win}])",
        description: "Delete match by ID",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "clearmatches",
        signature: "clearmatches([{win}])",
        description: "Clear all matches",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getmatches",
        signature: "getmatches([{win}])",
        description: "Return list of matches",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setmatches",
        signature: "setmatches({list} [, {win}])",
        description: "Restore matches from list",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "matchfuzzy",
        signature: "matchfuzzy({list}, {str} [, {dict}])",
        description: "Return fuzzy matches",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "matchfuzzypos",
        signature: "matchfuzzypos({list}, {str} [, {dict}])",
        description: "Return fuzzy matches with positions",
        availability: Availability::Common,
    },
    // Cursor/mark functions
    BuiltinFunction {
        name: "getcharpos",
        signature: "getcharpos({expr})",
        description: "Return char position of mark",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setcharpos",
        signature: "setcharpos({expr}, {list})",
        description: "Set char position of mark",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getcursorcharpos",
        signature: "getcursorcharpos([{winnr}])",
        description: "Return cursor char position",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setcursorcharpos",
        signature: "setcursorcharpos({lnum}, {col} [, {off}])",
        description: "Set cursor char position",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "charcol",
        signature: "charcol({expr} [, {winid}])",
        description: "Return char column of position",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getmarklist",
        signature: "getmarklist([{buf}])",
        description: "Return list of marks",
        availability: Availability::Common,
    },
    // File info functions
    BuiltinFunction {
        name: "getfsize",
        signature: "getfsize({fname})",
        description: "Return file size in bytes",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getftime",
        signature: "getftime({fname})",
        description: "Return file modification time",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getfperm",
        signature: "getfperm({fname})",
        description: "Return file permissions string",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setfperm",
        signature: "setfperm({fname}, {mode})",
        description: "Set file permissions",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getftype",
        signature: "getftype({fname})",
        description: "Return type of file",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "resolve",
        signature: "resolve({filename})",
        description: "Resolve symbolic links",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "simplify",
        signature: "simplify({filename})",
        description: "Simplify path without resolving links",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "pathshorten",
        signature: "pathshorten({path} [, {len}])",
        description: "Shorten directory names in path",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "isabsolutepath",
        signature: "isabsolutepath({path})",
        description: "Return TRUE if {path} is absolute",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "readdir",
        signature: "readdir({dir} [, {expr}])",
        description: "Return list of files in directory",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "readdirex",
        signature: "readdirex({dir} [, {expr} [, {dict}]])",
        description: "Return list of file info in directory",
        availability: Availability::Common,
    },
    // List functions
    BuiltinFunction {
        name: "min",
        signature: "min({expr})",
        description: "Return minimum value in list",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "max",
        signature: "max({expr})",
        description: "Return maximum value in list",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "reduce",
        signature: "reduce({object}, {func} [, {initial}])",
        description: "Reduce list to single value",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "mapnew",
        signature: "mapnew({expr1}, {expr2})",
        description: "Like map() but creates new list",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "extendnew",
        signature: "extendnew({expr1}, {expr2} [, {expr3}])",
        description: "Like extend() but creates new list",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "flattennew",
        signature: "flattennew({list} [, {maxdepth}])",
        description: "Like flatten() but creates new list",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "indexof",
        signature: "indexof({object}, {expr} [, {opts}])",
        description: "Return index where {expr} is true",
        availability: Availability::Common,
    },
    // Quickfix/location list
    BuiltinFunction {
        name: "getqflist",
        signature: "getqflist([{what}])",
        description: "Return quickfix list",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setqflist",
        signature: "setqflist({list} [, {action} [, {what}]])",
        description: "Set quickfix list",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getloclist",
        signature: "getloclist({nr} [, {what}])",
        description: "Return location list",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setloclist",
        signature: "setloclist({nr}, {list} [, {action} [, {what}]])",
        description: "Set location list",
        availability: Availability::Common,
    },
    // Jump/change list
    BuiltinFunction {
        name: "getjumplist",
        signature: "getjumplist([{winnr} [, {tabnr}]])",
        description: "Return jump list",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getchangelist",
        signature: "getchangelist([{buf}])",
        description: "Return change list",
        availability: Availability::Common,
    },
    // Tag functions
    BuiltinFunction {
        name: "taglist",
        signature: "taglist({expr} [, {filename}])",
        description: "Return list of matching tags",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "tagfiles",
        signature: "tagfiles()",
        description: "Return list of tag files",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "gettagstack",
        signature: "gettagstack([{winnr}])",
        description: "Return tag stack",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "settagstack",
        signature: "settagstack({winnr}, {dict} [, {action}])",
        description: "Set tag stack",
        availability: Availability::Common,
    },
    // Register functions
    BuiltinFunction {
        name: "getreg",
        signature: "getreg([{regname} [, 1 [, {list}]]])",
        description: "Return contents of register",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setreg",
        signature: "setreg({regname}, {value} [, {options}])",
        description: "Set register contents",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getregtype",
        signature: "getregtype([{regname}])",
        description: "Return type of register",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getreginfo",
        signature: "getreginfo([{regname}])",
        description: "Return info about register",
        availability: Availability::Common,
    },
    // Syntax/highlight functions
    BuiltinFunction {
        name: "synID",
        signature: "synID({lnum}, {col}, {trans})",
        description: "Return syntax ID at position",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "synIDattr",
        signature: "synIDattr({synID}, {what} [, {mode}])",
        description: "Return attribute of syntax ID",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "synIDtrans",
        signature: "synIDtrans({synID})",
        description: "Return translated syntax ID",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "synstack",
        signature: "synstack({lnum}, {col})",
        description: "Return syntax ID stack at position",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "synconcealed",
        signature: "synconcealed({lnum}, {col})",
        description: "Return concealed info at position",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "hlID",
        signature: "hlID({name})",
        description: "Return highlight ID of {name}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "hlexists",
        signature: "hlexists({name})",
        description: "Return TRUE if highlight {name} exists",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "hlget",
        signature: "hlget([{name} [, {resolve}]])",
        description: "Return highlight definition",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "hlset",
        signature: "hlset({list})",
        description: "Set highlight definitions",
        availability: Availability::Common,
    },
    // Completion functions
    BuiltinFunction {
        name: "complete",
        signature: "complete({startcol}, {matches})",
        description: "Set completion matches",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "complete_add",
        signature: "complete_add({expr})",
        description: "Add completion match",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "complete_check",
        signature: "complete_check()",
        description: "Return TRUE if completion interrupted",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "complete_info",
        signature: "complete_info([{what}])",
        description: "Return completion information",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "pumvisible",
        signature: "pumvisible()",
        description: "Return TRUE if popup menu visible",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "pum_getpos",
        signature: "pum_getpos()",
        description: "Return position of popup menu",
        availability: Availability::Common,
    },
    // Command line functions
    BuiltinFunction {
        name: "getcmdline",
        signature: "getcmdline()",
        description: "Return current command line",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setcmdline",
        signature: "setcmdline({str} [, {pos}])",
        description: "Set command line contents",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getcmdpos",
        signature: "getcmdpos()",
        description: "Return cursor position in cmdline",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setcmdpos",
        signature: "setcmdpos({pos})",
        description: "Set cursor position in cmdline",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getcmdtype",
        signature: "getcmdtype()",
        description: "Return current command line type",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getcmdwintype",
        signature: "getcmdwintype()",
        description: "Return command line window type",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getcompletion",
        signature: "getcompletion({pat}, {type} [, {filtered}])",
        description: "Return command line completions",
        availability: Availability::Common,
    },
    // Misc functions
    BuiltinFunction {
        name: "and",
        signature: "and({expr}, {expr})",
        description: "Bitwise AND",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "or",
        signature: "or({expr}, {expr})",
        description: "Bitwise OR",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "xor",
        signature: "xor({expr}, {expr})",
        description: "Bitwise XOR",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "invert",
        signature: "invert({expr})",
        description: "Bitwise invert",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "sha256",
        signature: "sha256({string})",
        description: "Return SHA256 checksum",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "rand",
        signature: "rand([{expr}])",
        description: "Return pseudo-random number",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "srand",
        signature: "srand([{expr}])",
        description: "Initialize random number seed",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "state",
        signature: "state([{what}])",
        description: "Return current state of Vim",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "undofile",
        signature: "undofile({name})",
        description: "Return undo file name for {name}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "undotree",
        signature: "undotree([{buf}])",
        description: "Return undo tree",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "shiftwidth",
        signature: "shiftwidth([{col}])",
        description: "Return effective shiftwidth value",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "wordcount",
        signature: "wordcount()",
        description: "Return word count statistics",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "nextnonblank",
        signature: "nextnonblank({lnum})",
        description: "Return line nr of next non-blank",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "prevnonblank",
        signature: "prevnonblank({lnum})",
        description: "Return line nr of prev non-blank",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "byte2line",
        signature: "byte2line({byte})",
        description: "Return line number at byte count",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "line2byte",
        signature: "line2byte({lnum})",
        description: "Return byte count at line",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "diff_filler",
        signature: "diff_filler({lnum})",
        description: "Return filler lines at {lnum}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "diff_hlID",
        signature: "diff_hlID({lnum}, {col})",
        description: "Return diff highlight ID",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "foldclosed",
        signature: "foldclosed({lnum})",
        description: "Return first line of fold at {lnum}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "foldclosedend",
        signature: "foldclosedend({lnum})",
        description: "Return last line of fold at {lnum}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "foldlevel",
        signature: "foldlevel({lnum})",
        description: "Return fold level at {lnum}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "foldtext",
        signature: "foldtext()",
        description: "Return text for closed fold",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "foldtextresult",
        signature: "foldtextresult({lnum})",
        description: "Return text displayed for fold",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "screenattr",
        signature: "screenattr({row}, {col})",
        description: "Return attribute at screen position",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "screenchar",
        signature: "screenchar({row}, {col})",
        description: "Return character at screen position",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "screenchars",
        signature: "screenchars({row}, {col})",
        description: "Return characters at screen position",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "screenstring",
        signature: "screenstring({row}, {col})",
        description: "Return string at screen position",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "screenpos",
        signature: "screenpos({winid}, {lnum}, {col})",
        description: "Return screen position of text",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "screencol",
        signature: "screencol()",
        description: "Return cursor screen column",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "screenrow",
        signature: "screenrow()",
        description: "Return cursor screen row",
        availability: Availability::Common,
    },
    // Neovim-only functions
    BuiltinFunction {
        name: "stdpath",
        signature: "stdpath({what})",
        description: "Return standard path locations",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "api_info",
        signature: "api_info()",
        description: "Return API metadata",
        availability: Availability::NeovimOnly,
    },
    // Sign functions
    BuiltinFunction {
        name: "sign_define",
        signature: "sign_define({name} [, {dict}])",
        description: "Define or update a sign",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "sign_getdefined",
        signature: "sign_getdefined([{name}])",
        description: "Return list of defined signs",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "sign_getplaced",
        signature: "sign_getplaced([{buf} [, {dict}]])",
        description: "Return list of placed signs",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "sign_jump",
        signature: "sign_jump({id}, {group}, {buf})",
        description: "Jump to a placed sign",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "sign_place",
        signature: "sign_place({id}, {group}, {name}, {buf} [, {dict}])",
        description: "Place a sign",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "sign_placelist",
        signature: "sign_placelist({list})",
        description: "Place multiple signs",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "sign_undefine",
        signature: "sign_undefine([{name}])",
        description: "Undefine signs",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "sign_unplace",
        signature: "sign_unplace({group} [, {dict}])",
        description: "Unplace signs",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "sign_unplacelist",
        signature: "sign_unplacelist({list})",
        description: "Unplace multiple signs",
        availability: Availability::Common,
    },
    // Text property functions
    BuiltinFunction {
        name: "prop_add",
        signature: "prop_add({lnum}, {col}, {props})",
        description: "Add a text property",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "prop_add_list",
        signature: "prop_add_list({props}, {items})",
        description: "Add text properties to multiple positions",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "prop_clear",
        signature: "prop_clear({lnum} [, {lnum_end} [, {props}]])",
        description: "Clear text properties",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "prop_find",
        signature: "prop_find({props} [, {direction}])",
        description: "Find a text property",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "prop_list",
        signature: "prop_list({lnum} [, {props}])",
        description: "Return list of text properties",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "prop_remove",
        signature: "prop_remove({props} [, {lnum} [, {lnum_end}]])",
        description: "Remove text properties",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "prop_type_add",
        signature: "prop_type_add({name}, {props})",
        description: "Add a text property type",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "prop_type_change",
        signature: "prop_type_change({name}, {props})",
        description: "Change a text property type",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "prop_type_delete",
        signature: "prop_type_delete({name} [, {props}])",
        description: "Delete a text property type",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "prop_type_get",
        signature: "prop_type_get({name} [, {props}])",
        description: "Return text property type definition",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "prop_type_list",
        signature: "prop_type_list([{props}])",
        description: "Return list of text property types",
        availability: Availability::Common,
    },
    // Spell functions
    BuiltinFunction {
        name: "spellbadword",
        signature: "spellbadword([{sentence}])",
        description: "Return misspelled word at cursor",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "spellsuggest",
        signature: "spellsuggest({word} [, {max} [, {capital}]])",
        description: "Return spelling suggestions",
        availability: Availability::Common,
    },
    // History functions
    BuiltinFunction {
        name: "histadd",
        signature: "histadd({history}, {item})",
        description: "Add item to history",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "histdel",
        signature: "histdel({history} [, {item}])",
        description: "Delete item from history",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "histget",
        signature: "histget({history} [, {index}])",
        description: "Return item from history",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "histnr",
        signature: "histnr({history})",
        description: "Return number of items in history",
        availability: Availability::Common,
    },
    // Assert functions
    BuiltinFunction {
        name: "assert_equal",
        signature: "assert_equal({expected}, {actual} [, {msg}])",
        description: "Assert two values are equal",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "assert_notequal",
        signature: "assert_notequal({expected}, {actual} [, {msg}])",
        description: "Assert two values are not equal",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "assert_true",
        signature: "assert_true({actual} [, {msg}])",
        description: "Assert value is true",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "assert_false",
        signature: "assert_false({actual} [, {msg}])",
        description: "Assert value is false",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "assert_match",
        signature: "assert_match({pattern}, {actual} [, {msg}])",
        description: "Assert value matches pattern",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "assert_notmatch",
        signature: "assert_notmatch({pattern}, {actual} [, {msg}])",
        description: "Assert value does not match pattern",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "assert_exception",
        signature: "assert_exception({error} [, {msg}])",
        description: "Assert exception was thrown",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "assert_beeps",
        signature: "assert_beeps({cmd})",
        description: "Assert command causes a beep",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "assert_nobeep",
        signature: "assert_nobeep({cmd})",
        description: "Assert command does not beep",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "assert_fails",
        signature: "assert_fails({cmd} [, {error} [, {msg} [, {lnum} [, {context}]]]])",
        description: "Assert command fails with error",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "assert_inrange",
        signature: "assert_inrange({lower}, {upper}, {actual} [, {msg}])",
        description: "Assert value is in range",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "assert_report",
        signature: "assert_report({msg})",
        description: "Report a test failure",
        availability: Availability::Common,
    },
    // Listener functions
    BuiltinFunction {
        name: "listener_add",
        signature: "listener_add({callback} [, {buf}])",
        description: "Add a buffer change listener",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "listener_flush",
        signature: "listener_flush([{buf}])",
        description: "Invoke listeners for buffer",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "listener_remove",
        signature: "listener_remove({id})",
        description: "Remove a listener",
        availability: Availability::Common,
    },
    // Mapping functions
    BuiltinFunction {
        name: "maparg",
        signature: "maparg({name} [, {mode} [, {abbr} [, {dict}]]])",
        description: "Return mapping definition",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "mapcheck",
        signature: "mapcheck({name} [, {mode} [, {abbr}]])",
        description: "Check if mapping exists",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "mapset",
        signature: "mapset({mode}, {abbr}, {dict})",
        description: "Set a mapping from dict",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "maplist",
        signature: "maplist([{abbr}])",
        description: "Return list of all mappings",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "hasmapto",
        signature: "hasmapto({what} [, {mode} [, {abbr}]])",
        description: "Check if mapping to {what} exists",
        availability: Availability::Common,
    },
    // Autocommand functions
    BuiltinFunction {
        name: "autocmd_add",
        signature: "autocmd_add({acmds})",
        description: "Add autocommands from list",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "autocmd_delete",
        signature: "autocmd_delete({acmds})",
        description: "Delete autocommands",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "autocmd_get",
        signature: "autocmd_get([{opts}])",
        description: "Return list of autocommands",
        availability: Availability::Common,
    },
    // Changenr/undo
    BuiltinFunction {
        name: "changenr",
        signature: "changenr()",
        description: "Return current change number",
        availability: Availability::Common,
    },
    // Encoding
    BuiltinFunction {
        name: "iconv",
        signature: "iconv({string}, {from}, {to})",
        description: "Convert encoding of {string}",
        availability: Availability::Common,
    },
    // Server functions
    BuiltinFunction {
        name: "serverlist",
        signature: "serverlist()",
        description: "Return list of server names",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "remote_expr",
        signature: "remote_expr({server}, {string} [, {idvar} [, {timeout}]])",
        description: "Send expression to server",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "remote_foreground",
        signature: "remote_foreground({server})",
        description: "Bring server to foreground",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "remote_peek",
        signature: "remote_peek({serverid} [, {retvar}])",
        description: "Check for server reply",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "remote_read",
        signature: "remote_read({serverid} [, {timeout}])",
        description: "Read reply from server",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "remote_send",
        signature: "remote_send({server}, {string} [, {idvar}])",
        description: "Send keys to server",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "remote_startserver",
        signature: "remote_startserver({name})",
        description: "Start server with {name}",
        availability: Availability::Common,
    },
    // Scripting/evaluation
    BuiltinFunction {
        name: "libcall",
        signature: "libcall({lib}, {func}, {arg})",
        description: "Call function in library",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "libcallnr",
        signature: "libcallnr({lib}, {func}, {arg})",
        description: "Call function in library, return number",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "luaeval",
        signature: "luaeval({expr} [, {arg}])",
        description: "Evaluate Lua expression",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "perleval",
        signature: "perleval({expr})",
        description: "Evaluate Perl expression",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "py3eval",
        signature: "py3eval({expr})",
        description: "Evaluate Python3 expression",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "pyeval",
        signature: "pyeval({expr})",
        description: "Evaluate Python expression",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "pyxeval",
        signature: "pyxeval({expr})",
        description: "Evaluate Python expression (2 or 3)",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "rubyeval",
        signature: "rubyeval({expr})",
        description: "Evaluate Ruby expression",
        availability: Availability::VimOnly,
    },
    // Vim-only: Popup functions
    BuiltinFunction {
        name: "popup_create",
        signature: "popup_create({what}, {options})",
        description: "Create a popup window",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_atcursor",
        signature: "popup_atcursor({what}, {options})",
        description: "Create popup at cursor position",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_beval",
        signature: "popup_beval({what}, {options})",
        description: "Create popup for balloon eval",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_notification",
        signature: "popup_notification({what}, {options})",
        description: "Create a notification popup",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_dialog",
        signature: "popup_dialog({what}, {options})",
        description: "Create a dialog popup",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_menu",
        signature: "popup_menu({what}, {options})",
        description: "Create a menu popup",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_hide",
        signature: "popup_hide({id})",
        description: "Hide a popup",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_show",
        signature: "popup_show({id})",
        description: "Show a hidden popup",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_move",
        signature: "popup_move({id}, {options})",
        description: "Move popup to new position",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_setoptions",
        signature: "popup_setoptions({id}, {options})",
        description: "Set popup options",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_settext",
        signature: "popup_settext({id}, {text})",
        description: "Set popup text",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_close",
        signature: "popup_close({id} [, {result}])",
        description: "Close popup",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_clear",
        signature: "popup_clear([{force}])",
        description: "Close all popups",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_filter_menu",
        signature: "popup_filter_menu({id}, {key})",
        description: "Filter for popup menu",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_filter_yesno",
        signature: "popup_filter_yesno({id}, {key})",
        description: "Filter for yes/no popup",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_getoptions",
        signature: "popup_getoptions({id})",
        description: "Return popup options",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_getpos",
        signature: "popup_getpos({id})",
        description: "Return popup position",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_findinfo",
        signature: "popup_findinfo()",
        description: "Return info popup window ID",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_findpreview",
        signature: "popup_findpreview()",
        description: "Return preview popup window ID",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_list",
        signature: "popup_list()",
        description: "Return list of all popup IDs",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_locate",
        signature: "popup_locate({row}, {col})",
        description: "Return popup ID at screen position",
        availability: Availability::VimOnly,
    },
    // Vim-only: Channel functions
    BuiltinFunction {
        name: "ch_canread",
        signature: "ch_canread({handle})",
        description: "Return TRUE if channel can be read",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "ch_close",
        signature: "ch_close({handle})",
        description: "Close channel",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "ch_close_in",
        signature: "ch_close_in({handle})",
        description: "Close input part of channel",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "ch_evalexpr",
        signature: "ch_evalexpr({handle}, {expr} [, {options}])",
        description: "Send expression over channel, return response",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "ch_evalraw",
        signature: "ch_evalraw({handle}, {string} [, {options}])",
        description: "Send raw string over channel",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "ch_getbufnr",
        signature: "ch_getbufnr({handle}, {what})",
        description: "Return buffer number for channel",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "ch_getjob",
        signature: "ch_getjob({handle})",
        description: "Return job for channel",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "ch_info",
        signature: "ch_info({handle})",
        description: "Return info about channel",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "ch_log",
        signature: "ch_log({msg} [, {handle}])",
        description: "Write message to channel log",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "ch_logfile",
        signature: "ch_logfile({fname} [, {mode}])",
        description: "Start logging channel activity",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "ch_open",
        signature: "ch_open({address} [, {options}])",
        description: "Open channel to {address}",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "ch_read",
        signature: "ch_read({handle} [, {options}])",
        description: "Read from channel",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "ch_readblob",
        signature: "ch_readblob({handle} [, {options}])",
        description: "Read blob from channel",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "ch_readraw",
        signature: "ch_readraw({handle} [, {options}])",
        description: "Read raw string from channel",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "ch_sendexpr",
        signature: "ch_sendexpr({handle}, {expr} [, {options}])",
        description: "Send expression over channel",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "ch_sendraw",
        signature: "ch_sendraw({handle}, {expr} [, {options}])",
        description: "Send raw string over channel",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "ch_setoptions",
        signature: "ch_setoptions({handle}, {options})",
        description: "Set channel options",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "ch_status",
        signature: "ch_status({handle} [, {options}])",
        description: "Return status of channel",
        availability: Availability::VimOnly,
    },
    // Vim-only: Job functions
    BuiltinFunction {
        name: "job_getchannel",
        signature: "job_getchannel({job})",
        description: "Return channel for job",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "job_info",
        signature: "job_info([{job}])",
        description: "Return info about job",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "job_setoptions",
        signature: "job_setoptions({job}, {options})",
        description: "Set job options",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "job_start",
        signature: "job_start({command} [, {options}])",
        description: "Start a job",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "job_status",
        signature: "job_status({job})",
        description: "Return status of job",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "job_stop",
        signature: "job_stop({job} [, {how}])",
        description: "Stop a job",
        availability: Availability::VimOnly,
    },
    // Vim-only: Terminal functions
    BuiltinFunction {
        name: "term_start",
        signature: "term_start({cmd} [, {options}])",
        description: "Start terminal in new window",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_list",
        signature: "term_list()",
        description: "Return list of terminal buffers",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_sendkeys",
        signature: "term_sendkeys({buf}, {keys})",
        description: "Send keys to terminal",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_wait",
        signature: "term_wait({buf} [, {time}])",
        description: "Wait for terminal to update",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_getjob",
        signature: "term_getjob({buf})",
        description: "Return job for terminal",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_getline",
        signature: "term_getline({buf}, {row})",
        description: "Return line from terminal",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_getscrolled",
        signature: "term_getscrolled({buf})",
        description: "Return scrolled lines count",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_getsize",
        signature: "term_getsize({buf})",
        description: "Return terminal size",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_getstatus",
        signature: "term_getstatus({buf})",
        description: "Return terminal status",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_gettitle",
        signature: "term_gettitle({buf})",
        description: "Return terminal title",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_gettty",
        signature: "term_gettty({buf} [, {input}])",
        description: "Return tty of terminal",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_setansicolors",
        signature: "term_setansicolors({buf}, {colors})",
        description: "Set ANSI colors for terminal",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_getansicolors",
        signature: "term_getansicolors({buf})",
        description: "Return ANSI colors of terminal",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_setapi",
        signature: "term_setapi({buf}, {expr})",
        description: "Set API function prefix for terminal",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_setkill",
        signature: "term_setkill({buf}, {how})",
        description: "Set how to kill terminal job",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_setrestore",
        signature: "term_setrestore({buf}, {command})",
        description: "Set command to restore terminal",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_setsize",
        signature: "term_setsize({buf}, {rows}, {cols})",
        description: "Set terminal size",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_dumpdiff",
        signature: "term_dumpdiff({filename}, {filename} [, {options}])",
        description: "Show diff of terminal dumps",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_dumpload",
        signature: "term_dumpload({filename} [, {options}])",
        description: "Load terminal dump in window",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_dumpwrite",
        signature: "term_dumpwrite({buf}, {filename} [, {options}])",
        description: "Write terminal dump to file",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_getattr",
        signature: "term_getattr({attr}, {what})",
        description: "Return attribute of terminal cell",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_getcursor",
        signature: "term_getcursor({buf})",
        description: "Return cursor position in terminal",
        availability: Availability::VimOnly,
    },
    // Neovim-only: Floating window / extmark functions
    BuiltinFunction {
        name: "nvim_create_buf",
        signature: "nvim_create_buf({listed}, {scratch})",
        description: "Create a new buffer",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_open_win",
        signature: "nvim_open_win({buffer}, {enter}, {config})",
        description: "Open a floating window",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_set_config",
        signature: "nvim_win_set_config({window}, {config})",
        description: "Set window config",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_get_config",
        signature: "nvim_win_get_config({window})",
        description: "Get window config",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_close",
        signature: "nvim_win_close({window}, {force})",
        description: "Close window",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_set_lines",
        signature: "nvim_buf_set_lines({buffer}, {start}, {end}, {strict}, {replacement})",
        description: "Set buffer lines",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_get_lines",
        signature: "nvim_buf_get_lines({buffer}, {start}, {end}, {strict})",
        description: "Get buffer lines",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_set_text",
        signature: "nvim_buf_set_text({buffer}, {start_row}, {start_col}, {end_row}, {end_col}, {replacement})",
        description: "Set text in buffer region",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_get_text",
        signature: "nvim_buf_get_text({buffer}, {start_row}, {start_col}, {end_row}, {end_col}, {opts})",
        description: "Get text from buffer region",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_set_extmark",
        signature: "nvim_buf_set_extmark({buffer}, {ns_id}, {line}, {col}, {opts})",
        description: "Create or update extmark",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_get_extmarks",
        signature: "nvim_buf_get_extmarks({buffer}, {ns_id}, {start}, {end}, {opts})",
        description: "Get extmarks in range",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_del_extmark",
        signature: "nvim_buf_del_extmark({buffer}, {ns_id}, {id})",
        description: "Delete extmark",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_create_namespace",
        signature: "nvim_create_namespace({name})",
        description: "Create namespace for extmarks",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_set_hl",
        signature: "nvim_set_hl({ns_id}, {name}, {val})",
        description: "Set highlight group",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_hl",
        signature: "nvim_get_hl({ns_id}, {opts})",
        description: "Get highlight definition",
        availability: Availability::NeovimOnly,
    },
    // More common utility functions
    BuiltinFunction {
        name: "matchstrpos",
        signature: "matchstrpos({string}, {pattern} [, {start} [, {count}]])",
        description: "Return match and positions",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "bufwinid",
        signature: "bufwinid({buf})",
        description: "Return window ID of buffer",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "tabpagewinnr",
        signature: "tabpagewinnr({tabarg} [, {arg}])",
        description: "Return window number in tab page",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "islocked",
        signature: "islocked({expr})",
        description: "Return TRUE if {expr} is locked",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setcellwidths",
        signature: "setcellwidths({list})",
        description: "Set character cell widths",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getcellwidths",
        signature: "getcellwidths()",
        description: "Return character cell width overrides",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "charclass",
        signature: "charclass({string})",
        description: "Return character class",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getcharpos",
        signature: "getcharpos({expr})",
        description: "Return character position",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getmousepos",
        signature: "getmousepos()",
        description: "Return mouse position",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getscriptinfo",
        signature: "getscriptinfo([{opts}])",
        description: "Return list of sourced scripts",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "gettext",
        signature: "gettext({text})",
        description: "Return translated text",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "searchcount",
        signature: "searchcount([{options}])",
        description: "Return search match count info",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "searchdecl",
        signature: "searchdecl({name} [, {global} [, {thisblock}]])",
        description: "Search for declaration",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setcmdline",
        signature: "setcmdline({str} [, {pos}])",
        description: "Set command line text",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setcharpos",
        signature: "setcharpos({expr}, {list})",
        description: "Set character position",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setcharsearch",
        signature: "setcharsearch({dict})",
        description: "Set character search settings",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getcharsearch",
        signature: "getcharsearch()",
        description: "Return character search settings",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setcursorcharpos",
        signature: "setcursorcharpos({list})",
        description: "Set cursor character position",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getcmdcompltype",
        signature: "getcmdcompltype()",
        description: "Return current command completion type",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getcmdscreenpos",
        signature: "getcmdscreenpos()",
        description: "Return cursor screen position in cmdline",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "fullcommand",
        signature: "fullcommand({name} [, {vim9}])",
        description: "Return full command name",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getbufoneline",
        signature: "getbufoneline({buf}, {lnum})",
        description: "Return single line from buffer",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "echoraw",
        signature: "echoraw({string})",
        description: "Output string without processing",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "keytrans",
        signature: "keytrans({string})",
        description: "Translate key codes to readable form",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setbufvar",
        signature: "setbufvar({buf}, {varname}, {val})",
        description: "Set buffer variable",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "setwinvar",
        signature: "setwinvar({winnr}, {varname}, {val})",
        description: "Set window variable",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "settabvar",
        signature: "settabvar({tabnr}, {varname}, {val})",
        description: "Set tab variable",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getcursorcharpos",
        signature: "getcursorcharpos([{winnr}])",
        description: "Return cursor character position",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "virtcol2col",
        signature: "virtcol2col({winid}, {lnum}, {col})",
        description: "Convert virtual column to byte column",
        availability: Availability::Common,
    },
    // Blob functions
    BuiltinFunction {
        name: "blob2list",
        signature: "blob2list({blob})",
        description: "Convert blob to list of numbers",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "list2blob",
        signature: "list2blob({list})",
        description: "Convert list of numbers to blob",
        availability: Availability::Common,
    },
    // Sound functions (Vim-only)
    BuiltinFunction {
        name: "sound_clear",
        signature: "sound_clear()",
        description: "Stop all sounds",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "sound_playevent",
        signature: "sound_playevent({name} [, {callback}])",
        description: "Play a sound event",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "sound_playfile",
        signature: "sound_playfile({path} [, {callback}])",
        description: "Play a sound file",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "sound_stop",
        signature: "sound_stop({id})",
        description: "Stop playing a sound",
        availability: Availability::VimOnly,
    },
    // Digraph functions
    BuiltinFunction {
        name: "digraph_get",
        signature: "digraph_get({chars})",
        description: "Return digraph for {chars}",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "digraph_getlist",
        signature: "digraph_getlist([{listall}])",
        description: "Return list of digraphs",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "digraph_set",
        signature: "digraph_set({chars}, {digraph})",
        description: "Set a digraph",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "digraph_setlist",
        signature: "digraph_setlist({list})",
        description: "Set multiple digraphs",
        availability: Availability::Common,
    },
    // Prompt buffer functions
    BuiltinFunction {
        name: "prompt_getprompt",
        signature: "prompt_getprompt({buf})",
        description: "Return prompt text of buffer",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "prompt_setcallback",
        signature: "prompt_setcallback({buf}, {callback})",
        description: "Set callback for prompt buffer",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "prompt_setinterrupt",
        signature: "prompt_setinterrupt({buf}, {callback})",
        description: "Set interrupt callback for prompt",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "prompt_setprompt",
        signature: "prompt_setprompt({buf}, {text})",
        description: "Set prompt text for buffer",
        availability: Availability::Common,
    },
    // Timer functions (missing)
    BuiltinFunction {
        name: "timer_info",
        signature: "timer_info([{id}])",
        description: "Return information about timers",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "timer_pause",
        signature: "timer_pause({id}, {pause})",
        description: "Pause or unpause a timer",
        availability: Availability::Common,
    },
    // Register functions
    BuiltinFunction {
        name: "reg_executing",
        signature: "reg_executing()",
        description: "Return register being executed",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "reg_recording",
        signature: "reg_recording()",
        description: "Return register being recorded to",
        availability: Availability::Common,
    },
    // GUI/Browse functions (Vim-only)
    BuiltinFunction {
        name: "browse",
        signature: "browse({save}, {title}, {initdir}, {default})",
        description: "Open file browser dialog",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "browsedir",
        signature: "browsedir({title}, {initdir})",
        description: "Open directory browser dialog",
        availability: Availability::VimOnly,
    },
    // Menu functions
    BuiltinFunction {
        name: "menu_info",
        signature: "menu_info({name} [, {mode}])",
        description: "Return information about a menu",
        availability: Availability::Common,
    },
    // Event/interrupt functions
    BuiltinFunction {
        name: "eventhandler",
        signature: "eventhandler()",
        description: "Return TRUE if in event handler",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "interrupt",
        signature: "interrupt()",
        description: "Interrupt script execution",
        availability: Availability::Common,
    },
    // Window movement functions
    BuiltinFunction {
        name: "win_move_separator",
        signature: "win_move_separator({nr}, {offset})",
        description: "Move window vertical separator",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "win_move_statusline",
        signature: "win_move_statusline({nr}, {offset})",
        description: "Move window status line",
        availability: Availability::Common,
    },
    // Other scripting functions
    BuiltinFunction {
        name: "mzeval",
        signature: "mzeval({expr})",
        description: "Evaluate MzScheme expression",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "debugbreak",
        signature: "debugbreak({pid})",
        description: "Interrupt process for debugging",
        availability: Availability::VimOnly,
    },
    // Balloon functions (Vim-only)
    BuiltinFunction {
        name: "balloon_gettext",
        signature: "balloon_gettext()",
        description: "Return current balloon text",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "balloon_show",
        signature: "balloon_show({expr})",
        description: "Show balloon with text",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "balloon_split",
        signature: "balloon_split({msg})",
        description: "Split message for balloon",
        availability: Availability::VimOnly,
    },
    // IM status
    BuiltinFunction {
        name: "getimstatus",
        signature: "getimstatus()",
        description: "Return IM status",
        availability: Availability::Common,
    },
    // ID function
    BuiltinFunction {
        name: "id",
        signature: "id({expr})",
        description: "Return unique identifier for reference",
        availability: Availability::Common,
    },
    // More Neovim API functions
    BuiltinFunction {
        name: "nvim_win_set_cursor",
        signature: "nvim_win_set_cursor({window}, {pos})",
        description: "Set cursor position in window",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_get_cursor",
        signature: "nvim_win_get_cursor({window})",
        description: "Get cursor position in window",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_line_count",
        signature: "nvim_buf_line_count({buffer})",
        description: "Return line count of buffer",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_get_name",
        signature: "nvim_buf_get_name({buffer})",
        description: "Return buffer name",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_set_name",
        signature: "nvim_buf_set_name({buffer}, {name})",
        description: "Set buffer name",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_is_valid",
        signature: "nvim_buf_is_valid({buffer})",
        description: "Check if buffer is valid",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_delete",
        signature: "nvim_buf_delete({buffer}, {opts})",
        description: "Delete buffer",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_list_bufs",
        signature: "nvim_list_bufs()",
        description: "List all buffers",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_list_wins",
        signature: "nvim_list_wins()",
        description: "List all windows",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_current_buf",
        signature: "nvim_get_current_buf()",
        description: "Return current buffer handle",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_current_win",
        signature: "nvim_get_current_win()",
        description: "Return current window handle",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_set_current_buf",
        signature: "nvim_set_current_buf({buffer})",
        description: "Set current buffer",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_set_current_win",
        signature: "nvim_set_current_win({window})",
        description: "Set current window",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_echo",
        signature: "nvim_echo({chunks}, {history}, {opts})",
        description: "Echo message with highlights",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_notify",
        signature: "nvim_notify({msg}, {log_level}, {opts})",
        description: "Show notification message",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_exec_lua",
        signature: "nvim_exec_lua({code}, {args})",
        description: "Execute Lua code",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_command",
        signature: "nvim_command({command})",
        description: "Execute ex command",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_eval",
        signature: "nvim_eval({expr})",
        description: "Evaluate vimscript expression",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_call_function",
        signature: "nvim_call_function({fn}, {args})",
        description: "Call vimscript function",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_replace_termcodes",
        signature: "nvim_replace_termcodes({str}, {from_part}, {do_lt}, {special})",
        description: "Replace terminal codes",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_feedkeys",
        signature: "nvim_feedkeys({keys}, {mode}, {escape_ks})",
        description: "Feed keys to Neovim",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_input",
        signature: "nvim_input({keys})",
        description: "Queue raw user input",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_mode",
        signature: "nvim_get_mode()",
        description: "Return current mode",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_option_value",
        signature: "nvim_get_option_value({name}, {opts})",
        description: "Get option value",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_set_option_value",
        signature: "nvim_set_option_value({name}, {value}, {opts})",
        description: "Set option value",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_var",
        signature: "nvim_get_var({name})",
        description: "Get global variable",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_set_var",
        signature: "nvim_set_var({name}, {value})",
        description: "Set global variable",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_del_var",
        signature: "nvim_del_var({name})",
        description: "Delete global variable",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_get_var",
        signature: "nvim_buf_get_var({buffer}, {name})",
        description: "Get buffer variable",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_set_var",
        signature: "nvim_buf_set_var({buffer}, {name}, {value})",
        description: "Set buffer variable",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_get_var",
        signature: "nvim_win_get_var({window}, {name})",
        description: "Get window variable",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_set_var",
        signature: "nvim_win_set_var({window}, {name}, {value})",
        description: "Set window variable",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_create_augroup",
        signature: "nvim_create_augroup({name}, {opts})",
        description: "Create autocommand group",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_create_autocmd",
        signature: "nvim_create_autocmd({event}, {opts})",
        description: "Create autocommand",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_del_augroup_by_id",
        signature: "nvim_del_augroup_by_id({id})",
        description: "Delete autocommand group by ID",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_del_augroup_by_name",
        signature: "nvim_del_augroup_by_name({name})",
        description: "Delete autocommand group by name",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_del_autocmd",
        signature: "nvim_del_autocmd({id})",
        description: "Delete autocommand",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_set_keymap",
        signature: "nvim_set_keymap({mode}, {lhs}, {rhs}, {opts})",
        description: "Set global keymap",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_del_keymap",
        signature: "nvim_del_keymap({mode}, {lhs})",
        description: "Delete global keymap",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_set_keymap",
        signature: "nvim_buf_set_keymap({buffer}, {mode}, {lhs}, {rhs}, {opts})",
        description: "Set buffer-local keymap",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_del_keymap",
        signature: "nvim_buf_del_keymap({buffer}, {mode}, {lhs})",
        description: "Delete buffer-local keymap",
        availability: Availability::NeovimOnly,
    },
    // Argument list functions
    BuiltinFunction {
        name: "argc",
        signature: "argc([{winid}])",
        description: "Return number of files in argument list",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "argidx",
        signature: "argidx()",
        description: "Return current index in argument list",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "arglistid",
        signature: "arglistid([{winnr} [, {tabnr}]])",
        description: "Return argument list ID",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "argv",
        signature: "argv([{nr} [, {winid}]])",
        description: "Return argument from argument list",
        availability: Availability::Common,
    },
    // Base64 functions
    BuiltinFunction {
        name: "base64_decode",
        signature: "base64_decode({string})",
        description: "Decode base64 encoded string",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "base64_encode",
        signature: "base64_encode({blob})",
        description: "Encode blob to base64 string",
        availability: Availability::Common,
    },
    // Blob/string conversion
    BuiltinFunction {
        name: "blob2str",
        signature: "blob2str({blob})",
        description: "Convert blob to string",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "str2blob",
        signature: "str2blob({string})",
        description: "Convert string to blob",
        availability: Availability::Common,
    },
    // Buffer/window
    BuiltinFunction {
        name: "bufwinnr",
        signature: "bufwinnr({buf})",
        description: "Return window number of buffer",
        availability: Availability::Common,
    },
    // Indent functions
    BuiltinFunction {
        name: "cindent",
        signature: "cindent({lnum})",
        description: "Return C indent for line",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "lispindent",
        signature: "lispindent({lnum})",
        description: "Return Lisp indent for line",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "indent",
        signature: "indent({lnum})",
        description: "Return indent of line",
        availability: Availability::Common,
    },
    // Command line
    BuiltinFunction {
        name: "cmdcomplete_info",
        signature: "cmdcomplete_info([{what}])",
        description: "Return command line completion info",
        availability: Availability::Common,
    },
    // Cscope
    BuiltinFunction {
        name: "cscope_connection",
        signature: "cscope_connection([{num} [, {dbpath} [, {prepend}]]])",
        description: "Check cscope connection",
        availability: Availability::VimOnly,
    },
    // File type detection
    BuiltinFunction {
        name: "did_filetype",
        signature: "did_filetype()",
        description: "Return TRUE if FileType autocommand was used",
        availability: Availability::Common,
    },
    // Diff
    BuiltinFunction {
        name: "diff",
        signature: "diff({fromlist}, {tolist} [, {options}])",
        description: "Return diff between two lists",
        availability: Availability::Common,
    },
    // Expand command
    BuiltinFunction {
        name: "expandcmd",
        signature: "expandcmd({string} [, {options}])",
        description: "Expand special items in command string",
        availability: Availability::Common,
    },
    // Find file/dir
    BuiltinFunction {
        name: "finddir",
        signature: "finddir({name} [, {path} [, {count}]])",
        description: "Find directory in path",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "findfile",
        signature: "findfile({name} [, {path} [, {count}]])",
        description: "Find file in path",
        availability: Availability::Common,
    },
    // Foreground (GUI)
    BuiltinFunction {
        name: "foreground",
        signature: "foreground()",
        description: "Bring Vim window to foreground",
        availability: Availability::VimOnly,
    },
    // Garbage collection
    BuiltinFunction {
        name: "garbagecollect",
        signature: "garbagecollect([{atexit}])",
        description: "Free unused memory",
        availability: Availability::Common,
    },
    // GUI font
    BuiltinFunction {
        name: "getfontname",
        signature: "getfontname([{name}])",
        description: "Return name of current font",
        availability: Availability::VimOnly,
    },
    // Mouse shape
    BuiltinFunction {
        name: "getmouseshape",
        signature: "getmouseshape()",
        description: "Return current mouse shape name",
        availability: Availability::VimOnly,
    },
    // Region functions (Vim 9.1+)
    BuiltinFunction {
        name: "getregion",
        signature: "getregion({pos1}, {pos2} [, {opts}])",
        description: "Return text in region",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getregionpos",
        signature: "getregionpos({pos1}, {pos2} [, {opts}])",
        description: "Return positions of region",
        availability: Availability::Common,
    },
    // Window position
    BuiltinFunction {
        name: "getwinpos",
        signature: "getwinpos([{timeout}])",
        description: "Return [X, Y] of GUI Vim window",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getwinposx",
        signature: "getwinposx()",
        description: "Return X position of GUI Vim window",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getwinposy",
        signature: "getwinposy()",
        description: "Return Y position of GUI Vim window",
        availability: Availability::Common,
    },
    // Glob to regex
    BuiltinFunction {
        name: "glob2regpat",
        signature: "glob2regpat({string})",
        description: "Convert glob pattern to regex",
        availability: Availability::Common,
    },
    // Local directory
    BuiltinFunction {
        name: "haslocaldir",
        signature: "haslocaldir([{winnr} [, {tabnr}]])",
        description: "Return TRUE if local directory is set",
        availability: Availability::Common,
    },
    // Type checking
    BuiltinFunction {
        name: "instanceof",
        signature: "instanceof({object}, {class})",
        description: "Return TRUE if object is instance of class",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "isinf",
        signature: "isinf({expr})",
        description: "Return TRUE if value is infinity",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "isnan",
        signature: "isnan({expr})",
        description: "Return TRUE if value is NaN",
        availability: Availability::Common,
    },
    // Match functions
    BuiltinFunction {
        name: "matchbufline",
        signature: "matchbufline({buf}, {pat}, {lnum}, {end} [, {dict}])",
        description: "Return all matches in buffer lines",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "matchstrlist",
        signature: "matchstrlist({list}, {pat} [, {dict}])",
        description: "Return all matches in list of strings",
        availability: Availability::Common,
    },
    // Popup echo (Vim-only)
    BuiltinFunction {
        name: "popup_findecho",
        signature: "popup_findecho()",
        description: "Return echo popup window ID",
        availability: Availability::VimOnly,
    },
    // Read blob
    BuiltinFunction {
        name: "readblob",
        signature: "readblob({fname} [, {offset} [, {size}]])",
        description: "Read file as blob",
        availability: Availability::Common,
    },
    // Server to client
    BuiltinFunction {
        name: "server2client",
        signature: "server2client({clientid}, {string})",
        description: "Send reply to client",
        availability: Availability::Common,
    },
    // Slice
    BuiltinFunction {
        name: "slice",
        signature: "slice({expr}, {start} [, {end}])",
        description: "Return slice of list or blob",
        availability: Availability::Common,
    },
    // Sound fold
    BuiltinFunction {
        name: "soundfold",
        signature: "soundfold({word})",
        description: "Return sound-folded word",
        availability: Availability::Common,
    },
    // String conversion
    BuiltinFunction {
        name: "string",
        signature: "string({expr})",
        description: "Convert expression to string",
        availability: Availability::Common,
    },
    // String transform
    BuiltinFunction {
        name: "strtrans",
        signature: "strtrans({string})",
        description: "Translate unprintable characters",
        availability: Availability::Common,
    },
    // Swap file functions
    BuiltinFunction {
        name: "swapfilelist",
        signature: "swapfilelist()",
        description: "Return list of swap file names",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "swapinfo",
        signature: "swapinfo({fname})",
        description: "Return info about swap file",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "swapname",
        signature: "swapname({buf})",
        description: "Return swap file name for buffer",
        availability: Availability::Common,
    },
    // Terminal functions (Vim-only, missing)
    BuiltinFunction {
        name: "term_getaltscreen",
        signature: "term_getaltscreen({buf})",
        description: "Return alternate screen flag",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "term_scrape",
        signature: "term_scrape({buf}, {row})",
        description: "Return terminal screen contents",
        availability: Availability::VimOnly,
    },
    // UTF-16 index
    BuiltinFunction {
        name: "utf16idx",
        signature: "utf16idx({string}, {idx} [, {countcc} [, {charidx}]])",
        description: "Return UTF-16 index of byte index",
        availability: Availability::Common,
    },
    // Assert (missing)
    BuiltinFunction {
        name: "assert_equalfile",
        signature: "assert_equalfile({fname1}, {fname2} [, {msg}])",
        description: "Assert two files have equal contents",
        availability: Availability::Common,
    },
    // Internationalization
    BuiltinFunction {
        name: "bindtextdomain",
        signature: "bindtextdomain({package}, {path})",
        description: "Set path for message translations",
        availability: Availability::Common,
    },
    // Wildmenu mode
    BuiltinFunction {
        name: "wildmenumode",
        signature: "wildmenumode()",
        description: "Return TRUE if wildmenu is active",
        availability: Availability::Common,
    },
    // Windows version
    BuiltinFunction {
        name: "windowsversion",
        signature: "windowsversion()",
        description: "Return Windows version string",
        availability: Availability::VimOnly,
    },
];

// ============================================================================
// Ex Commands
// ============================================================================

/// Information about a built-in Ex command
pub struct BuiltinCommand {
    pub name: &'static str,
    pub description: &'static str,
    pub availability: Availability,
}

/// List of commonly used Vim Ex commands
/// Reference: :help ex-cmd-index
pub static BUILTIN_COMMANDS: &[BuiltinCommand] = &[
    // Control flow
    BuiltinCommand {
        name: "if",
        description: "Execute commands when condition is true",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "else",
        description: "Execute commands when 'if' condition is false",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "elseif",
        description: "Execute commands when condition is true",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "endif",
        description: "End 'if' block",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "for",
        description: "Loop over a list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "endfor",
        description: "End 'for' loop",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "while",
        description: "Loop while condition is true",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "endwhile",
        description: "End 'while' loop",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "try",
        description: "Start try block for exception handling",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "catch",
        description: "Catch exceptions",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "finally",
        description: "Execute commands regardless of exception",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "endtry",
        description: "End 'try' block",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "throw",
        description: "Throw an exception",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "break",
        description: "Break out of loop",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "continue",
        description: "Continue loop from start",
        availability: Availability::Common,
    },
    // Function definition
    BuiltinCommand {
        name: "function",
        description: "Define a function",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "endfunction",
        description: "End function definition",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "return",
        description: "Return from function",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "call",
        description: "Call a function",
        availability: Availability::Common,
    },
    // Variable operations
    BuiltinCommand {
        name: "let",
        description: "Assign value to variable",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "const",
        description: "Define a constant",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "unlet",
        description: "Delete variable",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lockvar",
        description: "Lock variable",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "unlockvar",
        description: "Unlock variable",
        availability: Availability::Common,
    },
    // Output
    BuiltinCommand {
        name: "echo",
        description: "Echo expression",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "echom",
        description: "Echo message and save in history",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "echomsg",
        description: "Echo message and save in history",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "echoerr",
        description: "Echo error message",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "echon",
        description: "Echo without newline",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "echohl",
        description: "Set highlight group for echo",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "echowindow",
        description: "Echo in popup window",
        availability: Availability::Common,
    },
    // Mapping commands
    BuiltinCommand {
        name: "map",
        description: "Define key mapping (all modes)",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "nmap",
        description: "Define normal mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "vmap",
        description: "Define visual mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "xmap",
        description: "Define visual (not select) mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "smap",
        description: "Define select mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "imap",
        description: "Define insert mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cmap",
        description: "Define command-line mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "omap",
        description: "Define operator-pending mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lmap",
        description: "Define language mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tmap",
        description: "Define terminal mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "noremap",
        description: "Define non-recursive mapping (all modes)",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "nnoremap",
        description: "Define non-recursive normal mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "vnoremap",
        description: "Define non-recursive visual mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "xnoremap",
        description: "Define non-recursive visual (not select) mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "snoremap",
        description: "Define non-recursive select mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "inoremap",
        description: "Define non-recursive insert mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cnoremap",
        description: "Define non-recursive command-line mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "onoremap",
        description: "Define non-recursive operator-pending mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lnoremap",
        description: "Define non-recursive language mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tnoremap",
        description: "Define non-recursive terminal mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "unmap",
        description: "Remove mapping (all modes)",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "nunmap",
        description: "Remove normal mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "vunmap",
        description: "Remove visual mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "xunmap",
        description: "Remove visual (not select) mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sunmap",
        description: "Remove select mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "iunmap",
        description: "Remove insert mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cunmap",
        description: "Remove command-line mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ounmap",
        description: "Remove operator-pending mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lunmap",
        description: "Remove language mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tunmap",
        description: "Remove terminal mode mapping",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "mapclear",
        description: "Clear all mappings (all modes)",
        availability: Availability::Common,
    },
    // Autocommands
    BuiltinCommand {
        name: "autocmd",
        description: "Define autocommand",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "augroup",
        description: "Define autocommand group",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "doautocmd",
        description: "Execute autocommands",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "doautoall",
        description: "Execute autocommands for all buffers",
        availability: Availability::Common,
    },
    // Settings
    BuiltinCommand {
        name: "set",
        description: "Set option value",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "setlocal",
        description: "Set local option value",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "setglobal",
        description: "Set global option value",
        availability: Availability::Common,
    },
    // Highlighting
    BuiltinCommand {
        name: "highlight",
        description: "Define highlighting",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "syntax",
        description: "Define syntax highlighting",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "colorscheme",
        description: "Load colorscheme",
        availability: Availability::Common,
    },
    // Command definition
    BuiltinCommand {
        name: "command",
        description: "Define user command",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "delcommand",
        description: "Delete user command",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "comclear",
        description: "Clear all user commands",
        availability: Availability::Common,
    },
    // Execution
    BuiltinCommand {
        name: "execute",
        description: "Execute string as Ex command",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "normal",
        description: "Execute normal mode commands",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "source",
        description: "Read and execute commands from file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "runtime",
        description: "Source files from 'runtimepath'",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "finish",
        description: "Stop sourcing current script",
        availability: Availability::Common,
    },
    // Buffer/Window/Tab
    BuiltinCommand {
        name: "edit",
        description: "Edit a file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "enew",
        description: "Edit a new unnamed buffer",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "buffer",
        description: "Go to buffer",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "bdelete",
        description: "Delete buffer",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "bwipeout",
        description: "Wipe out buffer",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "split",
        description: "Split window horizontally",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "vsplit",
        description: "Split window vertically",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "new",
        description: "Create new window with empty buffer",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "vnew",
        description: "Create new vertical window with empty buffer",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "close",
        description: "Close window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "only",
        description: "Close all other windows",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tabnew",
        description: "Create new tab",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tabclose",
        description: "Close tab",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tabnext",
        description: "Go to next tab",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tabprevious",
        description: "Go to previous tab",
        availability: Availability::Common,
    },
    // File operations
    BuiltinCommand {
        name: "write",
        description: "Write buffer to file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "wall",
        description: "Write all buffers",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "quit",
        description: "Quit window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "qall",
        description: "Quit all windows",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "wq",
        description: "Write and quit",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "wqall",
        description: "Write all and quit",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "saveas",
        description: "Save buffer to new file",
        availability: Availability::Common,
    },
    // Search and substitute
    BuiltinCommand {
        name: "substitute",
        description: "Search and replace",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "global",
        description: "Execute command on matching lines",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "vglobal",
        description: "Execute command on non-matching lines",
        availability: Availability::Common,
    },
    // Misc
    BuiltinCommand {
        name: "silent",
        description: "Execute command silently",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "redraw",
        description: "Redraw screen",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sleep",
        description: "Pause execution",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "filetype",
        description: "Set filetype options",
        availability: Availability::Common,
    },
    // Neovim only
    BuiltinCommand {
        name: "lua",
        description: "Execute Lua code",
        availability: Availability::NeovimOnly,
    },
    BuiltinCommand {
        name: "luado",
        description: "Execute Lua for each line",
        availability: Availability::NeovimOnly,
    },
    BuiltinCommand {
        name: "luafile",
        description: "Execute Lua file",
        availability: Availability::NeovimOnly,
    },
    // Vim only
    BuiltinCommand {
        name: "vim9script",
        description: "Start Vim9 script",
        availability: Availability::VimOnly,
    },
    // Additional Ex commands from Vim documentation
    // Symbol commands
    BuiltinCommand {
        name: "!",
        description: "filter lines or execute an external command",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "!!",
        description: "repeat last \":!\" command",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "#",
        description: "same as \":number\"",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "&",
        description: "repeat last \":substitute\"",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "2match",
        description: "define a second match to highlight",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "3match",
        description: "define a third match to highlight",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "<",
        description: "shift lines one 'shiftwidth' left",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "=",
        description: "print the last line number",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: ">",
        description: "shift lines one 'shiftwidth' right",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "@",
        description: "execute contents of a register",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "@@",
        description: "repeat the previous \":@\"",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "~",
        description: "repeat last \":substitute\"",
        availability: Availability::Common,
    },
    // Argument list commands
    BuiltinCommand {
        name: "Next",
        description: "go to previous file in the argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "argadd",
        description: "add items to the argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "argdedupe",
        description: "remove duplicates from the argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "argdelete",
        description: "delete items from the argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "argdo",
        description: "do a command on all items in the argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "argedit",
        description: "add item to the argument list and edit it",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "argglobal",
        description: "define the global argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "arglocal",
        description: "define a local argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "args",
        description: "print the argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "argument",
        description: "go to specific file in the argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "first",
        description: "go to the first file in the argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "last",
        description: "go to the last file in the argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "next",
        description: "go to next file in the argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "previous",
        description: "go to previous file in argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "rewind",
        description: "go to the first file in the argument list",
        availability: Availability::Common,
    },
    // Buffer commands
    BuiltinCommand {
        name: "bNext",
        description: "go to previous buffer in the buffer list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "badd",
        description: "add buffer to the buffer list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ball",
        description: "open a window for each buffer in the buffer list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "balt",
        description: "like \":badd\" but also set the alternate file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "bfirst",
        description: "go to first buffer in the buffer list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "blast",
        description: "go to last buffer in the buffer list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "bmodified",
        description: "go to next buffer in the buffer list that has been modified",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "bnext",
        description: "go to next buffer in the buffer list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "bprevious",
        description: "go to previous buffer in the buffer list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "brewind",
        description: "go to first buffer in the buffer list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "bufdo",
        description: "execute command in each listed buffer",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "buffers",
        description: "list all files in the buffer list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "bunload",
        description: "unload a specific buffer",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "files",
        description: "list all files in the buffer list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ls",
        description: "list all buffers",
        availability: Availability::Common,
    },
    // Window positioning
    BuiltinCommand {
        name: "aboveleft",
        description: "make split window appear left or above",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "belowright",
        description: "make split window appear right or below",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "botright",
        description: "make split window appear at bottom or far right",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "browse",
        description: "use file selection dialog",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "confirm",
        description: "prompt user when confirmation required",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "hide",
        description: "hide current buffer for a command",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "horizontal",
        description: "following window command work horizontally",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "leftabove",
        description: "make split window appear left or above",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "rightbelow",
        description: "make split window appear right or below",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tab",
        description: "create new tab when opening new window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "topleft",
        description: "make split window appear at top or far left",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "vertical",
        description: "make following command split vertically",
        availability: Availability::Common,
    },
    // Quickfix commands
    BuiltinCommand {
        name: "cNext",
        description: "go to previous error",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cNfile",
        description: "go to last error in previous file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cabove",
        description: "go to error above current line",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "caddbuffer",
        description: "add errors from buffer",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "caddexpr",
        description: "add errors from expr",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "caddfile",
        description: "add error message to current quickfix list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cafter",
        description: "go to error after current cursor",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cbefore",
        description: "go to error before current cursor",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cbelow",
        description: "go to error below current line",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cbottom",
        description: "scroll to the bottom of the quickfix window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cbuffer",
        description: "parse error messages and jump to first error",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cc",
        description: "go to specific error",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cclose",
        description: "close quickfix window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cdo",
        description: "execute command in each valid error list entry",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cexpr",
        description: "read errors from expr and jump to first",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cfdo",
        description: "execute command in each file in error list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cfile",
        description: "read file with error messages and jump to first",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cfirst",
        description: "go to the specified error, default first one",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cgetbuffer",
        description: "get errors from buffer",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cgetexpr",
        description: "get errors from expr",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cgetfile",
        description: "read file with error messages",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "chistory",
        description: "list the error lists",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "clast",
        description: "go to the specified error, default last one",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "clist",
        description: "list all errors",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cnewer",
        description: "go to newer error list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cnext",
        description: "go to next error",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cnfile",
        description: "go to first error in next file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "colder",
        description: "go to older error list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "copen",
        description: "open quickfix window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cpfile",
        description: "go to last error in previous file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cprevious",
        description: "go to previous error",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cquit",
        description: "quit Vim with an error code",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "crewind",
        description: "go to the specified error, default first one",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cwindow",
        description: "open or close quickfix window",
        availability: Availability::Common,
    },
    // Location list commands
    BuiltinCommand {
        name: "lNext",
        description: "go to previous entry in location list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lNfile",
        description: "go to last entry in previous file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "labove",
        description: "go to location above current line",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "laddbuffer",
        description: "add locations from buffer",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "laddexpr",
        description: "add locations from expr",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "laddfile",
        description: "add locations to current location list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lafter",
        description: "go to location after current cursor",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lbefore",
        description: "go to location before current cursor",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lbelow",
        description: "go to location below current line",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lbottom",
        description: "scroll to the bottom of the location window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lbuffer",
        description: "parse locations and jump to first location",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lclose",
        description: "close location window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ldo",
        description: "execute command in valid location list entries",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lexpr",
        description: "read locations from expr and jump to first",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lfdo",
        description: "execute command in each file in location list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lfile",
        description: "read file with locations and jump to first",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lfirst",
        description: "go to the specified location, default first one",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lgetbuffer",
        description: "get locations from buffer",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lgetexpr",
        description: "get locations from expr",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lgetfile",
        description: "read file with locations",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lgrep",
        description: "run 'grepprg' and jump to first match",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lgrepadd",
        description: "like :grep, but append to current list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lhelpgrep",
        description: "like \":helpgrep\" but uses location list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lhistory",
        description: "list the location lists",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ll",
        description: "go to specific location",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "llast",
        description: "go to the specified location, default last one",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "llist",
        description: "list all locations",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lmake",
        description: "execute external command 'makeprg' and parse",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lnewer",
        description: "go to newer location list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lnext",
        description: "go to next location",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lnfile",
        description: "go to first location in next file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lolder",
        description: "go to older location list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lopen",
        description: "open location window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lpfile",
        description: "go to last location in previous file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lprevious",
        description: "go to previous location",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lrewind",
        description: "go to the specified location, default first one",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ltag",
        description: "jump to tag and add matching tags to the location list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lvimgrep",
        description: "search for pattern in files",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lvimgrepadd",
        description: "like :vimgrep, but append to current list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lwindow",
        description: "open or close location window",
        availability: Availability::Common,
    },
    // Directory commands
    BuiltinCommand {
        name: "cd",
        description: "change directory",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "chdir",
        description: "change directory",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lcd",
        description: "change directory locally",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lchdir",
        description: "change directory locally",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "pwd",
        description: "print current directory",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tcd",
        description: "change directory for tab page",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tchdir",
        description: "change directory for tab page",
        availability: Availability::Common,
    },
    // Text manipulation
    BuiltinCommand {
        name: "Print",
        description: "print lines",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "append",
        description: "append text",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "center",
        description: "format lines at the center",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "change",
        description: "replace a line or series of lines",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "copy",
        description: "copy lines",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "delete",
        description: "delete lines",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "insert",
        description: "insert text",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "join",
        description: "join lines",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "left",
        description: "left align lines",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "list",
        description: "print lines",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "move",
        description: "move lines",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "number",
        description: "print lines with line number",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "print",
        description: "print lines",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "put",
        description: "insert contents of register in the text",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "read",
        description: "read file into the text",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "retab",
        description: "change tab size",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "right",
        description: "right align text",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sort",
        description: "sort lines",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "t",
        description: "same as \":copy\"",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "uniq",
        description: "uniq lines",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "yank",
        description: "yank lines into a register",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "z",
        description: "print some lines",
        availability: Availability::Common,
    },
    // Undo/Redo
    BuiltinCommand {
        name: "earlier",
        description: "go to older change, undo",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "later",
        description: "go to newer change, redo",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "redo",
        description: "redo one undone change",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "rundo",
        description: "read undo information from a file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "undo",
        description: "undo last change(s)",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "undojoin",
        description: "join next change with previous undo block",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "undolist",
        description: "list leafs of the undo tree",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "wundo",
        description: "write undo information to a file",
        availability: Availability::Common,
    },
    // Mark commands
    BuiltinCommand {
        name: "changes",
        description: "print the change list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "clearjumps",
        description: "clear the jump list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "delmarks",
        description: "delete marks",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "jumps",
        description: "print the jump list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "k",
        description: "set a mark",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "mark",
        description: "set a mark",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "marks",
        description: "list all marks",
        availability: Availability::Common,
    },
    // Register/display
    BuiltinCommand {
        name: "ascii",
        description: "print ascii value of character under the cursor",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "display",
        description: "display registers",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "registers",
        description: "display the contents of registers",
        availability: Availability::Common,
    },
    // Tag commands
    BuiltinCommand {
        name: "cscope",
        description: "execute cscope command",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cstag",
        description: "use cscope to jump to a tag",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lcscope",
        description: "like \":cscope\" but uses location list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "pop",
        description: "jump to older entry in tag stack",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ptNext",
        description: ":tNext in preview window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ptag",
        description: "show tag in preview window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ptfirst",
        description: ":trewind in preview window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ptjump",
        description: ":tjump and show tag in preview window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ptlast",
        description: ":tlast in preview window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ptnext",
        description: ":tnext in preview window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ptprevious",
        description: ":tprevious in preview window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ptrewind",
        description: ":trewind in preview window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ptselect",
        description: ":tselect and show tag in preview window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "scscope",
        description: "split window and execute cscope command",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "stag",
        description: "split window and jump to a tag",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "stjump",
        description: "do \":tjump\" and split window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "stselect",
        description: "do \":tselect\" and split window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tNext",
        description: "jump to previous matching tag",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tag",
        description: "jump to tag",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tags",
        description: "show the contents of the tag stack",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tfirst",
        description: "jump to first matching tag",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tjump",
        description: "like \":tselect\", but jump directly when there is only one match",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tlast",
        description: "jump to last matching tag",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tnext",
        description: "jump to next matching tag",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tprevious",
        description: "jump to previous matching tag",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "trewind",
        description: "jump to first matching tag",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tselect",
        description: "list matching tags and select one",
        availability: Availability::Common,
    },
    // Tab commands
    BuiltinCommand {
        name: "tabNext",
        description: "go to previous tab page",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tabdo",
        description: "execute command in each tab page",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tabedit",
        description: "edit a file in a new tab page",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tabfind",
        description: "find file in 'path', edit it in a new tab page",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tabfirst",
        description: "go to first tab page",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tablast",
        description: "go to last tab page",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tabmove",
        description: "move tab page to other position",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tabonly",
        description: "close all tab pages except the current one",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tabrewind",
        description: "go to first tab page",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tabs",
        description: "list the tab pages and what they contain",
        availability: Availability::Common,
    },
    // Split window commands
    BuiltinCommand {
        name: "all",
        description: "open a window for each file in the argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "pbuffer",
        description: "edit buffer in the preview window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "pclose",
        description: "close preview window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "pedit",
        description: "edit file in the preview window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ppop",
        description: "\":pop\" in preview window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "psearch",
        description: "like \":ijump\" but shows match in preview window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "resize",
        description: "change current window height",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sNext",
        description: "split window and go to previous file in argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sall",
        description: "open a window for each file in argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sargument",
        description: "split window and go to specific file in argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sbNext",
        description: "split window and go to previous file in the buffer list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sball",
        description: "open a window for each file in the buffer list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sbfirst",
        description: "split window and go to first file in the buffer list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sblast",
        description: "split window and go to last file in buffer list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sbmodified",
        description: "split window and go to modified file in the buffer list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sbnext",
        description: "split window and go to next file in the buffer list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sbprevious",
        description: "split window and go to previous file in the buffer list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sbrewind",
        description: "split window and go to first file in the buffer list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sbuffer",
        description: "split window and go to specific file in the buffer list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sfind",
        description: "split current window and edit file in 'path'",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sfirst",
        description: "split window and go to first file in the argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "slast",
        description: "split window and go to last file in the argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "snext",
        description: "split window and go to next file in the argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sprevious",
        description: "split window and go to previous file in the argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "srewind",
        description: "split window and go to first file in the argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sunhide",
        description: "same as \":unhide\"",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sview",
        description: "split window and edit file read-only",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "unhide",
        description: "open a window for each loaded file in the buffer list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "wincmd",
        description: "execute a Window (CTRL-W) command",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "windo",
        description: "execute command in each window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "winpos",
        description: "get or set window position",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "winsize",
        description: "get or set window size (obsolete)",
        availability: Availability::Common,
    },
    // File/session commands
    BuiltinCommand {
        name: "X",
        description: "ask for encryption key",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "drop",
        description: "jump to window editing file or edit file in current window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ex",
        description: "same as \":edit\"",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "exit",
        description: "same as \":xit\"",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "file",
        description: "show or set the current file name",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "find",
        description: "find file in 'path' and edit it",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "loadview",
        description: "load view for current window from a file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "mkexrc",
        description: "write current mappings and settings to a file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "mksession",
        description: "write session info to a file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "mkview",
        description: "write view of current window to a file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "mkvimrc",
        description: "write current mappings and settings to a file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "oldfiles",
        description: "list files that have marks in the viminfo file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "preserve",
        description: "write all text to swap file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "quitall",
        description: "quit Vim",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "recover",
        description: "recover a file from a swap file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "rviminfo",
        description: "read from viminfo file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "swapname",
        description: "show the name of the current swap file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "update",
        description: "write buffer if modified",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "view",
        description: "edit a file read-only",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "visual",
        description: "same as \":edit\", but turns off \"Ex\" mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "wNext",
        description: "write to a file and go to previous file in argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "wnext",
        description: "write to a file and go to next file in argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "wprevious",
        description: "write to a file and go to previous file in argument list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "wviminfo",
        description: "write to viminfo file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "xall",
        description: "same as \":wqall\"",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "xit",
        description: "write if buffer changed and close window",
        availability: Availability::Common,
    },
    // Diff commands
    BuiltinCommand {
        name: "diffget",
        description: "remove differences in current buffer",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "diffoff",
        description: "switch off diff mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "diffpatch",
        description: "apply a patch and show differences",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "diffput",
        description: "remove differences in other buffer",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "diffsplit",
        description: "show differences with another file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "diffthis",
        description: "make current window a diff window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "diffupdate",
        description: "update 'diff' buffers",
        availability: Availability::Common,
    },
    // Fold commands
    BuiltinCommand {
        name: "fold",
        description: "create a fold",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "foldclose",
        description: "close folds",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "folddoclosed",
        description: "execute command on lines in a closed fold",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "folddoopen",
        description: "execute command on lines not in a closed fold",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "foldopen",
        description: "open folds",
        availability: Availability::Common,
    },
    // Abbreviation commands
    BuiltinCommand {
        name: "abbreviate",
        description: "enter abbreviation",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "abclear",
        description: "remove all abbreviations",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cabbrev",
        description: "like \":abbreviate\" but for Command-line mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cabclear",
        description: "clear all abbreviations for Command-line mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cnoreabbrev",
        description: "like \":noreabbrev\" but for Command-line mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cunabbrev",
        description: "like \":unabbrev\" but for Command-line mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "iabbrev",
        description: "like \":abbrev\" but for Insert mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "iabclear",
        description: "like \":abclear\" but for Insert mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "inoreabbrev",
        description: "like \":noreabbrev\" but for Insert mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "iunabbrev",
        description: "like \":unabbrev\" but for Insert mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "noreabbrev",
        description: "enter an abbreviation that will not be remapped",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "unabbreviate",
        description: "remove abbreviation",
        availability: Availability::Common,
    },
    // Menu commands
    BuiltinCommand {
        name: "amenu",
        description: "enter new menu item for all modes",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "anoremenu",
        description: "enter a new menu for all modes that will not be remapped",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "aunmenu",
        description: "remove menu for all modes",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cmenu",
        description: "add menu for Command-line mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cnoremenu",
        description: "like \":noremenu\" but for Command-line mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "cunmenu",
        description: "remove menu for Command-line mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "emenu",
        description: "execute a menu by name",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "imenu",
        description: "add menu for Insert mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "inoremenu",
        description: "like \":noremenu\" but for Insert mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "iunmenu",
        description: "remove menu for Insert mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "menu",
        description: "enter a new menu item",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "menutranslate",
        description: "add a menu translation item",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "nmenu",
        description: "add menu for Normal mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "nnoremenu",
        description: "like \":noremenu\" but for Normal mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "noremenu",
        description: "enter a menu that will not be remapped",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "nunmenu",
        description: "remove menu for Normal mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "omenu",
        description: "add menu for Operator-pending mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "onoremenu",
        description: "like \":noremenu\" but for Operator-pending mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ounmenu",
        description: "remove menu for Operator-pending mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "popup",
        description: "popup a menu by name",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "smenu",
        description: "add menu for Select mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "snoremenu",
        description: "like \":noremenu\" but for Select mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sunmenu",
        description: "remove menu for Select mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tearoff",
        description: "tear-off a menu",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tlmenu",
        description: "add menu for Terminal-Job mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tlnoremenu",
        description: "like \":noremenu\" but for Terminal-Job mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tlunmenu",
        description: "remove menu for Terminal-Job mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tmenu",
        description: "define menu tooltip",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tunmenu",
        description: "remove menu tooltip",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "unmenu",
        description: "remove menu",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "vmenu",
        description: "add menu for Visual+Select mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "vnoremenu",
        description: "like \":noremenu\" but for Visual+Select mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "vunmenu",
        description: "remove menu for Visual+Select mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "xmenu",
        description: "add menu for Visual mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "xnoremenu",
        description: "like \":noremenu\" but for Visual mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "xunmenu",
        description: "remove menu for Visual mode",
        availability: Availability::Common,
    },
    // Map clear commands
    BuiltinCommand {
        name: "cmapclear",
        description: "clear all mappings for Command-line mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "imapclear",
        description: "like \":mapclear\" but for Insert mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lmapclear",
        description: "like \":mapclear!\" but includes Lang-Arg mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "nmapclear",
        description: "clear all mappings for Normal mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "omapclear",
        description: "remove all mappings for Operator-pending mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "smapclear",
        description: "remove all mappings for Select mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tmapclear",
        description: "remove all mappings for Terminal-Job mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "vmapclear",
        description: "remove all mappings for Visual+Select mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "xmapclear",
        description: "remove all mappings for Visual mode",
        availability: Availability::Common,
    },
    // Help commands
    BuiltinCommand {
        name: "exusage",
        description: "overview of Ex commands",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "help",
        description: "open a help window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "helpclose",
        description: "close one help window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "helpfind",
        description: "dialog to open a help window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "helpgrep",
        description: "like \":grep\" but searches help files",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "helptags",
        description: "generate help tags for a directory",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "intro",
        description: "print the introductory message",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "messages",
        description: "view previously displayed messages",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "version",
        description: "print version number and other info",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "viusage",
        description: "overview of Normal mode commands",
        availability: Availability::Common,
    },
    // Search/grep commands
    BuiltinCommand {
        name: "grep",
        description: "run 'grepprg' and jump to first match",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "grepadd",
        description: "like :grep, but append to current list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "make",
        description: "execute external command 'makeprg' and parse",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "nohlsearch",
        description: "suspend 'hlsearch' highlighting",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "smagic",
        description: ":substitute with 'magic'",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "snomagic",
        description: ":substitute with 'nomagic'",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "vimgrep",
        description: "search for pattern in files",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "vimgrepadd",
        description: "like :vimgrep, but append to current list",
        availability: Availability::Common,
    },
    // Identifier search
    BuiltinCommand {
        name: "checkpath",
        description: "list included files",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "djump",
        description: "jump to #define",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "dl",
        description: "short for :delete with the 'l' flag",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "dlist",
        description: "list #defines",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "dp",
        description: "short for :delete with the 'p' flag",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "dsearch",
        description: "list one #define",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "dsplit",
        description: "split window and jump to #define",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ijump",
        description: "jump to definition of identifier",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ilist",
        description: "list lines where identifier matches",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "iput",
        description: "like :put, but adjust the indent",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "isearch",
        description: "list one line where identifier matches",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "isplit",
        description: "split window and jump to definition of identifier",
        availability: Availability::Common,
    },
    // Spell commands
    BuiltinCommand {
        name: "mkspell",
        description: "produce .spl spell file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "spelldump",
        description: "split window and fill with all correct words",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "spellgood",
        description: "add good word for spelling",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "spellinfo",
        description: "show info about loaded spell files",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "spellrare",
        description: "add rare word for spelling",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "spellrepall",
        description: "replace all bad words like last |z=|",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "spellundo",
        description: "remove good or bad word",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "spellwrong",
        description: "add spelling mistake",
        availability: Availability::Common,
    },
    // Debug commands
    BuiltinCommand {
        name: "breakadd",
        description: "add a debugger breakpoint",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "breakdel",
        description: "delete a debugger breakpoint",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "breaklist",
        description: "list debugger breakpoints",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "debug",
        description: "run a command in debugging mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "debuggreedy",
        description: "read debug mode commands from normal input",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "profile",
        description: "profiling functions and scripts",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "profdel",
        description: "stop profiling a function or script",
        availability: Availability::Common,
    },
    // Script commands
    BuiltinCommand {
        name: "delfunction",
        description: "delete a user function",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "scriptencoding",
        description: "encoding used in sourced Vim script",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "scriptnames",
        description: "list names of all sourced Vim scripts",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "scriptversion",
        description: "version of Vim script used",
        availability: Availability::Common,
    },
    // Misc commands
    BuiltinCommand {
        name: "behave",
        description: "set mouse and selection behavior",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "checktime",
        description: "check timestamp of loaded buffers",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "clipreset",
        description: "reset 'clipmethod'",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "compiler",
        description: "do settings for a specific compiler",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "defer",
        description: "call function when current function is done",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "digraphs",
        description: "show or enter digraphs",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "eval",
        description: "evaluate an expression and discard the result",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "filter",
        description: "filter output of following command",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "fixdel",
        description: "set key code of <Del>",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "goto",
        description: "go to byte in the buffer",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "hardcopy",
        description: "send text to the printer",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "history",
        description: "print a history list",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "keepalt",
        description: "following command keeps the alternate file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "keepjumps",
        description: "following command keeps jumplist and marks",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "keepmarks",
        description: "following command keeps marks where they are",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "keeppatterns",
        description: "following command keeps search pattern history",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "language",
        description: "set the language (locale)",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "legacy",
        description: "make following command use legacy script syntax",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "loadkeymap",
        description: "load the following keymaps until EOF",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lockmarks",
        description: "following command keeps marks where they are",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "lsp",
        description: "LSP client command",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "match",
        description: "define a match to highlight",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "mode",
        description: "show or change the screen mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "noautocmd",
        description: "following commands don't trigger autocommands",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "noswapfile",
        description: "following commands don't create a swap file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "open",
        description: "start open mode (not implemented)",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "options",
        description: "open the options-window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "ownsyntax",
        description: "set new local syntax highlight for this window",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "packadd",
        description: "add a plugin from 'packpath'",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "packloadall",
        description: "load all packages under 'packpath'",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "range",
        description: "go to last line in {range}",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "redir",
        description: "redirect messages to a file or register",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "redrawstatus",
        description: "force a redraw of the status line(s)",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "redrawtabline",
        description: "force a redraw of the tabline",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "redrawtabpanel",
        description: "force a redraw of the tabpanel",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "restart",
        description: "restart Vim",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sandbox",
        description: "execute a command in the sandbox",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "setfiletype",
        description: "set 'filetype', unless it was set already",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "shell",
        description: "escape to a shell",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sign",
        description: "manipulate signs",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "sleep!",
        description: "do nothing for a few seconds, without the cursor visible",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "smile",
        description: "make the user happy",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "star",
        description: "use the last Visual area, like :'<,'>",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "startgreplace",
        description: "start Virtual Replace mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "startinsert",
        description: "start Insert mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "startreplace",
        description: "start Replace mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "stop",
        description: "suspend the editor or escape to a shell",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "stopinsert",
        description: "stop Insert mode",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "suspend",
        description: "same as \":stop\"",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "syncbind",
        description: "sync scroll binding",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "syntime",
        description: "measure syntax highlighting speed",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "unsilent",
        description: "run a command not silently",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "verbose",
        description: "execute command with 'verbose' set",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "echoconsole",
        description: "like :echomsg but write to stdout",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "fclose",
        description: "close file dialog",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "gui",
        description: "start the GUI",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "gvim",
        description: "start the GUI",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "promptfind",
        description: "open GUI dialog for searching",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "promptrepl",
        description: "open GUI dialog for search/replace",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "simalt",
        description: "Win32 GUI: simulate Windows ALT key",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "wlrestore",
        description: "restore the Wayland compositor connection",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "xrestore",
        description: "restores the X server connection",
        availability: Availability::Common,
    },
    // Scripting languages - Perl
    BuiltinCommand {
        name: "perl",
        description: "execute Perl command",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "perldo",
        description: "execute Perl command for each line",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "perlfile",
        description: "execute Perl script file",
        availability: Availability::Common,
    },
    // Scripting languages - Python
    BuiltinCommand {
        name: "py3",
        description: "execute Python 3 command",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "py3do",
        description: "execute Python 3 command for each line",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "py3file",
        description: "execute Python 3 script file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "pydo",
        description: "execute Python command for each line",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "pyfile",
        description: "execute Python script file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "python",
        description: "execute Python command",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "python3",
        description: "same as :py3",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "pythonx",
        description: "same as :pyx",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "pyx",
        description: "execute python_x command",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "pyxdo",
        description: "execute python_x command for each line",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "pyxfile",
        description: "execute python_x script file",
        availability: Availability::Common,
    },
    // Scripting languages - Ruby
    BuiltinCommand {
        name: "ruby",
        description: "execute Ruby command",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "rubydo",
        description: "execute Ruby command for each line",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "rubyfile",
        description: "execute Ruby script file",
        availability: Availability::Common,
    },
    // Scripting languages - Tcl
    BuiltinCommand {
        name: "tcl",
        description: "execute Tcl command",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tcldo",
        description: "execute Tcl command for each line",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "tclfile",
        description: "execute Tcl script file",
        availability: Availability::Common,
    },
    // Scripting languages - MzScheme
    BuiltinCommand {
        name: "mzfile",
        description: "execute MzScheme script file",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "mzscheme",
        description: "execute MzScheme command",
        availability: Availability::Common,
    },
    // Netbeans
    BuiltinCommand {
        name: "nbclose",
        description: "close the current Netbeans session",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "nbkey",
        description: "pass a key to Netbeans",
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "nbstart",
        description: "start a new Netbeans session",
        availability: Availability::Common,
    },
    // Neovim only commands
    BuiltinCommand {
        name: "checkhealth",
        description: "run health checks",
        availability: Availability::NeovimOnly,
    },
    BuiltinCommand {
        name: "terminal",
        description: "open a terminal window",
        availability: Availability::NeovimOnly,
    },
    BuiltinCommand {
        name: "rshada",
        description: "read from shada file",
        availability: Availability::NeovimOnly,
    },
    BuiltinCommand {
        name: "wshada",
        description: "write to shada file",
        availability: Availability::NeovimOnly,
    },
    BuiltinCommand {
        name: "detach",
        description: "detach the current UI",
        availability: Availability::NeovimOnly,
    },
    BuiltinCommand {
        name: "trust",
        description: "manage trusted files",
        availability: Availability::NeovimOnly,
    },
    // Vim9 only commands
    BuiltinCommand {
        name: "abstract",
        description: "declare a Vim9 abstract class",
        availability: Availability::VimOnly,
    },
    BuiltinCommand {
        name: "class",
        description: "start of a class declaration",
        availability: Availability::VimOnly,
    },
    BuiltinCommand {
        name: "def",
        description: "define a Vim9 user function",
        availability: Availability::VimOnly,
    },
    BuiltinCommand {
        name: "defcompile",
        description: "compile Vim9 user functions in current script",
        availability: Availability::VimOnly,
    },
    BuiltinCommand {
        name: "disassemble",
        description: "disassemble Vim9 user function",
        availability: Availability::VimOnly,
    },
    BuiltinCommand {
        name: "endclass",
        description: "end of a class declaration",
        availability: Availability::VimOnly,
    },
    BuiltinCommand {
        name: "enddef",
        description: "end of a user function started with :def",
        availability: Availability::VimOnly,
    },
    BuiltinCommand {
        name: "endenum",
        description: "end of an enum declaration",
        availability: Availability::VimOnly,
    },
    BuiltinCommand {
        name: "endinterface",
        description: "end of an interface declaration",
        availability: Availability::VimOnly,
    },
    BuiltinCommand {
        name: "enum",
        description: "start of an enum declaration",
        availability: Availability::VimOnly,
    },
    BuiltinCommand {
        name: "export",
        description: "Vim9: export an item from a script",
        availability: Availability::VimOnly,
    },
    BuiltinCommand {
        name: "final",
        description: "declare an immutable variable in Vim9",
        availability: Availability::VimOnly,
    },
    BuiltinCommand {
        name: "import",
        description: "Vim9: import an item from another script",
        availability: Availability::VimOnly,
    },
    BuiltinCommand {
        name: "interface",
        description: "start of an interface declaration",
        availability: Availability::VimOnly,
    },
    BuiltinCommand {
        name: "public",
        description: "prefix for a class or object member",
        availability: Availability::VimOnly,
    },
    BuiltinCommand {
        name: "static",
        description: "prefix for a class member or function",
        availability: Availability::VimOnly,
    },
    BuiltinCommand {
        name: "this",
        description: "prefix for an object member during declaration",
        availability: Availability::VimOnly,
    },
    BuiltinCommand {
        name: "type",
        description: "create a type alias",
        availability: Availability::VimOnly,
    },
    BuiltinCommand {
        name: "var",
        description: "variable declaration in Vim9",
        availability: Availability::VimOnly,
    },
    BuiltinCommand {
        name: "vim9cmd",
        description: "make following command use Vim9 script syntax",
        availability: Availability::VimOnly,
    },
];

// ============================================================================
// Autocmd Events
// ============================================================================

/// Information about an autocmd event
pub struct AutocmdEvent {
    pub name: &'static str,
    pub description: &'static str,
    pub availability: Availability,
}

/// List of autocmd events
/// Reference: :help autocmd-events
pub static AUTOCMD_EVENTS: &[AutocmdEvent] = &[
    // Reading
    AutocmdEvent {
        name: "BufNewFile",
        description: "Starting to edit a file that doesn't exist",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "BufRead",
        description: "Starting to edit a new buffer (after reading)",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "BufReadPost",
        description: "After reading a buffer",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "BufReadPre",
        description: "Before reading a buffer",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "BufReadCmd",
        description: "Before reading a buffer (replaces read)",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "FileReadPost",
        description: "After reading a file with :read",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "FileReadPre",
        description: "Before reading a file with :read",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "StdinReadPost",
        description: "After reading from stdin",
        availability: Availability::Common,
    },
    // Writing
    AutocmdEvent {
        name: "BufWrite",
        description: "Starting to write the buffer",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "BufWritePost",
        description: "After writing the buffer",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "BufWritePre",
        description: "Before writing the buffer",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "BufWriteCmd",
        description: "Before writing buffer (replaces write)",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "FileWritePost",
        description: "After writing with :write",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "FileWritePre",
        description: "Before writing with :write",
        availability: Availability::Common,
    },
    // Buffer
    AutocmdEvent {
        name: "BufAdd",
        description: "After adding a buffer to the list",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "BufDelete",
        description: "Before deleting a buffer",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "BufEnter",
        description: "After entering a buffer",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "BufLeave",
        description: "Before leaving a buffer",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "BufWinEnter",
        description: "After buffer is displayed in a window",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "BufWinLeave",
        description: "Before buffer is removed from window",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "BufUnload",
        description: "Before unloading a buffer",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "BufHidden",
        description: "Before buffer becomes hidden",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "BufNew",
        description: "After creating a new buffer",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "BufModifiedSet",
        description: "After 'modified' option changes",
        availability: Availability::Common,
    },
    // File type
    AutocmdEvent {
        name: "FileType",
        description: "When 'filetype' option is set",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "Syntax",
        description: "When 'syntax' option is set",
        availability: Availability::Common,
    },
    // Window
    AutocmdEvent {
        name: "WinEnter",
        description: "After entering a window",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "WinLeave",
        description: "Before leaving a window",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "WinNew",
        description: "After creating a new window",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "WinClosed",
        description: "After closing a window",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "WinScrolled",
        description: "After window scrolled or resized",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "WinResized",
        description: "After window size changed",
        availability: Availability::Common,
    },
    // Tab
    AutocmdEvent {
        name: "TabEnter",
        description: "After entering a tab page",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "TabLeave",
        description: "Before leaving a tab page",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "TabNew",
        description: "After creating a new tab page",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "TabClosed",
        description: "After closing a tab page",
        availability: Availability::Common,
    },
    // Cursor
    AutocmdEvent {
        name: "CursorHold",
        description: "Cursor hasn't moved for 'updatetime'",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "CursorHoldI",
        description: "Cursor hasn't moved in Insert mode",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "CursorMoved",
        description: "After cursor moved in Normal mode",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "CursorMovedI",
        description: "After cursor moved in Insert mode",
        availability: Availability::Common,
    },
    // Insert mode
    AutocmdEvent {
        name: "InsertEnter",
        description: "Just before entering Insert mode",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "InsertLeave",
        description: "Just after leaving Insert mode",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "InsertLeavePre",
        description: "Just before leaving Insert mode",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "InsertCharPre",
        description: "Before inserting a character",
        availability: Availability::Common,
    },
    // Text changes
    AutocmdEvent {
        name: "TextChanged",
        description: "After text changed in Normal mode",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "TextChangedI",
        description: "After text changed in Insert mode",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "TextChangedP",
        description: "After text changed during completion",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "TextChangedT",
        description: "After text changed in Terminal mode",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "TextYankPost",
        description: "After yanking or deleting text",
        availability: Availability::Common,
    },
    // Vim events
    AutocmdEvent {
        name: "VimEnter",
        description: "After Vim startup",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "VimLeave",
        description: "Before exiting Vim",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "VimLeavePre",
        description: "Before exiting Vim (before VimLeave)",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "VimResized",
        description: "After Vim window size changed",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "VimResume",
        description: "After Vim resumed from suspend",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "VimSuspend",
        description: "Before Vim is suspended",
        availability: Availability::Common,
    },
    // Completion
    AutocmdEvent {
        name: "CompleteDone",
        description: "After completion is done",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "CompleteDonePre",
        description: "After completion, before clearing info",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "CompleteChanged",
        description: "After completion menu item changed",
        availability: Availability::Common,
    },
    // Command line
    AutocmdEvent {
        name: "CmdlineEnter",
        description: "After entering command-line mode",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "CmdlineLeave",
        description: "Before leaving command-line mode",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "CmdlineChanged",
        description: "After command-line text changed",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "CmdwinEnter",
        description: "After entering command-line window",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "CmdwinLeave",
        description: "Before leaving command-line window",
        availability: Availability::Common,
    },
    // Misc
    AutocmdEvent {
        name: "ColorScheme",
        description: "After loading a colorscheme",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "ColorSchemePre",
        description: "Before loading a colorscheme",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "DirChanged",
        description: "After current directory changed",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "DirChangedPre",
        description: "Before current directory changed",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "FocusGained",
        description: "Vim got input focus",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "FocusLost",
        description: "Vim lost input focus",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "OptionSet",
        description: "After option value changed",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "QuickFixCmdPre",
        description: "Before quickfix command",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "QuickFixCmdPost",
        description: "After quickfix command",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "SessionLoadPost",
        description: "After loading session file",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "ShellCmdPost",
        description: "After executing shell command",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "SourcePre",
        description: "Before sourcing a script",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "SourcePost",
        description: "After sourcing a script",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "SourceCmd",
        description: "When sourcing (replaces source)",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "User",
        description: "User-defined autocommand",
        availability: Availability::Common,
    },
    // Neovim only
    AutocmdEvent {
        name: "LspAttach",
        description: "After LSP client attaches to buffer",
        availability: Availability::NeovimOnly,
    },
    AutocmdEvent {
        name: "LspDetach",
        description: "After LSP client detaches from buffer",
        availability: Availability::NeovimOnly,
    },
    AutocmdEvent {
        name: "LspRequest",
        description: "After LSP request is started",
        availability: Availability::NeovimOnly,
    },
    AutocmdEvent {
        name: "LspProgress",
        description: "When LSP progress is updated",
        availability: Availability::NeovimOnly,
    },
    AutocmdEvent {
        name: "LspTokenUpdate",
        description: "After LSP semantic token updated",
        availability: Availability::NeovimOnly,
    },
    AutocmdEvent {
        name: "TermOpen",
        description: "After opening terminal buffer",
        availability: Availability::NeovimOnly,
    },
    AutocmdEvent {
        name: "TermClose",
        description: "After closing terminal buffer",
        availability: Availability::NeovimOnly,
    },
    AutocmdEvent {
        name: "TermEnter",
        description: "After entering Terminal mode",
        availability: Availability::NeovimOnly,
    },
    AutocmdEvent {
        name: "TermLeave",
        description: "After leaving Terminal mode",
        availability: Availability::NeovimOnly,
    },
    AutocmdEvent {
        name: "UIEnter",
        description: "After UI connects",
        availability: Availability::NeovimOnly,
    },
    AutocmdEvent {
        name: "UILeave",
        description: "After UI disconnects",
        availability: Availability::NeovimOnly,
    },
    AutocmdEvent {
        name: "RecordingEnter",
        description: "When starting to record a macro",
        availability: Availability::NeovimOnly,
    },
    AutocmdEvent {
        name: "RecordingLeave",
        description: "When stopping to record a macro",
        availability: Availability::NeovimOnly,
    },
    // Vim only
    AutocmdEvent {
        name: "SafeState",
        description: "Nothing pending, going to wait for input",
        availability: Availability::VimOnly,
    },
    AutocmdEvent {
        name: "SafeStateAgain",
        description: "SafeState triggered again",
        availability: Availability::VimOnly,
    },
    // Additional events
    AutocmdEvent {
        name: "BufCreate",
        description: "After creating a new buffer (alias for BufAdd)",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "BufFilePost",
        description: "After changing the name of the current buffer",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "BufFilePre",
        description: "Before changing the name of the current buffer",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "BufWipeout",
        description: "Before completely deleting a buffer",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "CmdUndefined",
        description: "When a user command is used but not defined",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "CmdlineLeavePre",
        description: "Just before leaving the command line",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "CursorMovedC",
        description: "After cursor moved in command-line mode",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "DiffUpdated",
        description: "After diffs have been updated",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "EncodingChanged",
        description: "After 'encoding' option has been changed",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "ExitPre",
        description: "When using a command that may make Vim exit",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "FileAppendCmd",
        description: "Before appending to a file (replaces append)",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "FileAppendPost",
        description: "After appending to a file",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "FileAppendPre",
        description: "Before appending to a file",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "FileChangedRO",
        description: "Before making first change to read-only file",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "FileChangedShell",
        description: "When Vim notices a file changed since editing started",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "FileChangedShellPost",
        description: "After handling a file changed since editing started",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "FileReadCmd",
        description: "Before reading a file with :read (replaces read)",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "FileWriteCmd",
        description: "Before writing a file (replaces write)",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "FilterReadPost",
        description: "After reading a file from a filter command",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "FilterReadPre",
        description: "Before reading a file from a filter command",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "FilterWritePost",
        description: "After writing a file for a filter command",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "FilterWritePre",
        description: "Before writing a file for a filter command",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "FuncUndefined",
        description: "When a user function is used but not defined",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "GUIEnter",
        description: "After starting the GUI successfully",
        availability: Availability::VimOnly,
    },
    AutocmdEvent {
        name: "GUIFailed",
        description: "After starting the GUI failed",
        availability: Availability::VimOnly,
    },
    AutocmdEvent {
        name: "InsertChange",
        description: "When typing <Insert> in Insert or Replace mode",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "KeyInputPre",
        description: "Just before a key is processed",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "MenuPopup",
        description: "Just before showing the popup menu",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "ModeChanged",
        description: "After changing the mode",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "QuitPre",
        description: "When using :quit, before deciding whether to exit",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "RemoteReply",
        description: "When a reply from a server Vim was received",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "SessionWritePost",
        description: "After writing a session file with :mksession",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "ShellFilterPost",
        description: "After executing a shell filter command",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "SpellFileMissing",
        description: "When a spell file is used but can't be found",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "StdinReadPre",
        description: "Before reading from stdin into the buffer",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "SwapExists",
        description: "When an existing swap file is detected",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "TabClosedPre",
        description: "Before closing a tab page",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "TermChanged",
        description: "After the value of 'term' has changed",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "TermResponse",
        description: "After the terminal response to t_RV is received",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "TermResponseAll",
        description: "After terminal responses to t_RV and others are received",
        availability: Availability::Common,
    },
    AutocmdEvent {
        name: "TerminalOpen",
        description: "After a terminal buffer was created",
        availability: Availability::NeovimOnly,
    },
    AutocmdEvent {
        name: "TerminalWinOpen",
        description: "After a terminal buffer was created in a new window",
        availability: Availability::NeovimOnly,
    },
    AutocmdEvent {
        name: "WinNewPre",
        description: "Before creating a new window",
        availability: Availability::Common,
    },
];

// ============================================================================
// Options
// ============================================================================

/// Information about a Vim option
pub struct BuiltinOption {
    pub name: &'static str,
    pub short: Option<&'static str>,
    pub description: &'static str,
    pub availability: Availability,
}

/// List of commonly used Vim options
/// Reference: :help option-list
pub static BUILTIN_OPTIONS: &[BuiltinOption] = &[
    // Display
    BuiltinOption {
        name: "number",
        short: Some("nu"),
        description: "Show line numbers",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "relativenumber",
        short: Some("rnu"),
        description: "Show relative line numbers",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "wrap",
        short: None,
        description: "Wrap long lines",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "linebreak",
        short: Some("lbr"),
        description: "Wrap at word boundaries",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "list",
        short: None,
        description: "Show invisible characters",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "listchars",
        short: Some("lcs"),
        description: "Characters for 'list' mode",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "cursorline",
        short: Some("cul"),
        description: "Highlight cursor line",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "cursorcolumn",
        short: Some("cuc"),
        description: "Highlight cursor column",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "signcolumn",
        short: Some("scl"),
        description: "When to show sign column",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "colorcolumn",
        short: Some("cc"),
        description: "Columns to highlight",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "laststatus",
        short: Some("ls"),
        description: "When to show status line",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "showtabline",
        short: Some("stal"),
        description: "When to show tab line",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "showmode",
        short: Some("smd"),
        description: "Show mode in command line",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "showcmd",
        short: Some("sc"),
        description: "Show partial command",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "ruler",
        short: Some("ru"),
        description: "Show cursor position",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "title",
        short: None,
        description: "Set window title",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "background",
        short: Some("bg"),
        description: "Background color brightness",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "termguicolors",
        short: Some("tgc"),
        description: "Use GUI colors in terminal",
        availability: Availability::Common,
    },
    // Indentation
    BuiltinOption {
        name: "tabstop",
        short: Some("ts"),
        description: "Number of spaces for tab",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "shiftwidth",
        short: Some("sw"),
        description: "Indent width",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "softtabstop",
        short: Some("sts"),
        description: "Spaces for tab in editing",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "expandtab",
        short: Some("et"),
        description: "Use spaces instead of tabs",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "autoindent",
        short: Some("ai"),
        description: "Copy indent from current line",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "smartindent",
        short: Some("si"),
        description: "Smart autoindenting",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "cindent",
        short: Some("cin"),
        description: "C-style indenting",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "indentexpr",
        short: Some("inde"),
        description: "Expression for indent",
        availability: Availability::Common,
    },
    // Search
    BuiltinOption {
        name: "hlsearch",
        short: Some("hls"),
        description: "Highlight search matches",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "incsearch",
        short: Some("is"),
        description: "Incremental search",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "ignorecase",
        short: Some("ic"),
        description: "Ignore case in search",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "smartcase",
        short: Some("scs"),
        description: "Override 'ignorecase' if uppercase",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "wrapscan",
        short: Some("ws"),
        description: "Search wraps around file",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "magic",
        short: None,
        description: "Special characters in patterns",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "gdefault",
        short: Some("gd"),
        description: "Global substitute by default",
        availability: Availability::Common,
    },
    // Editing
    BuiltinOption {
        name: "hidden",
        short: Some("hid"),
        description: "Allow hidden buffers",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "autoread",
        short: Some("ar"),
        description: "Auto-read changed files",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "autowrite",
        short: Some("aw"),
        description: "Auto-write before commands",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "backup",
        short: Some("bk"),
        description: "Keep backup file",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "backupdir",
        short: Some("bdir"),
        description: "Directory for backup files",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "swapfile",
        short: Some("swf"),
        description: "Use swap file",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "directory",
        short: Some("dir"),
        description: "Directory for swap files",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "undofile",
        short: Some("udf"),
        description: "Save undo history to file",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "undodir",
        short: Some("udir"),
        description: "Directory for undo files",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "undolevels",
        short: Some("ul"),
        description: "Maximum undo levels",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "modifiable",
        short: Some("ma"),
        description: "Allow modifications",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "readonly",
        short: Some("ro"),
        description: "Buffer is read-only",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "textwidth",
        short: Some("tw"),
        description: "Maximum line width",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "formatoptions",
        short: Some("fo"),
        description: "Auto-formatting options",
        availability: Availability::Common,
    },
    // Completion
    BuiltinOption {
        name: "completeopt",
        short: Some("cot"),
        description: "Completion options",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "complete",
        short: Some("cpt"),
        description: "Completion sources",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "pumheight",
        short: Some("ph"),
        description: "Popup menu height",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "pumwidth",
        short: Some("pw"),
        description: "Popup menu width",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "wildmenu",
        short: Some("wmnu"),
        description: "Command-line completion menu",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "wildmode",
        short: Some("wim"),
        description: "Completion mode for wildmenu",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "wildignore",
        short: Some("wig"),
        description: "Patterns to ignore in wildmenu",
        availability: Availability::Common,
    },
    // Encoding
    BuiltinOption {
        name: "encoding",
        short: Some("enc"),
        description: "Internal encoding",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "fileencoding",
        short: Some("fenc"),
        description: "File encoding",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "fileencodings",
        short: Some("fencs"),
        description: "Encoding detection list",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "fileformat",
        short: Some("ff"),
        description: "File line ending format",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "fileformats",
        short: Some("ffs"),
        description: "Line ending detection list",
        availability: Availability::Common,
    },
    // Misc
    BuiltinOption {
        name: "mouse",
        short: None,
        description: "Enable mouse support",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "clipboard",
        short: Some("cb"),
        description: "Clipboard integration",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "timeout",
        short: Some("to"),
        description: "Timeout for mapped keys",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "timeoutlen",
        short: Some("tm"),
        description: "Timeout length in ms",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "ttimeout",
        short: None,
        description: "Timeout for key codes",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "ttimeoutlen",
        short: Some("ttm"),
        description: "Timeout for key codes in ms",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "updatetime",
        short: Some("ut"),
        description: "Swap write and CursorHold delay",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "spell",
        short: None,
        description: "Enable spell checking",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "spelllang",
        short: Some("spl"),
        description: "Languages for spell checking",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "syntax",
        short: Some("syn"),
        description: "Syntax highlighting",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "filetype",
        short: Some("ft"),
        description: "File type",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "compatible",
        short: Some("cp"),
        description: "Vi compatibility mode",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "belloff",
        short: Some("bo"),
        description: "Disable bell for events",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "visualbell",
        short: Some("vb"),
        description: "Use visual bell",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "errorbells",
        short: Some("eb"),
        description: "Ring bell on errors",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "virtualedit",
        short: Some("ve"),
        description: "Allow cursor past end of line",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "scrolloff",
        short: Some("so"),
        description: "Lines to keep above/below cursor",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "sidescrolloff",
        short: Some("siso"),
        description: "Columns to keep left/right of cursor",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "splitbelow",
        short: Some("sb"),
        description: "New window below current",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "splitright",
        short: Some("spr"),
        description: "New window right of current",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "equalalways",
        short: Some("ea"),
        description: "Make windows equal size",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "foldmethod",
        short: Some("fdm"),
        description: "Folding method",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "foldlevel",
        short: Some("fdl"),
        description: "Close folds with level",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "foldexpr",
        short: Some("fde"),
        description: "Expression for fold level",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "foldcolumn",
        short: Some("fdc"),
        description: "Width of fold column",
        availability: Availability::Common,
    },
    // Neovim only
    BuiltinOption {
        name: "inccommand",
        short: Some("icm"),
        description: "Live preview of :substitute",
        availability: Availability::NeovimOnly,
    },
    BuiltinOption {
        name: "winblend",
        short: None,
        description: "Window transparency",
        availability: Availability::NeovimOnly,
    },
    BuiltinOption {
        name: "pumblend",
        short: None,
        description: "Popup menu transparency",
        availability: Availability::NeovimOnly,
    },
];

// ============================================================================
// Mapping Options
// ============================================================================

/// Information about a mapping option
pub struct MapOption {
    pub name: &'static str,
    pub description: &'static str,
}

/// List of mapping options
/// Reference: :help :map-arguments
pub static MAP_OPTIONS: &[MapOption] = &[
    MapOption {
        name: "<buffer>",
        description: "Mapping is local to buffer",
    },
    MapOption {
        name: "<nowait>",
        description: "Don't wait for longer mappings",
    },
    MapOption {
        name: "<silent>",
        description: "Don't show mapping in command line",
    },
    MapOption {
        name: "<script>",
        description: "Only remap script-local mappings",
    },
    MapOption {
        name: "<expr>",
        description: "RHS is an expression to evaluate",
    },
    MapOption {
        name: "<unique>",
        description: "Fail if mapping already exists",
    },
];

// ============================================================================
// has() Features
// ============================================================================

/// Information about a has() feature
pub struct HasFeature {
    pub name: &'static str,
    pub description: &'static str,
    pub availability: Availability,
}

/// Version prefixes for has() that should not be warned about
/// Reference: :help has()
/// Note: Will be used for diagnostics to allow patch/version checks
#[allow(dead_code)]
pub static HAS_VERSION_PREFIXES: &[&str] = &["patch-", "nvim-"];

/// List of has() features
/// Reference: :help feature-list
pub static HAS_FEATURES: &[HasFeature] = &[
    // Editor
    HasFeature {
        name: "nvim",
        description: "Running on Neovim",
        availability: Availability::NeovimOnly,
    },
    HasFeature {
        name: "vim9script",
        description: "Vim9 script support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "vim_starting",
        description: "True during startup",
        availability: Availability::Common,
    },
    // Language support
    HasFeature {
        name: "python3",
        description: "Python 3 support",
        availability: Availability::Common,
    },
    HasFeature {
        name: "lua",
        description: "Lua support",
        availability: Availability::Common,
    },
    HasFeature {
        name: "ruby",
        description: "Ruby support",
        availability: Availability::Common,
    },
    HasFeature {
        name: "perl",
        description: "Perl support",
        availability: Availability::Common,
    },
    HasFeature {
        name: "tcl",
        description: "Tcl support",
        availability: Availability::Common,
    },
    // UI
    HasFeature {
        name: "gui_running",
        description: "GUI is running",
        availability: Availability::Common,
    },
    HasFeature {
        name: "gui",
        description: "GUI support compiled in",
        availability: Availability::Common,
    },
    HasFeature {
        name: "termguicolors",
        description: "True color support",
        availability: Availability::Common,
    },
    // OS
    HasFeature {
        name: "unix",
        description: "Unix-like system",
        availability: Availability::Common,
    },
    HasFeature {
        name: "win32",
        description: "32-bit Windows",
        availability: Availability::Common,
    },
    HasFeature {
        name: "win64",
        description: "64-bit Windows",
        availability: Availability::Common,
    },
    HasFeature {
        name: "mac",
        description: "macOS",
        availability: Availability::Common,
    },
    HasFeature {
        name: "macunix",
        description: "macOS with Unix features",
        availability: Availability::Common,
    },
    HasFeature {
        name: "linux",
        description: "Linux",
        availability: Availability::Common,
    },
    HasFeature {
        name: "bsd",
        description: "BSD",
        availability: Availability::Common,
    },
    // Features
    HasFeature {
        name: "clipboard",
        description: "Clipboard support",
        availability: Availability::Common,
    },
    HasFeature {
        name: "clipboard_working",
        description: "Clipboard is working",
        availability: Availability::Common,
    },
    HasFeature {
        name: "conceal",
        description: "Conceal support",
        availability: Availability::Common,
    },
    HasFeature {
        name: "folding",
        description: "Folding support",
        availability: Availability::Common,
    },
    HasFeature {
        name: "syntax",
        description: "Syntax highlighting",
        availability: Availability::Common,
    },
    HasFeature {
        name: "spell",
        description: "Spell checking",
        availability: Availability::Common,
    },
    HasFeature {
        name: "virtualedit",
        description: "Virtual editing",
        availability: Availability::Common,
    },
    HasFeature {
        name: "signs",
        description: "Sign support",
        availability: Availability::Common,
    },
    HasFeature {
        name: "mouse",
        description: "Mouse support",
        availability: Availability::Common,
    },
    // Core features
    HasFeature {
        name: "eval",
        description: "Expression evaluation",
        availability: Availability::Common,
    },
    HasFeature {
        name: "float",
        description: "Floating point support",
        availability: Availability::Common,
    },
    HasFeature {
        name: "job",
        description: "Job support",
        availability: Availability::Common,
    },
    HasFeature {
        name: "channel",
        description: "Channel support",
        availability: Availability::Common,
    },
    HasFeature {
        name: "timers",
        description: "Timer support",
        availability: Availability::Common,
    },
    HasFeature {
        name: "lambda",
        description: "Lambda expressions",
        availability: Availability::Common,
    },
    HasFeature {
        name: "packages",
        description: "Package support",
        availability: Availability::Common,
    },
    HasFeature {
        name: "terminal",
        description: "Terminal support",
        availability: Availability::Common,
    },
    HasFeature {
        name: "textprop",
        description: "Text properties",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "popupwin",
        description: "Popup window support",
        availability: Availability::VimOnly,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_availability_is_compatible() {
        // EditorMode::Both allows everything
        assert!(Availability::Common.is_compatible(EditorMode::Both));
        assert!(Availability::VimOnly.is_compatible(EditorMode::Both));
        assert!(Availability::NeovimOnly.is_compatible(EditorMode::Both));

        // EditorMode::VimOnly excludes NeovimOnly
        assert!(Availability::Common.is_compatible(EditorMode::VimOnly));
        assert!(Availability::VimOnly.is_compatible(EditorMode::VimOnly));
        assert!(!Availability::NeovimOnly.is_compatible(EditorMode::VimOnly));

        // EditorMode::NeovimOnly excludes VimOnly
        assert!(Availability::Common.is_compatible(EditorMode::NeovimOnly));
        assert!(!Availability::VimOnly.is_compatible(EditorMode::NeovimOnly));
        assert!(Availability::NeovimOnly.is_compatible(EditorMode::NeovimOnly));
    }

    #[test]
    fn test_availability_label_suffix() {
        assert_eq!(Availability::Common.label_suffix(), "");
        assert_eq!(Availability::VimOnly.label_suffix(), " [Vim only]");
        assert_eq!(Availability::NeovimOnly.label_suffix(), " [Neovim only]");
    }
}
