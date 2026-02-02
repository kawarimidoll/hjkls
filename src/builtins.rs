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

/// Information about a built-in variable (v: scope)
pub struct BuiltinVariable {
    pub name: &'static str,
    pub description: &'static str,
    pub availability: Availability,
}

/// List of Vim built-in functions
/// Reference: :help function-list
pub static BUILTIN_FUNCTIONS: &[BuiltinFunction] = &[
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
    BuiltinFunction {
        name: "changenr",
        signature: "changenr()",
        description: "Return current change number",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "iconv",
        signature: "iconv({string}, {from}, {to})",
        description: "Convert encoding of {string}",
        availability: Availability::Common,
    },
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
    BuiltinFunction {
        name: "menu_info",
        signature: "menu_info({name} [, {mode}])",
        description: "Return information about a menu",
        availability: Availability::Common,
    },
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
    BuiltinFunction {
        name: "getimstatus",
        signature: "getimstatus()",
        description: "Return IM status",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "id",
        signature: "id({expr})",
        description: "Return unique identifier for reference",
        availability: Availability::Common,
    },
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
    BuiltinFunction {
        name: "bufwinnr",
        signature: "bufwinnr({buf})",
        description: "Return window number of buffer",
        availability: Availability::Common,
    },
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
    BuiltinFunction {
        name: "cmdcomplete_info",
        signature: "cmdcomplete_info([{what}])",
        description: "Return command line completion info",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "cscope_connection",
        signature: "cscope_connection([{num} [, {dbpath} [, {prepend}]]])",
        description: "Check cscope connection",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "did_filetype",
        signature: "did_filetype()",
        description: "Return TRUE if FileType autocommand was used",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "diff",
        signature: "diff({fromlist}, {tolist} [, {options}])",
        description: "Return diff between two lists",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "expandcmd",
        signature: "expandcmd({string} [, {options}])",
        description: "Expand special items in command string",
        availability: Availability::Common,
    },
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
    BuiltinFunction {
        name: "foreground",
        signature: "foreground()",
        description: "Bring Vim window to foreground",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "garbagecollect",
        signature: "garbagecollect([{atexit}])",
        description: "Free unused memory",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getfontname",
        signature: "getfontname([{name}])",
        description: "Return name of current font",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "getmouseshape",
        signature: "getmouseshape()",
        description: "Return current mouse shape name",
        availability: Availability::VimOnly,
    },
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
    BuiltinFunction {
        name: "glob2regpat",
        signature: "glob2regpat({string})",
        description: "Convert glob pattern to regex",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "haslocaldir",
        signature: "haslocaldir([{winnr} [, {tabnr}]])",
        description: "Return TRUE if local directory is set",
        availability: Availability::Common,
    },
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
    BuiltinFunction {
        name: "popup_findecho",
        signature: "popup_findecho()",
        description: "Return echo popup window ID",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "readblob",
        signature: "readblob({fname} [, {offset} [, {size}]])",
        description: "Read file as blob",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "server2client",
        signature: "server2client({clientid}, {string})",
        description: "Send reply to client",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "slice",
        signature: "slice({expr}, {start} [, {end}])",
        description: "Return slice of list or blob",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "soundfold",
        signature: "soundfold({word})",
        description: "Return sound-folded word",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "string",
        signature: "string({expr})",
        description: "Convert expression to string",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "strtrans",
        signature: "strtrans({string})",
        description: "Translate unprintable characters",
        availability: Availability::Common,
    },
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
    BuiltinFunction {
        name: "utf16idx",
        signature: "utf16idx({string}, {idx} [, {countcc} [, {charidx}]])",
        description: "Return UTF-16 index of byte index",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "assert_equalfile",
        signature: "assert_equalfile({fname1}, {fname2} [, {msg}])",
        description: "Assert two files have equal contents",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "bindtextdomain",
        signature: "bindtextdomain({package}, {path})",
        description: "Set path for message translations",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "wildmenumode",
        signature: "wildmenumode()",
        description: "Return TRUE if wildmenu is active",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "windowsversion",
        signature: "windowsversion()",
        description: "Return Windows version string",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "chanclose",
        signature: "chanclose({id} [, {stream}])",
        description: "Close a channel or a specific stream",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "chansend",
        signature: "chansend({id}, {data})",
        description: "Send data to channel",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "jobpid",
        signature: "jobpid({job})",
        description: "Return the PID of a job",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "jobresize",
        signature: "jobresize({job}, {width}, {height})",
        description: "Resize the pseudo terminal window of a job",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "jobstart",
        signature: "jobstart({cmd} [, {opts}])",
        description: "Spawn a job",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "jobstop",
        signature: "jobstop({id})",
        description: "Stop a job",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "jobwait",
        signature: "jobwait({jobs} [, {timeout}])",
        description: "Wait for jobs to complete",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "rpcnotify",
        signature: "rpcnotify({channel}, {event} [, {args}...])",
        description: "Send RPC notification to channel",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "rpcrequest",
        signature: "rpcrequest({channel}, {method} [, {args}...])",
        description: "Send RPC request to channel",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "serverstart",
        signature: "serverstart([{address}])",
        description: "Start listening for RPC messages",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "serverstop",
        signature: "serverstop({address})",
        description: "Stop listening on address",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "sockconnect",
        signature: "sockconnect({mode}, {address} [, {opts}])",
        description: "Connect a socket to an address",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "stdioopen",
        signature: "stdioopen({opts})",
        description: "Open stdin and stdout as a channel",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "msgpackdump",
        signature: "msgpackdump({list} [, {type}])",
        description: "Convert list to msgpack",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "msgpackparse",
        signature: "msgpackparse({data})",
        description: "Convert msgpack to list",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "ctxget",
        signature: "ctxget([{index}])",
        description: "Return context at index from context stack",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "ctxpop",
        signature: "ctxpop()",
        description: "Pop and restore context from stack",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "ctxpush",
        signature: "ctxpush([{types}])",
        description: "Push current context on stack",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "ctxset",
        signature: "ctxset({context} [, {index}])",
        description: "Set context at index on stack",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "ctxsize",
        signature: "ctxsize()",
        description: "Return size of context stack",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "dictwatcheradd",
        signature: "dictwatcheradd({dict}, {pattern}, {callback})",
        description: "Add a watcher to a dictionary",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "dictwatcherdel",
        signature: "dictwatcherdel({dict}, {pattern}, {callback})",
        description: "Remove a watcher from a dictionary",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "wait",
        signature: "wait({timeout}, {condition} [, {interval}])",
        description: "Wait until condition is true or timeout",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "err_teapot",
        signature: "err_teapot([{expr}])",
        description: "Produce error E418 or E503",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "exists_compiled",
        signature: "exists_compiled({expr})",
        description: "Check if expression exists at compile time",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "filecopy",
        signature: "filecopy({from}, {to})",
        description: "Copy file from one location to another",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "foreach",
        signature: "foreach({expr1}, {expr2})",
        description: "Call function for each item without modifying",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "getcellpixels",
        signature: "getcellpixels()",
        description: "Return terminal cell pixel size",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "getcmdcomplpat",
        signature: "getcmdcomplpat()",
        description: "Return completion pattern of command-line",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "getcmdprompt",
        signature: "getcmdprompt()",
        description: "Return current command-line prompt",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "getcompletiontype",
        signature: "getcompletiontype({pat})",
        description: "Return type of command-line completion",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "getstacktrace",
        signature: "getstacktrace()",
        description: "Return current stack trace",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "js_decode",
        signature: "js_decode({string})",
        description: "Decode JS style JSON",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "js_encode",
        signature: "js_encode({expr})",
        description: "Encode JS style JSON",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "menu_get",
        signature: "menu_get({path} [, {modes}])",
        description: "Return list of menus matching path",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "ngettext",
        signature: "ngettext({single}, {plural}, {number} [, {domain}])",
        description: "Return translated string based on number",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "popup_setbuf",
        signature: "popup_setbuf({id}, {buf})",
        description: "Set buffer for popup window",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "preinserted",
        signature: "preinserted()",
        description: "Return if text is pre-inserted after cursor",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "prompt_getinput",
        signature: "prompt_getinput({buf})",
        description: "Get input from prompt buffer",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "redraw_listener_add",
        signature: "redraw_listener_add({callback})",
        description: "Add a callback for redraw events",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "redraw_listener_remove",
        signature: "redraw_listener_remove({id})",
        description: "Remove a redraw listener callback",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "reg_recorded",
        signature: "reg_recorded()",
        description: "Return name of last recorded register",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "terminalprops",
        signature: "terminalprops()",
        description: "Return terminal properties dictionary",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_alloc_fail",
        signature: "test_alloc_fail({id}, {countdown}, {repeat})",
        description: "Make memory allocation fail for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_autochdir",
        signature: "test_autochdir()",
        description: "Enable autochdir during startup for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_feedinput",
        signature: "test_feedinput({string})",
        description: "Add key sequence to input buffer for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_garbagecollect_now",
        signature: "test_garbagecollect_now()",
        description: "Free memory immediately for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_garbagecollect_soon",
        signature: "test_garbagecollect_soon()",
        description: "Free memory soon for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_getvalue",
        signature: "test_getvalue({string})",
        description: "Get value of internal variable for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_gui_event",
        signature: "test_gui_event({event}, {args})",
        description: "Generate GUI event for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_ignore_error",
        signature: "test_ignore_error({expr})",
        description: "Ignore specific error for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_mswin_event",
        signature: "test_mswin_event({event}, {args})",
        description: "Generate MS-Windows event for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_null_blob",
        signature: "test_null_blob()",
        description: "Return null blob for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_null_channel",
        signature: "test_null_channel()",
        description: "Return null channel for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_null_dict",
        signature: "test_null_dict()",
        description: "Return null dict for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_null_function",
        signature: "test_null_function()",
        description: "Return null function for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_null_job",
        signature: "test_null_job()",
        description: "Return null job for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_null_list",
        signature: "test_null_list()",
        description: "Return null list for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_null_partial",
        signature: "test_null_partial()",
        description: "Return null partial for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_null_string",
        signature: "test_null_string()",
        description: "Return null string for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_null_tuple",
        signature: "test_null_tuple()",
        description: "Return null tuple for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_option_not_set",
        signature: "test_option_not_set({name})",
        description: "Reset flag indicating option was set",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_override",
        signature: "test_override({expr}, {val})",
        description: "Override Vim internal for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_refcount",
        signature: "test_refcount({expr})",
        description: "Get reference count for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_setmouse",
        signature: "test_setmouse({row}, {col})",
        description: "Set mouse position for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_settime",
        signature: "test_settime({expr})",
        description: "Set current time for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_srand_seed",
        signature: "test_srand_seed([{seed}])",
        description: "Set seed for srand testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_unknown",
        signature: "test_unknown()",
        description: "Return unknown value for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "test_void",
        signature: "test_void()",
        description: "Return void value for testing",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "uri_decode",
        signature: "uri_decode({string})",
        description: "URI-decode a string",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "uri_encode",
        signature: "uri_encode({string})",
        description: "URI-encode a string",
        availability: Availability::VimOnly,
    },
    BuiltinFunction {
        name: "wildtrigger",
        signature: "wildtrigger()",
        description: "Start wildcard expansion in command-line",
        availability: Availability::Common,
    },
    BuiltinFunction {
        name: "nvim__complete_set",
        signature: "nvim__complete_set({index}, {opts})",
        description: "Set info for completion item at given index (experimental)",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim__exec_lua_fast",
        signature: "nvim__exec_lua_fast({code}, {args})",
        description: "Execute Lua code during api-fast contexts (experimental)",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim__get_runtime",
        signature: "nvim__get_runtime({pat}, {all}, {opts})",
        description: "Find files in runtime directories",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim__id",
        signature: "nvim__id({obj})",
        description: "Returns object given as argument (testing)",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim__id_array",
        signature: "nvim__id_array({arr})",
        description: "Returns array given as argument (testing)",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim__id_dict",
        signature: "nvim__id_dict({dct})",
        description: "Returns dict given as argument (testing)",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim__id_float",
        signature: "nvim__id_float({flt})",
        description: "Returns floating-point value given as argument (testing)",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim__inspect_cell",
        signature: "nvim__inspect_cell({grid}, {row}, {col})",
        description: "Inspect a cell in the grid",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim__invalidate_glyph_cache",
        signature: "nvim__invalidate_glyph_cache()",
        description: "Force a glyph cache clear (testing)",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim__ns_get",
        signature: "nvim__ns_get({ns_id})",
        description: "Get the properties for namespace (experimental)",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim__ns_set",
        signature: "nvim__ns_set({ns_id}, {opts})",
        description: "Set some properties for namespace (experimental)",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim__redraw",
        signature: "nvim__redraw({opts})",
        description: "Instruct Nvim to redraw various components (experimental)",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim__stats",
        signature: "nvim__stats()",
        description: "Gets internal stats",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_attach",
        signature: "nvim_buf_attach({buffer}, {send_buffer}, {opts})",
        description: "Activates buffer-update events on a channel or as Lua callbacks",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_call",
        signature: "nvim_buf_call({buffer}, {fun})",
        description: "Call a function with buffer as temporary current buffer",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_clear_namespace",
        signature: "nvim_buf_clear_namespace({buffer}, {ns_id}, {line_start}, {line_end})",
        description: "Clears namespaced objects from a region",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_create_user_command",
        signature: "nvim_buf_create_user_command({buffer}, {name}, {command}, {opts})",
        description: "Creates a buffer-local command",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_del_mark",
        signature: "nvim_buf_del_mark({buffer}, {name})",
        description: "Deletes a named mark in the buffer",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_del_user_command",
        signature: "nvim_buf_del_user_command({buffer}, {name})",
        description: "Delete a buffer-local user-defined command",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_del_var",
        signature: "nvim_buf_del_var({buffer}, {name})",
        description: "Removes a buffer-scoped (b:) variable",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_detach",
        signature: "nvim_buf_detach({buffer})",
        description: "Deactivates buffer-update events on the channel",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_get_changedtick",
        signature: "nvim_buf_get_changedtick({buffer})",
        description: "Gets a changed tick of a buffer",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_get_commands",
        signature: "nvim_buf_get_commands({buffer}, {opts})",
        description: "Gets a map of buffer-local user-commands",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_get_extmark_by_id",
        signature: "nvim_buf_get_extmark_by_id({buffer}, {ns_id}, {id}, {opts})",
        description: "Gets the position of an extmark",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_get_keymap",
        signature: "nvim_buf_get_keymap({buffer}, {mode})",
        description: "Gets a list of buffer-local mapping definitions",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_get_mark",
        signature: "nvim_buf_get_mark({buffer}, {name})",
        description: "Returns a (row,col) tuple representing the position of the named mark",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_get_offset",
        signature: "nvim_buf_get_offset({buffer}, {index})",
        description: "Returns the byte offset of a line (0-indexed)",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_is_loaded",
        signature: "nvim_buf_is_loaded({buffer})",
        description: "Checks if a buffer is valid and loaded",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_buf_set_mark",
        signature: "nvim_buf_set_mark({buffer}, {name}, {line}, {col}, {opts})",
        description: "Sets a named mark in the given buffer",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_call_dict_function",
        signature: "nvim_call_dict_function({dict}, {fn}, {args})",
        description: "Calls a Vimscript Dictionary-function with the given arguments",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_chan_send",
        signature: "nvim_chan_send({chan}, {data})",
        description: "Sends raw data to channel",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_clear_autocmds",
        signature: "nvim_clear_autocmds({opts})",
        description: "Clears all autocommands selected by opts",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_cmd",
        signature: "nvim_cmd({cmd}, {opts})",
        description: "Executes an Ex command (structured Dict)",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_create_user_command",
        signature: "nvim_create_user_command({name}, {command}, {opts})",
        description: "Creates a global user-commands command",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_del_current_line",
        signature: "nvim_del_current_line()",
        description: "Deletes the current line",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_del_mark",
        signature: "nvim_del_mark({name})",
        description: "Deletes an uppercase/file named mark",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_del_user_command",
        signature: "nvim_del_user_command({name})",
        description: "Delete a user-defined command",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_error_event",
        signature: "nvim_error_event({type}, {msg})",
        description: "Emitted on the client channel if an async API request responds with an error",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_eval_statusline",
        signature: "nvim_eval_statusline({str}, {opts})",
        description: "Evaluates statusline string",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_exec_autocmds",
        signature: "nvim_exec_autocmds({event}, {opts})",
        description: "Execute all autocommands for event that match the corresponding opts",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_all_options_info",
        signature: "nvim_get_all_options_info()",
        description: "Gets the option information for all options",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_api_info",
        signature: "nvim_get_api_info()",
        description: "Returns a 2-tuple (Array) with channel id and api-metadata map",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_autocmds",
        signature: "nvim_get_autocmds({opts})",
        description: "Get all autocommands that match the corresponding opts",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_chan_info",
        signature: "nvim_get_chan_info({chan})",
        description: "Gets information about a channel",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_color_by_name",
        signature: "nvim_get_color_by_name({name})",
        description: "Returns the 24-bit RGB value of a color name or #rrggbb string",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_color_map",
        signature: "nvim_get_color_map()",
        description: "Returns a map of color names and RGB values",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_commands",
        signature: "nvim_get_commands({opts})",
        description: "Gets a map of global (non-buffer-local) Ex commands",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_context",
        signature: "nvim_get_context({opts})",
        description: "Gets a map of the current editor state",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_current_line",
        signature: "nvim_get_current_line()",
        description: "Gets the current line",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_current_tabpage",
        signature: "nvim_get_current_tabpage()",
        description: "Gets the current tabpage",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_hl_id_by_name",
        signature: "nvim_get_hl_id_by_name({name})",
        description: "Gets a highlight group by name",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_hl_ns",
        signature: "nvim_get_hl_ns({opts})",
        description: "Gets the active highlight namespace",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_keymap",
        signature: "nvim_get_keymap({mode})",
        description: "Gets a list of global (non-buffer-local) mapping definitions",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_mark",
        signature: "nvim_get_mark({name}, {opts})",
        description: "Returns a (row, col, buffer, buffername) tuple for uppercase/file mark",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_namespaces",
        signature: "nvim_get_namespaces()",
        description: "Gets existing, non-anonymous namespaces",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_proc",
        signature: "nvim_get_proc({pid})",
        description: "Gets info describing process pid",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_proc_children",
        signature: "nvim_get_proc_children({pid})",
        description: "Gets the immediate children of process pid",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_runtime_file",
        signature: "nvim_get_runtime_file({name}, {all})",
        description: "Finds files in runtime directories, in runtimepath order",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_get_vvar",
        signature: "nvim_get_vvar({name})",
        description: "Gets a v: variable",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_input_mouse",
        signature: "nvim_input_mouse({button}, {action}, {modifier}, {grid}, {row}, {col})",
        description: "Send mouse event from GUI",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_list_chans",
        signature: "nvim_list_chans()",
        description: "Get information about all open channels",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_list_runtime_paths",
        signature: "nvim_list_runtime_paths()",
        description: "Gets the paths contained in runtime-search-path",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_list_tabpages",
        signature: "nvim_list_tabpages()",
        description: "Gets the current list of tab-IDs",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_list_uis",
        signature: "nvim_list_uis()",
        description: "Gets a list of dictionaries representing attached UIs",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_load_context",
        signature: "nvim_load_context({dict})",
        description: "Sets the current editor state from the given context map",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_open_term",
        signature: "nvim_open_term({buffer}, {opts})",
        description: "Open a terminal instance in a buffer",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_parse_cmd",
        signature: "nvim_parse_cmd({str}, {opts})",
        description: "Parse command line",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_parse_expression",
        signature: "nvim_parse_expression({expr}, {flags}, {highlight})",
        description: "Parse a Vimscript expression",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_paste",
        signature: "nvim_paste({data}, {crlf}, {phase})",
        description: "Pastes at cursor (in any mode), and sets redo so dot will repeat",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_put",
        signature: "nvim_put({lines}, {type}, {after}, {follow})",
        description: "Puts text at cursor, in any mode",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_select_popupmenu_item",
        signature: "nvim_select_popupmenu_item({item}, {insert}, {finish}, {opts})",
        description: "Selects an item in the completion popup menu",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_set_client_info",
        signature: "nvim_set_client_info({name}, {version}, {type}, {methods}, {attributes})",
        description: "Self-identifies the client, and sets optional flags on the channel",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_set_current_dir",
        signature: "nvim_set_current_dir({dir})",
        description: "Changes the global working directory",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_set_current_line",
        signature: "nvim_set_current_line({line})",
        description: "Sets the text on the current line",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_set_current_tabpage",
        signature: "nvim_set_current_tabpage({tabpage})",
        description: "Sets the current tabpage",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_set_decoration_provider",
        signature: "nvim_set_decoration_provider({ns_id}, {opts})",
        description: "Set or change decoration provider for a namespace",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_set_hl_ns",
        signature: "nvim_set_hl_ns({ns_id})",
        description: "Set active namespace for highlights defined with nvim_set_hl()",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_set_hl_ns_fast",
        signature: "nvim_set_hl_ns_fast({ns_id})",
        description: "Set active namespace for highlights while redrawing",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_set_vvar",
        signature: "nvim_set_vvar({name}, {value})",
        description: "Sets a v: variable, if it is not readonly",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_strwidth",
        signature: "nvim_strwidth({text})",
        description: "Calculates the number of display cells occupied by text",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_tabpage_del_var",
        signature: "nvim_tabpage_del_var({tabpage}, {name})",
        description: "Removes a tab-scoped (t:) variable",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_tabpage_get_number",
        signature: "nvim_tabpage_get_number({tabpage})",
        description: "Gets the tabpage number",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_tabpage_get_var",
        signature: "nvim_tabpage_get_var({tabpage}, {name})",
        description: "Gets a tab-scoped (t:) variable",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_tabpage_get_win",
        signature: "nvim_tabpage_get_win({tabpage})",
        description: "Gets the current window in a tabpage",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_tabpage_is_valid",
        signature: "nvim_tabpage_is_valid({tabpage})",
        description: "Checks if a tabpage is valid",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_tabpage_list_wins",
        signature: "nvim_tabpage_list_wins({tabpage})",
        description: "Gets the windows in a tabpage",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_tabpage_set_var",
        signature: "nvim_tabpage_set_var({tabpage}, {name}, {value})",
        description: "Sets a tab-scoped (t:) variable",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_tabpage_set_win",
        signature: "nvim_tabpage_set_win({tabpage}, {win})",
        description: "Sets the current window in a tabpage",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_ui_attach",
        signature: "nvim_ui_attach({width}, {height}, {options})",
        description: "Activates UI events on the channel",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_ui_detach",
        signature: "nvim_ui_detach()",
        description: "Deactivates UI events on the channel",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_ui_pum_set_bounds",
        signature: "nvim_ui_pum_set_bounds({width}, {height}, {row}, {col})",
        description: "Tells Nvim the geometry of the popupmenu",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_ui_pum_set_height",
        signature: "nvim_ui_pum_set_height({height})",
        description: "Tells Nvim the number of elements displaying in the popupmenu",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_ui_send",
        signature: "nvim_ui_send({content})",
        description: "Sends arbitrary data to a UI (experimental)",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_ui_set_focus",
        signature: "nvim_ui_set_focus({gained})",
        description: "Tells the nvim server if focus was gained or lost by the GUI",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_ui_set_option",
        signature: "nvim_ui_set_option({name}, {value})",
        description: "Sets a UI option",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_ui_term_event",
        signature: "nvim_ui_term_event({event}, {value})",
        description: "Emitted by the TUI client to signal when a host-terminal event occurred",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_ui_try_resize",
        signature: "nvim_ui_try_resize({width}, {height})",
        description: "Try to resize the UI",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_ui_try_resize_grid",
        signature: "nvim_ui_try_resize_grid({grid}, {width}, {height})",
        description: "Tell Nvim to resize a grid",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_call",
        signature: "nvim_win_call({window}, {fun})",
        description: "Calls a function with window as temporary current window",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_del_var",
        signature: "nvim_win_del_var({window}, {name})",
        description: "Removes a window-scoped (w:) variable",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_get_buf",
        signature: "nvim_win_get_buf({window})",
        description: "Gets the current buffer in a window",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_get_height",
        signature: "nvim_win_get_height({window})",
        description: "Gets the window height",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_get_number",
        signature: "nvim_win_get_number({window})",
        description: "Gets the window number",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_get_position",
        signature: "nvim_win_get_position({window})",
        description: "Gets the window position in display cells",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_get_tabpage",
        signature: "nvim_win_get_tabpage({window})",
        description: "Gets the window tabpage",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_get_width",
        signature: "nvim_win_get_width({window})",
        description: "Gets the window width",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_hide",
        signature: "nvim_win_hide({window})",
        description: "Closes the window and hide the buffer it contains",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_is_valid",
        signature: "nvim_win_is_valid({window})",
        description: "Checks if a window is valid",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_set_buf",
        signature: "nvim_win_set_buf({window}, {buffer})",
        description: "Sets the current buffer in a window, without side effects",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_set_height",
        signature: "nvim_win_set_height({window}, {height})",
        description: "Sets the window height",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_set_hl_ns",
        signature: "nvim_win_set_hl_ns({window}, {ns_id})",
        description: "Set highlight namespace for a window",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_set_width",
        signature: "nvim_win_set_width({window}, {width})",
        description: "Sets the window width",
        availability: Availability::NeovimOnly,
    },
    BuiltinFunction {
        name: "nvim_win_text_height",
        signature: "nvim_win_text_height({window}, {opts})",
        description: "Computes the number of screen lines occupied by a range of text",
        availability: Availability::NeovimOnly,
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
    /// Minimum abbreviation length (e.g., 1 for "q[uit]" means "q" is valid)
    pub min_abbrev: u8,
}

impl BuiltinCommand {
    /// Returns the shortest valid abbreviation of this command
    pub fn min_name(&self) -> &str {
        &self.name[..self.min_abbrev as usize]
    }

    /// Check if the input matches this command (considering abbreviation)
    pub fn matches(&self, input: &str) -> bool {
        input.len() >= self.min_abbrev as usize
            && input.len() <= self.name.len()
            && self.name.starts_with(input)
    }
}

/// List of commonly used Vim Ex commands
/// Reference: :help ex-cmd-index
pub static BUILTIN_COMMANDS: &[BuiltinCommand] = &[
    BuiltinCommand {
        name: "if",
        description: "Execute commands when condition is true",
        availability: Availability::Common,
        min_abbrev: 2, // :if
    },
    BuiltinCommand {
        name: "else",
        description: "Execute commands when 'if' condition is false",
        availability: Availability::Common,
        min_abbrev: 2, // :el[se]
    },
    BuiltinCommand {
        name: "elseif",
        description: "Execute commands when condition is true",
        availability: Availability::Common,
        min_abbrev: 5, // :elsei[f]
    },
    BuiltinCommand {
        name: "endif",
        description: "End 'if' block",
        availability: Availability::Common,
        min_abbrev: 2, // :en[dif]
    },
    BuiltinCommand {
        name: "for",
        description: "Loop over a list",
        availability: Availability::Common,
        min_abbrev: 3, // :for
    },
    BuiltinCommand {
        name: "endfor",
        description: "End 'for' loop",
        availability: Availability::Common,
        min_abbrev: 5, // :endfo[r]
    },
    BuiltinCommand {
        name: "while",
        description: "Loop while condition is true",
        availability: Availability::Common,
        min_abbrev: 2, // :wh[ile]
    },
    BuiltinCommand {
        name: "endwhile",
        description: "End 'while' loop",
        availability: Availability::Common,
        min_abbrev: 4, // :endw[hile]
    },
    BuiltinCommand {
        name: "try",
        description: "Start try block for exception handling",
        availability: Availability::Common,
        min_abbrev: 3, // :try
    },
    BuiltinCommand {
        name: "catch",
        description: "Catch exceptions",
        availability: Availability::Common,
        min_abbrev: 3, // :cat[ch]
    },
    BuiltinCommand {
        name: "finally",
        description: "Execute commands regardless of exception",
        availability: Availability::Common,
        min_abbrev: 4, // :fina[lly]
    },
    BuiltinCommand {
        name: "endtry",
        description: "End 'try' block",
        availability: Availability::Common,
        min_abbrev: 4, // :endt[ry]
    },
    BuiltinCommand {
        name: "throw",
        description: "Throw an exception",
        availability: Availability::Common,
        min_abbrev: 2, // :th[row]
    },
    BuiltinCommand {
        name: "break",
        description: "Break out of loop",
        availability: Availability::Common,
        min_abbrev: 4, // :brea[k]
    },
    BuiltinCommand {
        name: "continue",
        description: "Continue loop from start",
        availability: Availability::Common,
        min_abbrev: 3, // :con[tinue]
    },
    BuiltinCommand {
        name: "function",
        description: "Define a function",
        availability: Availability::Common,
        min_abbrev: 2, // :fu[nction]
    },
    BuiltinCommand {
        name: "endfunction",
        description: "End function definition",
        availability: Availability::Common,
        min_abbrev: 4, // :endf[unction]
    },
    BuiltinCommand {
        name: "return",
        description: "Return from function",
        availability: Availability::Common,
        min_abbrev: 4, // :retu[rn]
    },
    BuiltinCommand {
        name: "call",
        description: "Call a function",
        availability: Availability::Common,
        min_abbrev: 3, // :cal[l]
    },
    BuiltinCommand {
        name: "let",
        description: "Assign value to variable",
        availability: Availability::Common,
        min_abbrev: 3, // :let
    },
    BuiltinCommand {
        name: "const",
        description: "Define a constant",
        availability: Availability::Common,
        min_abbrev: 4, // :cons[t]
    },
    BuiltinCommand {
        name: "unlet",
        description: "Delete variable",
        availability: Availability::Common,
        min_abbrev: 3, // :unl[et]
    },
    BuiltinCommand {
        name: "lockvar",
        description: "Lock variable",
        availability: Availability::Common,
        min_abbrev: 5, // :lockv[ar]
    },
    BuiltinCommand {
        name: "unlockvar",
        description: "Unlock variable",
        availability: Availability::Common,
        min_abbrev: 4, // :unlo[ckvar]
    },
    BuiltinCommand {
        name: "echo",
        description: "Echo expression",
        availability: Availability::Common,
        min_abbrev: 2, // :ec[ho]
    },
    BuiltinCommand {
        name: "echom",
        description: "Echo message and save in history",
        availability: Availability::Common,
        min_abbrev: 5, // :echom[sg] (alias)
    },
    BuiltinCommand {
        name: "echomsg",
        description: "Echo message and save in history",
        availability: Availability::Common,
        min_abbrev: 5, // :echom[sg]
    },
    BuiltinCommand {
        name: "echoerr",
        description: "Echo error message",
        availability: Availability::Common,
        min_abbrev: 5, // :echoe[rr]
    },
    BuiltinCommand {
        name: "echon",
        description: "Echo without newline",
        availability: Availability::Common,
        min_abbrev: 5, // :echon
    },
    BuiltinCommand {
        name: "echohl",
        description: "Set highlight group for echo",
        availability: Availability::Common,
        min_abbrev: 5, // :echoh[l]
    },
    BuiltinCommand {
        name: "echowindow",
        description: "Echo in popup window",
        availability: Availability::Common,
        min_abbrev: 5, // :echow[indow]
    },
    BuiltinCommand {
        name: "map",
        description: "Define key mapping (all modes)",
        availability: Availability::Common,
        min_abbrev: 3, // :map
    },
    BuiltinCommand {
        name: "nmap",
        description: "Define normal mode mapping",
        availability: Availability::Common,
        min_abbrev: 2, // :nm[ap]
    },
    BuiltinCommand {
        name: "vmap",
        description: "Define visual mode mapping",
        availability: Availability::Common,
        min_abbrev: 2, // :vm[ap]
    },
    BuiltinCommand {
        name: "xmap",
        description: "Define visual (not select) mode mapping",
        availability: Availability::Common,
        min_abbrev: 2, // :xm[ap]
    },
    BuiltinCommand {
        name: "smap",
        description: "Define select mode mapping",
        availability: Availability::Common,
        min_abbrev: 4, // :smap
    },
    BuiltinCommand {
        name: "imap",
        description: "Define insert mode mapping",
        availability: Availability::Common,
        min_abbrev: 2, // :im[ap]
    },
    BuiltinCommand {
        name: "cmap",
        description: "Define command-line mode mapping",
        availability: Availability::Common,
        min_abbrev: 2, // :cm[ap]
    },
    BuiltinCommand {
        name: "omap",
        description: "Define operator-pending mode mapping",
        availability: Availability::Common,
        min_abbrev: 2, // :om[ap]
    },
    BuiltinCommand {
        name: "lmap",
        description: "Define language mapping",
        availability: Availability::Common,
        min_abbrev: 2, // :lm[ap]
    },
    BuiltinCommand {
        name: "tmap",
        description: "Define terminal mode mapping",
        availability: Availability::Common,
        min_abbrev: 3, // :tma[p]
    },
    BuiltinCommand {
        name: "noremap",
        description: "Define non-recursive mapping (all modes)",
        availability: Availability::Common,
        min_abbrev: 2, // :no[remap]
    },
    BuiltinCommand {
        name: "nnoremap",
        description: "Define non-recursive normal mode mapping",
        availability: Availability::Common,
        min_abbrev: 2, // :nn[oremap]
    },
    BuiltinCommand {
        name: "vnoremap",
        description: "Define non-recursive visual mode mapping",
        availability: Availability::Common,
        min_abbrev: 2, // :vn[oremap]
    },
    BuiltinCommand {
        name: "xnoremap",
        description: "Define non-recursive visual (not select) mode mapping",
        availability: Availability::Common,
        min_abbrev: 2, // :xn[oremap]
    },
    BuiltinCommand {
        name: "snoremap",
        description: "Define non-recursive select mode mapping",
        availability: Availability::Common,
        min_abbrev: 4, // :snor[emap]
    },
    BuiltinCommand {
        name: "inoremap",
        description: "Define non-recursive insert mode mapping",
        availability: Availability::Common,
        min_abbrev: 3, // :ino[remap]
    },
    BuiltinCommand {
        name: "cnoremap",
        description: "Define non-recursive command-line mode mapping",
        availability: Availability::Common,
        min_abbrev: 3, // :cno[remap]
    },
    BuiltinCommand {
        name: "onoremap",
        description: "Define non-recursive operator-pending mode mapping",
        availability: Availability::Common,
        min_abbrev: 3, // :ono[remap]
    },
    BuiltinCommand {
        name: "lnoremap",
        description: "Define non-recursive language mapping",
        availability: Availability::Common,
        min_abbrev: 2, // :ln[oremap]
    },
    BuiltinCommand {
        name: "tnoremap",
        description: "Define non-recursive terminal mode mapping",
        availability: Availability::Common,
        min_abbrev: 3, // :tno[remap]
    },
    BuiltinCommand {
        name: "unmap",
        description: "Remove mapping (all modes)",
        availability: Availability::Common,
        min_abbrev: 3, // :unm[ap]
    },
    BuiltinCommand {
        name: "nunmap",
        description: "Remove normal mode mapping",
        availability: Availability::Common,
        min_abbrev: 3, // :nun[map]
    },
    BuiltinCommand {
        name: "vunmap",
        description: "Remove visual mode mapping",
        availability: Availability::Common,
        min_abbrev: 2, // :vu[nmap]
    },
    BuiltinCommand {
        name: "xunmap",
        description: "Remove visual (not select) mode mapping",
        availability: Availability::Common,
        min_abbrev: 2, // :xu[nmap]
    },
    BuiltinCommand {
        name: "sunmap",
        description: "Remove select mode mapping",
        availability: Availability::Common,
        min_abbrev: 4, // :sunm[ap]
    },
    BuiltinCommand {
        name: "iunmap",
        description: "Remove insert mode mapping",
        availability: Availability::Common,
        min_abbrev: 2, // :iu[nmap]
    },
    BuiltinCommand {
        name: "cunmap",
        description: "Remove command-line mode mapping",
        availability: Availability::Common,
        min_abbrev: 2, // :cu[nmap]
    },
    BuiltinCommand {
        name: "ounmap",
        description: "Remove operator-pending mode mapping",
        availability: Availability::Common,
        min_abbrev: 2, // :ou[nmap]
    },
    BuiltinCommand {
        name: "lunmap",
        description: "Remove language mapping",
        availability: Availability::Common,
        min_abbrev: 2, // :lu[nmap]
    },
    BuiltinCommand {
        name: "tunmap",
        description: "Remove terminal mode mapping",
        availability: Availability::Common,
        min_abbrev: 5, // :tunma[p]
    },
    BuiltinCommand {
        name: "mapclear",
        description: "Clear all mappings (all modes)",
        availability: Availability::Common,
        min_abbrev: 4, // :mapc[lear]
    },
    BuiltinCommand {
        name: "autocmd",
        description: "Define autocommand",
        availability: Availability::Common,
        min_abbrev: 2, // :au[tocmd]
    },
    BuiltinCommand {
        name: "augroup",
        description: "Define autocommand group",
        availability: Availability::Common,
        min_abbrev: 3, // :aug[roup]
    },
    BuiltinCommand {
        name: "doautocmd",
        description: "Execute autocommands",
        availability: Availability::Common,
        min_abbrev: 2, // :do[autocmd]
    },
    BuiltinCommand {
        name: "doautoall",
        description: "Execute autocommands for all buffers",
        availability: Availability::Common,
        min_abbrev: 7, // :doautoa[ll]
    },
    BuiltinCommand {
        name: "set",
        description: "Set option value",
        availability: Availability::Common,
        min_abbrev: 2, // :se[t]
    },
    BuiltinCommand {
        name: "setlocal",
        description: "Set local option value",
        availability: Availability::Common,
        min_abbrev: 4, // :setl[ocal]
    },
    BuiltinCommand {
        name: "setglobal",
        description: "Set global option value",
        availability: Availability::Common,
        min_abbrev: 4, // :setg[lobal]
    },
    BuiltinCommand {
        name: "highlight",
        description: "Define highlighting",
        availability: Availability::Common,
        min_abbrev: 2, // :hi[ghlight]
    },
    BuiltinCommand {
        name: "syntax",
        description: "Define syntax highlighting",
        availability: Availability::Common,
        min_abbrev: 2, // :sy[ntax]
    },
    BuiltinCommand {
        name: "colorscheme",
        description: "Load colorscheme",
        availability: Availability::Common,
        min_abbrev: 4, // :colo[rscheme]
    },
    BuiltinCommand {
        name: "command",
        description: "Define user command",
        availability: Availability::Common,
        min_abbrev: 3, // :com[mand]
    },
    BuiltinCommand {
        name: "delcommand",
        description: "Delete user command",
        availability: Availability::Common,
        min_abbrev: 4, // :delc[ommand]
    },
    BuiltinCommand {
        name: "comclear",
        description: "Clear all user commands",
        availability: Availability::Common,
        min_abbrev: 4, // :comc[lear]
    },
    BuiltinCommand {
        name: "execute",
        description: "Execute string as Ex command",
        availability: Availability::Common,
        min_abbrev: 3, // :exe[cute]
    },
    BuiltinCommand {
        name: "normal",
        description: "Execute normal mode commands",
        availability: Availability::Common,
        min_abbrev: 4, // :norm[al]
    },
    BuiltinCommand {
        name: "source",
        description: "Read and execute commands from file",
        availability: Availability::Common,
        min_abbrev: 2, // :so[urce]
    },
    BuiltinCommand {
        name: "runtime",
        description: "Source files from 'runtimepath'",
        availability: Availability::Common,
        min_abbrev: 2, // :ru[ntime]
    },
    BuiltinCommand {
        name: "finish",
        description: "Stop sourcing current script",
        availability: Availability::Common,
        min_abbrev: 4, // :fini[sh]
    },
    BuiltinCommand {
        name: "edit",
        description: "Edit a file",
        availability: Availability::Common,
        min_abbrev: 1, // :e[dit]
    },
    BuiltinCommand {
        name: "enew",
        description: "Edit a new unnamed buffer",
        availability: Availability::Common,
        min_abbrev: 3, // :ene[w]
    },
    BuiltinCommand {
        name: "buffer",
        description: "Go to buffer",
        availability: Availability::Common,
        min_abbrev: 1, // :b[uffer]
    },
    BuiltinCommand {
        name: "bdelete",
        description: "Delete buffer",
        availability: Availability::Common,
        min_abbrev: 2, // :bd[elete]
    },
    BuiltinCommand {
        name: "bwipeout",
        description: "Wipe out buffer",
        availability: Availability::Common,
        min_abbrev: 2, // :bw[ipeout]
    },
    BuiltinCommand {
        name: "split",
        description: "Split window horizontally",
        availability: Availability::Common,
        min_abbrev: 2, // :sp[lit]
    },
    BuiltinCommand {
        name: "vsplit",
        description: "Split window vertically",
        availability: Availability::Common,
        min_abbrev: 2, // :vs[plit]
    },
    BuiltinCommand {
        name: "new",
        description: "Create new window with empty buffer",
        availability: Availability::Common,
        min_abbrev: 3, // :new
    },
    BuiltinCommand {
        name: "vnew",
        description: "Create new vertical window with empty buffer",
        availability: Availability::Common,
        min_abbrev: 3, // :vne[w]
    },
    BuiltinCommand {
        name: "close",
        description: "Close window",
        availability: Availability::Common,
        min_abbrev: 3, // :clo[se]
    },
    BuiltinCommand {
        name: "only",
        description: "Close all other windows",
        availability: Availability::Common,
        min_abbrev: 2, // :on[ly]
    },
    BuiltinCommand {
        name: "tabnew",
        description: "Create new tab",
        availability: Availability::Common,
        min_abbrev: 6, // :tabnew
    },
    BuiltinCommand {
        name: "tabclose",
        description: "Close tab",
        availability: Availability::Common,
        min_abbrev: 4, // :tabc[lose]
    },
    BuiltinCommand {
        name: "tabnext",
        description: "Go to next tab",
        availability: Availability::Common,
        min_abbrev: 4, // :tabn[ext]
    },
    BuiltinCommand {
        name: "tabprevious",
        description: "Go to previous tab",
        availability: Availability::Common,
        min_abbrev: 4, // :tabp[revious]
    },
    BuiltinCommand {
        name: "write",
        description: "Write buffer to file",
        availability: Availability::Common,
        min_abbrev: 1, // :w[rite]
    },
    BuiltinCommand {
        name: "wall",
        description: "Write all buffers",
        availability: Availability::Common,
        min_abbrev: 2, // :wa[ll]
    },
    BuiltinCommand {
        name: "quit",
        description: "Quit window",
        availability: Availability::Common,
        min_abbrev: 1, // :q[uit]
    },
    BuiltinCommand {
        name: "qall",
        description: "Quit all windows",
        availability: Availability::Common,
        min_abbrev: 2, // :qa[ll]
    },
    BuiltinCommand {
        name: "wq",
        description: "Write and quit",
        availability: Availability::Common,
        min_abbrev: 2, // :wq
    },
    BuiltinCommand {
        name: "wqall",
        description: "Write all and quit",
        availability: Availability::Common,
        min_abbrev: 3, // :wqa[ll]
    },
    BuiltinCommand {
        name: "saveas",
        description: "Save buffer to new file",
        availability: Availability::Common,
        min_abbrev: 3, // :sav[eas]
    },
    BuiltinCommand {
        name: "substitute",
        description: "Search and replace",
        availability: Availability::Common,
        min_abbrev: 1, // :s[ubstitute]
    },
    BuiltinCommand {
        name: "global",
        description: "Execute command on matching lines",
        availability: Availability::Common,
        min_abbrev: 1, // :g[lobal]
    },
    BuiltinCommand {
        name: "vglobal",
        description: "Execute command on non-matching lines",
        availability: Availability::Common,
        min_abbrev: 1, // :v[global]
    },
    BuiltinCommand {
        name: "silent",
        description: "Execute command silently",
        availability: Availability::Common,
        min_abbrev: 3, // :sil[ent]
    },
    BuiltinCommand {
        name: "redraw",
        description: "Redraw screen",
        availability: Availability::Common,
        min_abbrev: 4, // :redr[aw]
    },
    BuiltinCommand {
        name: "sleep",
        description: "Pause execution",
        availability: Availability::Common,
        min_abbrev: 2, // :sl[eep]
    },
    BuiltinCommand {
        name: "filetype",
        description: "Set filetype options",
        availability: Availability::Common,
        min_abbrev: 5, // :filet[ype]
    },
    BuiltinCommand {
        name: "lua",
        description: "Execute Lua code",
        availability: Availability::NeovimOnly,
        min_abbrev: 3, // :lua (no abbreviation)
    },
    BuiltinCommand {
        name: "luado",
        description: "Execute Lua for each line",
        availability: Availability::NeovimOnly,
        min_abbrev: 4, // :luad[o]
    },
    BuiltinCommand {
        name: "luafile",
        description: "Execute Lua file",
        availability: Availability::NeovimOnly,
        min_abbrev: 4, // :luaf[ile]
    },
    BuiltinCommand {
        name: "vim9script",
        description: "Start Vim9 script",
        availability: Availability::VimOnly,
        min_abbrev: 5, // :vim9s[cript]
    },
    BuiltinCommand {
        name: "!",
        description: "filter lines or execute an external command",
        availability: Availability::Common,
        min_abbrev: 1, // :!
    },
    BuiltinCommand {
        name: "!!",
        description: "repeat last \":!\" command",
        availability: Availability::Common,
        min_abbrev: 2, // :!!
    },
    BuiltinCommand {
        name: "#",
        description: "same as \":number\"",
        availability: Availability::Common,
        min_abbrev: 1, // :#
    },
    BuiltinCommand {
        name: "&",
        description: "repeat last \":substitute\"",
        availability: Availability::Common,
        min_abbrev: 1, // :&
    },
    BuiltinCommand {
        name: "2match",
        description: "define a second match to highlight",
        availability: Availability::Common,
        min_abbrev: 4, // :2mat[ch]
    },
    BuiltinCommand {
        name: "3match",
        description: "define a third match to highlight",
        availability: Availability::Common,
        min_abbrev: 4, // :3mat[ch]
    },
    BuiltinCommand {
        name: "<",
        description: "shift lines one 'shiftwidth' left",
        availability: Availability::Common,
        min_abbrev: 1, // :<
    },
    BuiltinCommand {
        name: "=",
        description: "print the last line number",
        availability: Availability::Common,
        min_abbrev: 1, // :=
    },
    BuiltinCommand {
        name: ">",
        description: "shift lines one 'shiftwidth' right",
        availability: Availability::Common,
        min_abbrev: 1, // :>
    },
    BuiltinCommand {
        name: "@",
        description: "execute contents of a register",
        availability: Availability::Common,
        min_abbrev: 1, // :@
    },
    BuiltinCommand {
        name: "@@",
        description: "repeat the previous \":@\"",
        availability: Availability::Common,
        min_abbrev: 2, // :@@
    },
    BuiltinCommand {
        name: "~",
        description: "repeat last \":substitute\"",
        availability: Availability::Common,
        min_abbrev: 1, // :~
    },
    BuiltinCommand {
        name: "Next",
        description: "go to previous file in the argument list",
        availability: Availability::Common,
        min_abbrev: 1, // :N[ext]
    },
    BuiltinCommand {
        name: "argadd",
        description: "add items to the argument list",
        availability: Availability::Common,
        min_abbrev: 4, // :arga[dd]
    },
    BuiltinCommand {
        name: "argdedupe",
        description: "remove duplicates from the argument list",
        availability: Availability::Common,
        min_abbrev: 6, // :argded[upe]
    },
    BuiltinCommand {
        name: "argdelete",
        description: "delete items from the argument list",
        availability: Availability::Common,
        min_abbrev: 4, // :argd[elete]
    },
    BuiltinCommand {
        name: "argdo",
        description: "do a command on all items in the argument list",
        availability: Availability::Common,
        min_abbrev: 5, // :argdo (no abbreviation)
    },
    BuiltinCommand {
        name: "argedit",
        description: "add item to the argument list and edit it",
        availability: Availability::Common,
        min_abbrev: 4, // :arge[dit]
    },
    BuiltinCommand {
        name: "argglobal",
        description: "define the global argument list",
        availability: Availability::Common,
        min_abbrev: 4, // :argg[lobal]
    },
    BuiltinCommand {
        name: "arglocal",
        description: "define a local argument list",
        availability: Availability::Common,
        min_abbrev: 4, // :argl[ocal]
    },
    BuiltinCommand {
        name: "args",
        description: "print the argument list",
        availability: Availability::Common,
        min_abbrev: 2, // :ar[gs]
    },
    BuiltinCommand {
        name: "argument",
        description: "go to specific file in the argument list",
        availability: Availability::Common,
        min_abbrev: 4, // :argu[ment]
    },
    BuiltinCommand {
        name: "first",
        description: "go to the first file in the argument list",
        availability: Availability::Common,
        min_abbrev: 3, // :fir[st]
    },
    BuiltinCommand {
        name: "last",
        description: "go to the last file in the argument list",
        availability: Availability::Common,
        min_abbrev: 2, // :la[st]
    },
    BuiltinCommand {
        name: "next",
        description: "go to next file in the argument list",
        availability: Availability::Common,
        min_abbrev: 1, // :n[ext]
    },
    BuiltinCommand {
        name: "previous",
        description: "go to previous file in argument list",
        availability: Availability::Common,
        min_abbrev: 4, // :prev[ious]
    },
    BuiltinCommand {
        name: "rewind",
        description: "go to the first file in the argument list",
        availability: Availability::Common,
        min_abbrev: 3, // :rew[ind]
    },
    BuiltinCommand {
        name: "bNext",
        description: "go to previous buffer in the buffer list",
        availability: Availability::Common,
        min_abbrev: 2, // :bN[ext]
    },
    BuiltinCommand {
        name: "badd",
        description: "add buffer to the buffer list",
        availability: Availability::Common,
        min_abbrev: 3, // :bad[d]
    },
    BuiltinCommand {
        name: "ball",
        description: "open a window for each buffer in the buffer list",
        availability: Availability::Common,
        min_abbrev: 2, // :ba[ll]
    },
    BuiltinCommand {
        name: "balt",
        description: "like \":badd\" but also set the alternate file",
        availability: Availability::Common,
        min_abbrev: 4, // :balt (no abbreviation)
    },
    BuiltinCommand {
        name: "bfirst",
        description: "go to first buffer in the buffer list",
        availability: Availability::Common,
        min_abbrev: 2, // :bf[irst]
    },
    BuiltinCommand {
        name: "blast",
        description: "go to last buffer in the buffer list",
        availability: Availability::Common,
        min_abbrev: 2, // :bl[ast]
    },
    BuiltinCommand {
        name: "bmodified",
        description: "go to next buffer in the buffer list that has been modified",
        availability: Availability::Common,
        min_abbrev: 2, // :bm[odified]
    },
    BuiltinCommand {
        name: "bnext",
        description: "go to next buffer in the buffer list",
        availability: Availability::Common,
        min_abbrev: 2, // :bn[ext]
    },
    BuiltinCommand {
        name: "bprevious",
        description: "go to previous buffer in the buffer list",
        availability: Availability::Common,
        min_abbrev: 2, // :bp[revious]
    },
    BuiltinCommand {
        name: "brewind",
        description: "go to first buffer in the buffer list",
        availability: Availability::Common,
        min_abbrev: 2, // :br[ewind]
    },
    BuiltinCommand {
        name: "bufdo",
        description: "execute command in each listed buffer",
        availability: Availability::Common,
        min_abbrev: 4, // :bufd[o]
    },
    BuiltinCommand {
        name: "buffers",
        description: "list all files in the buffer list",
        availability: Availability::Common,
        min_abbrev: 7, // :buffers (no abbreviation)
    },
    BuiltinCommand {
        name: "bunload",
        description: "unload a specific buffer",
        availability: Availability::Common,
        min_abbrev: 3, // :bun[load]
    },
    BuiltinCommand {
        name: "files",
        description: "list all files in the buffer list",
        availability: Availability::Common,
        min_abbrev: 5, // :files (no abbreviation)
    },
    BuiltinCommand {
        name: "ls",
        description: "list all buffers",
        availability: Availability::Common,
        min_abbrev: 2, // :ls (no abbreviation)
    },
    BuiltinCommand {
        name: "aboveleft",
        description: "make split window appear left or above",
        availability: Availability::Common,
        min_abbrev: 3, // :abo[veleft]
    },
    BuiltinCommand {
        name: "belowright",
        description: "make split window appear right or below",
        availability: Availability::Common,
        min_abbrev: 3, // :bel[owright]
    },
    BuiltinCommand {
        name: "botright",
        description: "make split window appear at bottom or far right",
        availability: Availability::Common,
        min_abbrev: 2, // :bo[tright]
    },
    BuiltinCommand {
        name: "browse",
        description: "use file selection dialog",
        availability: Availability::Common,
        min_abbrev: 3, // :bro[wse]
    },
    BuiltinCommand {
        name: "confirm",
        description: "prompt user when confirmation required",
        availability: Availability::Common,
        min_abbrev: 4, // :conf[irm]
    },
    BuiltinCommand {
        name: "hide",
        description: "hide current buffer for a command",
        availability: Availability::Common,
        min_abbrev: 3, // :hid[e]
    },
    BuiltinCommand {
        name: "horizontal",
        description: "following window command work horizontally",
        availability: Availability::Common,
        min_abbrev: 3, // :hor[izontal]
    },
    BuiltinCommand {
        name: "leftabove",
        description: "make split window appear left or above",
        availability: Availability::Common,
        min_abbrev: 5, // :lefta[bove]
    },
    BuiltinCommand {
        name: "rightbelow",
        description: "make split window appear right or below",
        availability: Availability::Common,
        min_abbrev: 6, // :rightb[elow]
    },
    BuiltinCommand {
        name: "tab",
        description: "create new tab when opening new window",
        availability: Availability::Common,
        min_abbrev: 3, // :tab
    },
    BuiltinCommand {
        name: "topleft",
        description: "make split window appear at top or far left",
        availability: Availability::Common,
        min_abbrev: 2, // :to[pleft]
    },
    BuiltinCommand {
        name: "vertical",
        description: "make following command split vertically",
        availability: Availability::Common,
        min_abbrev: 4, // :vert[ical]
    },
    BuiltinCommand {
        name: "cNext",
        description: "go to previous error",
        availability: Availability::Common,
        min_abbrev: 2, // :cN[ext]
    },
    BuiltinCommand {
        name: "cNfile",
        description: "go to last error in previous file",
        availability: Availability::Common,
        min_abbrev: 3, // :cNf[ile]
    },
    BuiltinCommand {
        name: "cabove",
        description: "go to error above current line",
        availability: Availability::Common,
        min_abbrev: 4, // :cabo[ve]
    },
    BuiltinCommand {
        name: "caddbuffer",
        description: "add errors from buffer",
        availability: Availability::Common,
        min_abbrev: 3, // :cad[dbuffer]
    },
    BuiltinCommand {
        name: "caddexpr",
        description: "add errors from expr",
        availability: Availability::Common,
        min_abbrev: 5, // :cadde[xpr]
    },
    BuiltinCommand {
        name: "caddfile",
        description: "add error message to current quickfix list",
        availability: Availability::Common,
        min_abbrev: 5, // :caddf[ile]
    },
    BuiltinCommand {
        name: "cafter",
        description: "go to error after current cursor",
        availability: Availability::Common,
        min_abbrev: 3, // :caf[ter]
    },
    BuiltinCommand {
        name: "cbefore",
        description: "go to error before current cursor",
        availability: Availability::Common,
        min_abbrev: 3, // :cbe[fore]
    },
    BuiltinCommand {
        name: "cbelow",
        description: "go to error below current line",
        availability: Availability::Common,
        min_abbrev: 4, // :cbel[ow]
    },
    BuiltinCommand {
        name: "cbottom",
        description: "scroll to the bottom of the quickfix window",
        availability: Availability::Common,
        min_abbrev: 3, // :cbo[ttom]
    },
    BuiltinCommand {
        name: "cbuffer",
        description: "parse error messages and jump to first error",
        availability: Availability::Common,
        min_abbrev: 2, // :cb[uffer]
    },
    BuiltinCommand {
        name: "cc",
        description: "go to specific error",
        availability: Availability::Common,
        min_abbrev: 2, // :cc
    },
    BuiltinCommand {
        name: "cclose",
        description: "close quickfix window",
        availability: Availability::Common,
        min_abbrev: 3, // :ccl[ose]
    },
    BuiltinCommand {
        name: "cdo",
        description: "execute command in each valid error list entry",
        availability: Availability::Common,
        min_abbrev: 3, // :cdo
    },
    BuiltinCommand {
        name: "cexpr",
        description: "read errors from expr and jump to first",
        availability: Availability::Common,
        min_abbrev: 3, // :cex[pr]
    },
    BuiltinCommand {
        name: "cfdo",
        description: "execute command in each file in error list",
        availability: Availability::Common,
        min_abbrev: 3, // :cfd[o]
    },
    BuiltinCommand {
        name: "cfile",
        description: "read file with error messages and jump to first",
        availability: Availability::Common,
        min_abbrev: 2, // :cf[ile]
    },
    BuiltinCommand {
        name: "cfirst",
        description: "go to the specified error, default first one",
        availability: Availability::Common,
        min_abbrev: 4, // :cfir[st]
    },
    BuiltinCommand {
        name: "cgetbuffer",
        description: "get errors from buffer",
        availability: Availability::Common,
        min_abbrev: 5, // :cgetb[uffer]
    },
    BuiltinCommand {
        name: "cgetexpr",
        description: "get errors from expr",
        availability: Availability::Common,
        min_abbrev: 5, // :cgete[xpr]
    },
    BuiltinCommand {
        name: "cgetfile",
        description: "read file with error messages",
        availability: Availability::Common,
        min_abbrev: 2, // :cg[etfile]
    },
    BuiltinCommand {
        name: "chistory",
        description: "list the error lists",
        availability: Availability::Common,
        min_abbrev: 3, // :chi[story]
    },
    BuiltinCommand {
        name: "clast",
        description: "go to the specified error, default last one",
        availability: Availability::Common,
        min_abbrev: 3, // :cla[st]
    },
    BuiltinCommand {
        name: "clist",
        description: "list all errors",
        availability: Availability::Common,
        min_abbrev: 2, // :cl[ist]
    },
    BuiltinCommand {
        name: "cnewer",
        description: "go to newer error list",
        availability: Availability::Common,
        min_abbrev: 4, // :cnew[er]
    },
    BuiltinCommand {
        name: "cnext",
        description: "go to next error",
        availability: Availability::Common,
        min_abbrev: 2, // :cn[ext]
    },
    BuiltinCommand {
        name: "cnfile",
        description: "go to first error in next file",
        availability: Availability::Common,
        min_abbrev: 3, // :cnf[ile]
    },
    BuiltinCommand {
        name: "colder",
        description: "go to older error list",
        availability: Availability::Common,
        min_abbrev: 3, // :col[der]
    },
    BuiltinCommand {
        name: "copen",
        description: "open quickfix window",
        availability: Availability::Common,
        min_abbrev: 4, // :cope[n]
    },
    BuiltinCommand {
        name: "cpfile",
        description: "go to last error in previous file",
        availability: Availability::Common,
        min_abbrev: 3, // :cpf[ile]
    },
    BuiltinCommand {
        name: "cprevious",
        description: "go to previous error",
        availability: Availability::Common,
        min_abbrev: 2, // :cp[revious]
    },
    BuiltinCommand {
        name: "cquit",
        description: "quit Vim with an error code",
        availability: Availability::Common,
        min_abbrev: 2, // :cq[uit]
    },
    BuiltinCommand {
        name: "crewind",
        description: "go to the specified error, default first one",
        availability: Availability::Common,
        min_abbrev: 2, // :cr[ewind]
    },
    BuiltinCommand {
        name: "cwindow",
        description: "open or close quickfix window",
        availability: Availability::Common,
        min_abbrev: 2, // :cw[indow]
    },
    BuiltinCommand {
        name: "lNext",
        description: "go to previous entry in location list",
        availability: Availability::Common,
        min_abbrev: 2, // :lN[ext]
    },
    BuiltinCommand {
        name: "lNfile",
        description: "go to last entry in previous file",
        availability: Availability::Common,
        min_abbrev: 3, // :lNf[ile]
    },
    BuiltinCommand {
        name: "labove",
        description: "go to location above current line",
        availability: Availability::Common,
        min_abbrev: 3, // :lab[ove]
    },
    BuiltinCommand {
        name: "laddbuffer",
        description: "add locations from buffer",
        availability: Availability::Common,
        min_abbrev: 5, // :laddb[uffer]
    },
    BuiltinCommand {
        name: "laddexpr",
        description: "add locations from expr",
        availability: Availability::Common,
        min_abbrev: 3, // :lad[dexpr]
    },
    BuiltinCommand {
        name: "laddfile",
        description: "add locations to current location list",
        availability: Availability::Common,
        min_abbrev: 5, // :laddf[ile]
    },
    BuiltinCommand {
        name: "lafter",
        description: "go to location after current cursor",
        availability: Availability::Common,
        min_abbrev: 3, // :laf[ter]
    },
    BuiltinCommand {
        name: "lbefore",
        description: "go to location before current cursor",
        availability: Availability::Common,
        min_abbrev: 3, // :lbe[fore]
    },
    BuiltinCommand {
        name: "lbelow",
        description: "go to location below current line",
        availability: Availability::Common,
        min_abbrev: 4, // :lbel[ow]
    },
    BuiltinCommand {
        name: "lbottom",
        description: "scroll to the bottom of the location window",
        availability: Availability::Common,
        min_abbrev: 3, // :lbo[ttom]
    },
    BuiltinCommand {
        name: "lbuffer",
        description: "parse locations and jump to first location",
        availability: Availability::Common,
        min_abbrev: 2, // :lb[uffer]
    },
    BuiltinCommand {
        name: "lclose",
        description: "close location window",
        availability: Availability::Common,
        min_abbrev: 3, // :lcl[ose]
    },
    BuiltinCommand {
        name: "ldo",
        description: "execute command in valid location list entries",
        availability: Availability::Common,
        min_abbrev: 2, // :ld[o]
    },
    BuiltinCommand {
        name: "lexpr",
        description: "read locations from expr and jump to first",
        availability: Availability::Common,
        min_abbrev: 3, // :lex[pr]
    },
    BuiltinCommand {
        name: "lfdo",
        description: "execute command in each file in location list",
        availability: Availability::Common,
        min_abbrev: 3, // :lfd[o]
    },
    BuiltinCommand {
        name: "lfile",
        description: "read file with locations and jump to first",
        availability: Availability::Common,
        min_abbrev: 2, // :lf[ile]
    },
    BuiltinCommand {
        name: "lfirst",
        description: "go to the specified location, default first one",
        availability: Availability::Common,
        min_abbrev: 4, // :lfir[st]
    },
    BuiltinCommand {
        name: "lgetbuffer",
        description: "get locations from buffer",
        availability: Availability::Common,
        min_abbrev: 5, // :lgetb[uffer]
    },
    BuiltinCommand {
        name: "lgetexpr",
        description: "get locations from expr",
        availability: Availability::Common,
        min_abbrev: 5, // :lgete[xpr]
    },
    BuiltinCommand {
        name: "lgetfile",
        description: "read file with locations",
        availability: Availability::Common,
        min_abbrev: 2, // :lg[etfile]
    },
    BuiltinCommand {
        name: "lgrep",
        description: "run 'grepprg' and jump to first match",
        availability: Availability::Common,
        min_abbrev: 3, // :lgr[ep]
    },
    BuiltinCommand {
        name: "lgrepadd",
        description: "like :grep, but append to current list",
        availability: Availability::Common,
        min_abbrev: 6, // :lgrepa[dd]
    },
    BuiltinCommand {
        name: "lhelpgrep",
        description: "like \":helpgrep\" but uses location list",
        availability: Availability::Common,
        min_abbrev: 2, // :lh[elpgrep]
    },
    BuiltinCommand {
        name: "lhistory",
        description: "list the location lists",
        availability: Availability::Common,
        min_abbrev: 3, // :lhi[story]
    },
    BuiltinCommand {
        name: "ll",
        description: "go to specific location",
        availability: Availability::Common,
        min_abbrev: 2, // :ll
    },
    BuiltinCommand {
        name: "llast",
        description: "go to the specified location, default last one",
        availability: Availability::Common,
        min_abbrev: 3, // :lla[st]
    },
    BuiltinCommand {
        name: "llist",
        description: "list all locations",
        availability: Availability::Common,
        min_abbrev: 3, // :lli[st]
    },
    BuiltinCommand {
        name: "lmake",
        description: "execute external command 'makeprg' and parse",
        availability: Availability::Common,
        min_abbrev: 4, // :lmak[e]
    },
    BuiltinCommand {
        name: "lnewer",
        description: "go to newer location list",
        availability: Availability::Common,
        min_abbrev: 4, // :lnew[er]
    },
    BuiltinCommand {
        name: "lnext",
        description: "go to next location",
        availability: Availability::Common,
        min_abbrev: 3, // :lne[xt]
    },
    BuiltinCommand {
        name: "lnfile",
        description: "go to first location in next file",
        availability: Availability::Common,
        min_abbrev: 3, // :lnf[ile]
    },
    BuiltinCommand {
        name: "lolder",
        description: "go to older location list",
        availability: Availability::Common,
        min_abbrev: 3, // :lol[der]
    },
    BuiltinCommand {
        name: "lopen",
        description: "open location window",
        availability: Availability::Common,
        min_abbrev: 3, // :lop[en]
    },
    BuiltinCommand {
        name: "lpfile",
        description: "go to last location in previous file",
        availability: Availability::Common,
        min_abbrev: 3, // :lpf[ile]
    },
    BuiltinCommand {
        name: "lprevious",
        description: "go to previous location",
        availability: Availability::Common,
        min_abbrev: 2, // :lp[revious]
    },
    BuiltinCommand {
        name: "lrewind",
        description: "go to the specified location, default first one",
        availability: Availability::Common,
        min_abbrev: 2, // :lr[ewind]
    },
    BuiltinCommand {
        name: "ltag",
        description: "jump to tag and add matching tags to the location list",
        availability: Availability::Common,
        min_abbrev: 2, // :lt[ag]
    },
    BuiltinCommand {
        name: "lvimgrep",
        description: "search for pattern in files",
        availability: Availability::Common,
        min_abbrev: 2, // :lv[imgrep]
    },
    BuiltinCommand {
        name: "lvimgrepadd",
        description: "like :vimgrep, but append to current list",
        availability: Availability::Common,
        min_abbrev: 8, // :lvimgrepa[dd]
    },
    BuiltinCommand {
        name: "lwindow",
        description: "open or close location window",
        availability: Availability::Common,
        min_abbrev: 2, // :lw[indow]
    },
    BuiltinCommand {
        name: "cd",
        description: "change directory",
        availability: Availability::Common,
        min_abbrev: 2, // :cd
    },
    BuiltinCommand {
        name: "chdir",
        description: "change directory",
        availability: Availability::Common,
        min_abbrev: 3, // :chd[ir]
    },
    BuiltinCommand {
        name: "lcd",
        description: "change directory locally",
        availability: Availability::Common,
        min_abbrev: 2, // :lc[d]
    },
    BuiltinCommand {
        name: "lchdir",
        description: "change directory locally",
        availability: Availability::Common,
        min_abbrev: 3, // :lch[dir]
    },
    BuiltinCommand {
        name: "pwd",
        description: "print current directory",
        availability: Availability::Common,
        min_abbrev: 2, // :pw[d]
    },
    BuiltinCommand {
        name: "tcd",
        description: "change directory for tab page",
        availability: Availability::Common,
        min_abbrev: 2, // :tc[d]
    },
    BuiltinCommand {
        name: "tchdir",
        description: "change directory for tab page",
        availability: Availability::Common,
        min_abbrev: 3, // :tch[dir]
    },
    BuiltinCommand {
        name: "Print",
        description: "print lines",
        availability: Availability::Common,
        min_abbrev: 1, // :P[rint]
    },
    BuiltinCommand {
        name: "append",
        description: "append text",
        availability: Availability::Common,
        min_abbrev: 1, // :a[ppend]
    },
    BuiltinCommand {
        name: "center",
        description: "format lines at the center",
        availability: Availability::Common,
        min_abbrev: 2, // :ce[nter]
    },
    BuiltinCommand {
        name: "change",
        description: "replace a line or series of lines",
        availability: Availability::Common,
        min_abbrev: 1, // :c[hange]
    },
    BuiltinCommand {
        name: "copy",
        description: "copy lines",
        availability: Availability::Common,
        min_abbrev: 2, // :co[py]
    },
    BuiltinCommand {
        name: "delete",
        description: "delete lines",
        availability: Availability::Common,
        min_abbrev: 1, // :d[elete]
    },
    BuiltinCommand {
        name: "insert",
        description: "insert text",
        availability: Availability::Common,
        min_abbrev: 1, // :i[nsert]
    },
    BuiltinCommand {
        name: "join",
        description: "join lines",
        availability: Availability::Common,
        min_abbrev: 1, // :j[oin]
    },
    BuiltinCommand {
        name: "left",
        description: "left align lines",
        availability: Availability::Common,
        min_abbrev: 2, // :le[ft]
    },
    BuiltinCommand {
        name: "list",
        description: "print lines",
        availability: Availability::Common,
        min_abbrev: 1, // :l[ist]
    },
    BuiltinCommand {
        name: "move",
        description: "move lines",
        availability: Availability::Common,
        min_abbrev: 1, // :m[ove]
    },
    BuiltinCommand {
        name: "number",
        description: "print lines with line number",
        availability: Availability::Common,
        min_abbrev: 2, // :nu[mber]
    },
    BuiltinCommand {
        name: "print",
        description: "print lines",
        availability: Availability::Common,
        min_abbrev: 1, // :p[rint]
    },
    BuiltinCommand {
        name: "put",
        description: "insert contents of register in the text",
        availability: Availability::Common,
        min_abbrev: 2, // :pu[t]
    },
    BuiltinCommand {
        name: "read",
        description: "read file into the text",
        availability: Availability::Common,
        min_abbrev: 1, // :r[ead]
    },
    BuiltinCommand {
        name: "retab",
        description: "change tab size",
        availability: Availability::Common,
        min_abbrev: 3, // :ret[ab]
    },
    BuiltinCommand {
        name: "right",
        description: "right align text",
        availability: Availability::Common,
        min_abbrev: 2, // :ri[ght]
    },
    BuiltinCommand {
        name: "sort",
        description: "sort lines",
        availability: Availability::Common,
        min_abbrev: 3, // :sor[t]
    },
    BuiltinCommand {
        name: "t",
        description: "same as \":copy\"",
        availability: Availability::Common,
        min_abbrev: 1, // :t
    },
    BuiltinCommand {
        name: "uniq",
        description: "uniq lines",
        availability: Availability::Common,
        min_abbrev: 3, // :uni[q]
    },
    BuiltinCommand {
        name: "yank",
        description: "yank lines into a register",
        availability: Availability::Common,
        min_abbrev: 1, // :y[ank]
    },
    BuiltinCommand {
        name: "z",
        description: "print some lines",
        availability: Availability::Common,
        min_abbrev: 1, // :z
    },
    BuiltinCommand {
        name: "earlier",
        description: "go to older change, undo",
        availability: Availability::Common,
        min_abbrev: 2, // :ea[rlier]
    },
    BuiltinCommand {
        name: "later",
        description: "go to newer change, redo",
        availability: Availability::Common,
        min_abbrev: 3, // :lat[er]
    },
    BuiltinCommand {
        name: "redo",
        description: "redo one undone change",
        availability: Availability::Common,
        min_abbrev: 3, // :red[o]
    },
    BuiltinCommand {
        name: "rundo",
        description: "read undo information from a file",
        availability: Availability::Common,
        min_abbrev: 4, // :rund[o]
    },
    BuiltinCommand {
        name: "undo",
        description: "undo last change(s)",
        availability: Availability::Common,
        min_abbrev: 1, // :u[ndo]
    },
    BuiltinCommand {
        name: "undojoin",
        description: "join next change with previous undo block",
        availability: Availability::Common,
        min_abbrev: 5, // :undoj[oin]
    },
    BuiltinCommand {
        name: "undolist",
        description: "list leafs of the undo tree",
        availability: Availability::Common,
        min_abbrev: 5, // :undol[ist]
    },
    BuiltinCommand {
        name: "wundo",
        description: "write undo information to a file",
        availability: Availability::Common,
        min_abbrev: 2, // :wu[ndo]
    },
    BuiltinCommand {
        name: "changes",
        description: "print the change list",
        availability: Availability::Common,
        min_abbrev: 7, // :changes
    },
    BuiltinCommand {
        name: "clearjumps",
        description: "clear the jump list",
        availability: Availability::Common,
        min_abbrev: 3, // :cle[arjumps]
    },
    BuiltinCommand {
        name: "delmarks",
        description: "delete marks",
        availability: Availability::Common,
        min_abbrev: 4, // :delm[arks]
    },
    BuiltinCommand {
        name: "jumps",
        description: "print the jump list",
        availability: Availability::Common,
        min_abbrev: 2, // :ju[mps]
    },
    BuiltinCommand {
        name: "k",
        description: "set a mark",
        availability: Availability::Common,
        min_abbrev: 1, // :k
    },
    BuiltinCommand {
        name: "mark",
        description: "set a mark",
        availability: Availability::Common,
        min_abbrev: 2, // :ma[rk]
    },
    BuiltinCommand {
        name: "marks",
        description: "list all marks",
        availability: Availability::Common,
        min_abbrev: 5, // :marks
    },
    BuiltinCommand {
        name: "ascii",
        description: "print ascii value of character under the cursor",
        availability: Availability::Common,
        min_abbrev: 2, // :as[cii]
    },
    BuiltinCommand {
        name: "display",
        description: "display registers",
        availability: Availability::Common,
        min_abbrev: 2, // :di[splay]
    },
    BuiltinCommand {
        name: "registers",
        description: "display the contents of registers",
        availability: Availability::Common,
        min_abbrev: 3, // :reg[isters]
    },
    BuiltinCommand {
        name: "cscope",
        description: "execute cscope command",
        availability: Availability::Common,
        min_abbrev: 2, // :cs[cope]
    },
    BuiltinCommand {
        name: "cstag",
        description: "use cscope to jump to a tag",
        availability: Availability::Common,
        min_abbrev: 3, // :cst[ag]
    },
    BuiltinCommand {
        name: "lcscope",
        description: "like \":cscope\" but uses location list",
        availability: Availability::Common,
        min_abbrev: 3, // :lcs[cope]
    },
    BuiltinCommand {
        name: "pop",
        description: "jump to older entry in tag stack",
        availability: Availability::Common,
        min_abbrev: 2, // :po[p]
    },
    BuiltinCommand {
        name: "ptNext",
        description: ":tNext in preview window",
        availability: Availability::Common,
        min_abbrev: 3, // :ptN[ext]
    },
    BuiltinCommand {
        name: "ptag",
        description: "show tag in preview window",
        availability: Availability::Common,
        min_abbrev: 2, // :pt[ag]
    },
    BuiltinCommand {
        name: "ptfirst",
        description: ":trewind in preview window",
        availability: Availability::Common,
        min_abbrev: 3, // :ptf[irst]
    },
    BuiltinCommand {
        name: "ptjump",
        description: ":tjump and show tag in preview window",
        availability: Availability::Common,
        min_abbrev: 3, // :ptj[ump]
    },
    BuiltinCommand {
        name: "ptlast",
        description: ":tlast in preview window",
        availability: Availability::Common,
        min_abbrev: 3, // :ptl[ast]
    },
    BuiltinCommand {
        name: "ptnext",
        description: ":tnext in preview window",
        availability: Availability::Common,
        min_abbrev: 3, // :ptn[ext]
    },
    BuiltinCommand {
        name: "ptprevious",
        description: ":tprevious in preview window",
        availability: Availability::Common,
        min_abbrev: 3, // :ptp[revious]
    },
    BuiltinCommand {
        name: "ptrewind",
        description: ":trewind in preview window",
        availability: Availability::Common,
        min_abbrev: 3, // :ptr[ewind]
    },
    BuiltinCommand {
        name: "ptselect",
        description: ":tselect and show tag in preview window",
        availability: Availability::Common,
        min_abbrev: 3, // :pts[elect]
    },
    BuiltinCommand {
        name: "scscope",
        description: "split window and execute cscope command",
        availability: Availability::Common,
        min_abbrev: 3, // :scs[cope]
    },
    BuiltinCommand {
        name: "stag",
        description: "split window and jump to a tag",
        availability: Availability::Common,
        min_abbrev: 3, // :sta[g]
    },
    BuiltinCommand {
        name: "stjump",
        description: "do \":tjump\" and split window",
        availability: Availability::Common,
        min_abbrev: 3, // :stj[ump]
    },
    BuiltinCommand {
        name: "stselect",
        description: "do \":tselect\" and split window",
        availability: Availability::Common,
        min_abbrev: 3, // :sts[elect]
    },
    BuiltinCommand {
        name: "tNext",
        description: "jump to previous matching tag",
        availability: Availability::Common,
        min_abbrev: 2, // :tN[ext]
    },
    BuiltinCommand {
        name: "tag",
        description: "jump to tag",
        availability: Availability::Common,
        min_abbrev: 2, // :ta[g]
    },
    BuiltinCommand {
        name: "tags",
        description: "show the contents of the tag stack",
        availability: Availability::Common,
        min_abbrev: 4, // :tags
    },
    BuiltinCommand {
        name: "tfirst",
        description: "jump to first matching tag",
        availability: Availability::Common,
        min_abbrev: 2, // :tf[irst]
    },
    BuiltinCommand {
        name: "tjump",
        description: "like \":tselect\", but jump directly when there is only one match",
        availability: Availability::Common,
        min_abbrev: 2, // :tj[ump]
    },
    BuiltinCommand {
        name: "tlast",
        description: "jump to last matching tag",
        availability: Availability::Common,
        min_abbrev: 2, // :tl[ast]
    },
    BuiltinCommand {
        name: "tnext",
        description: "jump to next matching tag",
        availability: Availability::Common,
        min_abbrev: 2, // :tn[ext]
    },
    BuiltinCommand {
        name: "tprevious",
        description: "jump to previous matching tag",
        availability: Availability::Common,
        min_abbrev: 2, // :tp[revious]
    },
    BuiltinCommand {
        name: "trewind",
        description: "jump to first matching tag",
        availability: Availability::Common,
        min_abbrev: 2, // :tr[ewind]
    },
    BuiltinCommand {
        name: "tselect",
        description: "list matching tags and select one",
        availability: Availability::Common,
        min_abbrev: 2, // :ts[elect]
    },
    BuiltinCommand {
        name: "tabNext",
        description: "go to previous tab page",
        availability: Availability::Common,
        min_abbrev: 4, // :tabN[ext]
    },
    BuiltinCommand {
        name: "tabdo",
        description: "execute command in each tab page",
        availability: Availability::Common,
        min_abbrev: 4, // :tabd[o]
    },
    BuiltinCommand {
        name: "tabedit",
        description: "edit a file in a new tab page",
        availability: Availability::Common,
        min_abbrev: 4, // :tabe[dit]
    },
    BuiltinCommand {
        name: "tabfind",
        description: "find file in 'path', edit it in a new tab page",
        availability: Availability::Common,
        min_abbrev: 4, // :tabf[ind]
    },
    BuiltinCommand {
        name: "tabfirst",
        description: "go to first tab page",
        availability: Availability::Common,
        min_abbrev: 6, // :tabfir[st]
    },
    BuiltinCommand {
        name: "tablast",
        description: "go to last tab page",
        availability: Availability::Common,
        min_abbrev: 4, // :tabl[ast]
    },
    BuiltinCommand {
        name: "tabmove",
        description: "move tab page to other position",
        availability: Availability::Common,
        min_abbrev: 4, // :tabm[ove]
    },
    BuiltinCommand {
        name: "tabonly",
        description: "close all tab pages except the current one",
        availability: Availability::Common,
        min_abbrev: 4, // :tabo[nly]
    },
    BuiltinCommand {
        name: "tabrewind",
        description: "go to first tab page",
        availability: Availability::Common,
        min_abbrev: 4, // :tabr[ewind]
    },
    BuiltinCommand {
        name: "tabs",
        description: "list the tab pages and what they contain",
        availability: Availability::Common,
        min_abbrev: 4, // :tabs
    },
    BuiltinCommand {
        name: "all",
        description: "open a window for each file in the argument list",
        availability: Availability::Common,
        min_abbrev: 2, // :al[l]
    },
    BuiltinCommand {
        name: "pbuffer",
        description: "edit buffer in the preview window",
        availability: Availability::Common,
        min_abbrev: 2, // :pb[uffer]
    },
    BuiltinCommand {
        name: "pclose",
        description: "close preview window",
        availability: Availability::Common,
        min_abbrev: 2, // :pc[lose]
    },
    BuiltinCommand {
        name: "pedit",
        description: "edit file in the preview window",
        availability: Availability::Common,
        min_abbrev: 3, // :ped[it]
    },
    BuiltinCommand {
        name: "ppop",
        description: "\":pop\" in preview window",
        availability: Availability::Common,
        min_abbrev: 2, // :pp[op]
    },
    BuiltinCommand {
        name: "psearch",
        description: "like \":ijump\" but shows match in preview window",
        availability: Availability::Common,
        min_abbrev: 2, // :ps[earch]
    },
    BuiltinCommand {
        name: "resize",
        description: "change current window height",
        availability: Availability::Common,
        min_abbrev: 3, // :res[ize]
    },
    BuiltinCommand {
        name: "sNext",
        description: "split window and go to previous file in argument list",
        availability: Availability::Common,
        min_abbrev: 2, // :sN[ext]
    },
    BuiltinCommand {
        name: "sall",
        description: "open a window for each file in argument list",
        availability: Availability::Common,
        min_abbrev: 3, // :sal[l]
    },
    BuiltinCommand {
        name: "sargument",
        description: "split window and go to specific file in argument list",
        availability: Availability::Common,
        min_abbrev: 2, // :sa[rgument]
    },
    BuiltinCommand {
        name: "sbNext",
        description: "split window and go to previous file in the buffer list",
        availability: Availability::Common,
        min_abbrev: 3, // :sbN[ext]
    },
    BuiltinCommand {
        name: "sball",
        description: "open a window for each file in the buffer list",
        availability: Availability::Common,
        min_abbrev: 3, // :sba[ll]
    },
    BuiltinCommand {
        name: "sbfirst",
        description: "split window and go to first file in the buffer list",
        availability: Availability::Common,
        min_abbrev: 3, // :sbf[irst]
    },
    BuiltinCommand {
        name: "sblast",
        description: "split window and go to last file in buffer list",
        availability: Availability::Common,
        min_abbrev: 3, // :sbl[ast]
    },
    BuiltinCommand {
        name: "sbmodified",
        description: "split window and go to modified file in the buffer list",
        availability: Availability::Common,
        min_abbrev: 3, // :sbm[odified]
    },
    BuiltinCommand {
        name: "sbnext",
        description: "split window and go to next file in the buffer list",
        availability: Availability::Common,
        min_abbrev: 3, // :sbn[ext]
    },
    BuiltinCommand {
        name: "sbprevious",
        description: "split window and go to previous file in the buffer list",
        availability: Availability::Common,
        min_abbrev: 3, // :sbp[revious]
    },
    BuiltinCommand {
        name: "sbrewind",
        description: "split window and go to first file in the buffer list",
        availability: Availability::Common,
        min_abbrev: 3, // :sbr[ewind]
    },
    BuiltinCommand {
        name: "sbuffer",
        description: "split window and go to specific file in the buffer list",
        availability: Availability::Common,
        min_abbrev: 2, // :sb[uffer]
    },
    BuiltinCommand {
        name: "sfind",
        description: "split current window and edit file in 'path'",
        availability: Availability::Common,
        min_abbrev: 2, // :sf[ind]
    },
    BuiltinCommand {
        name: "sfirst",
        description: "split window and go to first file in the argument list",
        availability: Availability::Common,
        min_abbrev: 4, // :sfir[st]
    },
    BuiltinCommand {
        name: "slast",
        description: "split window and go to last file in the argument list",
        availability: Availability::Common,
        min_abbrev: 3, // :sla[st]
    },
    BuiltinCommand {
        name: "snext",
        description: "split window and go to next file in the argument list",
        availability: Availability::Common,
        min_abbrev: 2, // :sn[ext]
    },
    BuiltinCommand {
        name: "sprevious",
        description: "split window and go to previous file in the argument list",
        availability: Availability::Common,
        min_abbrev: 3, // :spr[evious]
    },
    BuiltinCommand {
        name: "srewind",
        description: "split window and go to first file in the argument list",
        availability: Availability::Common,
        min_abbrev: 3, // :sre[wind]
    },
    BuiltinCommand {
        name: "sunhide",
        description: "same as \":unhide\"",
        availability: Availability::Common,
        min_abbrev: 3, // :sun[hide]
    },
    BuiltinCommand {
        name: "sview",
        description: "split window and edit file read-only",
        availability: Availability::Common,
        min_abbrev: 2, // :sv[iew]
    },
    BuiltinCommand {
        name: "unhide",
        description: "open a window for each loaded file in the buffer list",
        availability: Availability::Common,
        min_abbrev: 3, // :unh[ide]
    },
    BuiltinCommand {
        name: "wincmd",
        description: "execute a Window (CTRL-W) command",
        availability: Availability::Common,
        min_abbrev: 4, // :winc[md]
    },
    BuiltinCommand {
        name: "windo",
        description: "execute command in each window",
        availability: Availability::Common,
        min_abbrev: 4, // :wind[o]
    },
    BuiltinCommand {
        name: "winpos",
        description: "get or set window position",
        availability: Availability::Common,
        min_abbrev: 4, // :winp[os]
    },
    BuiltinCommand {
        name: "winsize",
        description: "get or set window size (obsolete)",
        availability: Availability::Common,
        min_abbrev: 2, // :wi[nsize]
    },
    BuiltinCommand {
        name: "X",
        description: "ask for encryption key",
        availability: Availability::Common,
        min_abbrev: 1, // :X
    },
    BuiltinCommand {
        name: "drop",
        description: "jump to window editing file or edit file in current window",
        availability: Availability::Common,
        min_abbrev: 2, // :dr[op]
    },
    BuiltinCommand {
        name: "ex",
        description: "same as \":edit\"",
        availability: Availability::Common,
        min_abbrev: 2, // :ex
    },
    BuiltinCommand {
        name: "exit",
        description: "same as \":xit\"",
        availability: Availability::Common,
        min_abbrev: 3, // :exi[t]
    },
    BuiltinCommand {
        name: "file",
        description: "show or set the current file name",
        availability: Availability::Common,
        min_abbrev: 1, // :f[ile]
    },
    BuiltinCommand {
        name: "find",
        description: "find file in 'path' and edit it",
        availability: Availability::Common,
        min_abbrev: 3, // :fin[d]
    },
    BuiltinCommand {
        name: "loadview",
        description: "load view for current window from a file",
        availability: Availability::Common,
        min_abbrev: 2, // :lo[adview]
    },
    BuiltinCommand {
        name: "mkexrc",
        description: "write current mappings and settings to a file",
        availability: Availability::Common,
        min_abbrev: 2, // :mk[exrc]
    },
    BuiltinCommand {
        name: "mksession",
        description: "write session info to a file",
        availability: Availability::Common,
        min_abbrev: 3, // :mks[ession]
    },
    BuiltinCommand {
        name: "mkview",
        description: "write view of current window to a file",
        availability: Availability::Common,
        min_abbrev: 5, // :mkvie[w]
    },
    BuiltinCommand {
        name: "mkvimrc",
        description: "write current mappings and settings to a file",
        availability: Availability::Common,
        min_abbrev: 3, // :mkv[imrc]
    },
    BuiltinCommand {
        name: "oldfiles",
        description: "list files that have marks in the viminfo file",
        availability: Availability::Common,
        min_abbrev: 2, // :ol[dfiles]
    },
    BuiltinCommand {
        name: "preserve",
        description: "write all text to swap file",
        availability: Availability::Common,
        min_abbrev: 3, // :pre[serve]
    },
    BuiltinCommand {
        name: "quitall",
        description: "quit Vim",
        availability: Availability::Common,
        min_abbrev: 5, // :quita[ll]
    },
    BuiltinCommand {
        name: "recover",
        description: "recover a file from a swap file",
        availability: Availability::Common,
        min_abbrev: 3, // :rec[over]
    },
    BuiltinCommand {
        name: "rviminfo",
        description: "read from viminfo file",
        availability: Availability::Common,
        min_abbrev: 2, // :rv[iminfo]
    },
    BuiltinCommand {
        name: "swapname",
        description: "show the name of the current swap file",
        availability: Availability::Common,
        min_abbrev: 2, // :sw[apname]
    },
    BuiltinCommand {
        name: "update",
        description: "write buffer if modified",
        availability: Availability::Common,
        min_abbrev: 2, // :up[date]
    },
    BuiltinCommand {
        name: "view",
        description: "edit a file read-only",
        availability: Availability::Common,
        min_abbrev: 3, // :vie[w]
    },
    BuiltinCommand {
        name: "visual",
        description: "same as \":edit\", but turns off \"Ex\" mode",
        availability: Availability::Common,
        min_abbrev: 2, // :vi[sual]
    },
    BuiltinCommand {
        name: "wNext",
        description: "write to a file and go to previous file in argument list",
        availability: Availability::Common,
        min_abbrev: 2, // :wN[ext]
    },
    BuiltinCommand {
        name: "wnext",
        description: "write to a file and go to next file in argument list",
        availability: Availability::Common,
        min_abbrev: 2, // :wn[ext]
    },
    BuiltinCommand {
        name: "wprevious",
        description: "write to a file and go to previous file in argument list",
        availability: Availability::Common,
        min_abbrev: 2, // :wp[revious]
    },
    BuiltinCommand {
        name: "wviminfo",
        description: "write to viminfo file",
        availability: Availability::Common,
        min_abbrev: 2, // :wv[iminfo]
    },
    BuiltinCommand {
        name: "xall",
        description: "same as \":wqall\"",
        availability: Availability::Common,
        min_abbrev: 2, // :xa[ll]
    },
    BuiltinCommand {
        name: "xit",
        description: "write if buffer changed and close window",
        availability: Availability::Common,
        min_abbrev: 1, // :x[it]
    },
    BuiltinCommand {
        name: "diffget",
        description: "remove differences in current buffer",
        availability: Availability::Common,
        min_abbrev: 5, // :diffg[et]
    },
    BuiltinCommand {
        name: "diffoff",
        description: "switch off diff mode",
        availability: Availability::Common,
        min_abbrev: 5, // :diffo[ff]
    },
    BuiltinCommand {
        name: "diffpatch",
        description: "apply a patch and show differences",
        availability: Availability::Common,
        min_abbrev: 5, // :diffp[atch]
    },
    BuiltinCommand {
        name: "diffput",
        description: "remove differences in other buffer",
        availability: Availability::Common,
        min_abbrev: 6, // :diffpu[t]
    },
    BuiltinCommand {
        name: "diffsplit",
        description: "show differences with another file",
        availability: Availability::Common,
        min_abbrev: 5, // :diffs[plit]
    },
    BuiltinCommand {
        name: "diffthis",
        description: "make current window a diff window",
        availability: Availability::Common,
        min_abbrev: 5, // :difft[his]
    },
    BuiltinCommand {
        name: "diffupdate",
        description: "update 'diff' buffers",
        availability: Availability::Common,
        min_abbrev: 3, // :dif[fupdate]
    },
    BuiltinCommand {
        name: "fold",
        description: "create a fold",
        availability: Availability::Common,
        min_abbrev: 2, // :fo[ld]
    },
    BuiltinCommand {
        name: "foldclose",
        description: "close folds",
        availability: Availability::Common,
        min_abbrev: 5, // :foldc[lose]
    },
    BuiltinCommand {
        name: "folddoclosed",
        description: "execute command on lines in a closed fold",
        availability: Availability::Common,
        min_abbrev: 7, // :folddoc[losed]
    },
    BuiltinCommand {
        name: "folddoopen",
        description: "execute command on lines not in a closed fold",
        availability: Availability::Common,
        min_abbrev: 5, // :foldd[oopen]
    },
    BuiltinCommand {
        name: "foldopen",
        description: "open folds",
        availability: Availability::Common,
        min_abbrev: 5, // :foldo[pen]
    },
    BuiltinCommand {
        name: "abbreviate",
        description: "enter abbreviation",
        availability: Availability::Common,
        min_abbrev: 2, // :ab[breviate]
    },
    BuiltinCommand {
        name: "abclear",
        description: "remove all abbreviations",
        availability: Availability::Common,
        min_abbrev: 3, // :abc[lear]
    },
    BuiltinCommand {
        name: "cabbrev",
        description: "like \":abbreviate\" but for Command-line mode",
        availability: Availability::Common,
        min_abbrev: 2, // :ca[bbrev]
    },
    BuiltinCommand {
        name: "cabclear",
        description: "clear all abbreviations for Command-line mode",
        availability: Availability::Common,
        min_abbrev: 4, // :cabc[lear]
    },
    BuiltinCommand {
        name: "cnoreabbrev",
        description: "like \":noreabbrev\" but for Command-line mode",
        availability: Availability::Common,
        min_abbrev: 6, // :cnorea[bbrev]
    },
    BuiltinCommand {
        name: "cunabbrev",
        description: "like \":unabbrev\" but for Command-line mode",
        availability: Availability::Common,
        min_abbrev: 4, // :cuna[bbrev]
    },
    BuiltinCommand {
        name: "iabbrev",
        description: "like \":abbrev\" but for Insert mode",
        availability: Availability::Common,
        min_abbrev: 2, // :ia[bbrev]
    },
    BuiltinCommand {
        name: "iabclear",
        description: "like \":abclear\" but for Insert mode",
        availability: Availability::Common,
        min_abbrev: 4, // :iabc[lear]
    },
    BuiltinCommand {
        name: "inoreabbrev",
        description: "like \":noreabbrev\" but for Insert mode",
        availability: Availability::Common,
        min_abbrev: 6, // :inorea[bbrev]
    },
    BuiltinCommand {
        name: "iunabbrev",
        description: "like \":unabbrev\" but for Insert mode",
        availability: Availability::Common,
        min_abbrev: 4, // :iuna[bbrev]
    },
    BuiltinCommand {
        name: "noreabbrev",
        description: "enter an abbreviation that will not be remapped",
        availability: Availability::Common,
        min_abbrev: 5, // :norea[bbrev]
    },
    BuiltinCommand {
        name: "unabbreviate",
        description: "remove abbreviation",
        availability: Availability::Common,
        min_abbrev: 3, // :una[bbreviate]
    },
    BuiltinCommand {
        name: "amenu",
        description: "enter new menu item for all modes",
        availability: Availability::Common,
        min_abbrev: 2, // :am[enu]
    },
    BuiltinCommand {
        name: "anoremenu",
        description: "enter a new menu for all modes that will not be remapped",
        availability: Availability::Common,
        min_abbrev: 2, // :an[oremenu]
    },
    BuiltinCommand {
        name: "aunmenu",
        description: "remove menu for all modes",
        availability: Availability::Common,
        min_abbrev: 3, // :aun[menu]
    },
    BuiltinCommand {
        name: "cmenu",
        description: "add menu for Command-line mode",
        availability: Availability::Common,
        min_abbrev: 3, // :cme[nu]
    },
    BuiltinCommand {
        name: "cnoremenu",
        description: "like \":noremenu\" but for Command-line mode",
        availability: Availability::Common,
        min_abbrev: 7, // :cnoreme[nu]
    },
    BuiltinCommand {
        name: "cunmenu",
        description: "remove menu for Command-line mode",
        availability: Availability::Common,
        min_abbrev: 5, // :cunme[nu]
    },
    BuiltinCommand {
        name: "emenu",
        description: "execute a menu by name",
        availability: Availability::Common,
        min_abbrev: 2, // :em[enu]
    },
    BuiltinCommand {
        name: "imenu",
        description: "add menu for Insert mode",
        availability: Availability::Common,
        min_abbrev: 3, // :ime[nu]
    },
    BuiltinCommand {
        name: "inoremenu",
        description: "like \":noremenu\" but for Insert mode",
        availability: Availability::Common,
        min_abbrev: 7, // :inoreme[nu]
    },
    BuiltinCommand {
        name: "iunmenu",
        description: "remove menu for Insert mode",
        availability: Availability::Common,
        min_abbrev: 5, // :iunme[nu]
    },
    BuiltinCommand {
        name: "menu",
        description: "enter a new menu item",
        availability: Availability::Common,
        min_abbrev: 2, // :me[nu]
    },
    BuiltinCommand {
        name: "menutranslate",
        description: "add a menu translation item",
        availability: Availability::Common,
        min_abbrev: 5, // :menut[ranslate]
    },
    BuiltinCommand {
        name: "nmenu",
        description: "add menu for Normal mode",
        availability: Availability::Common,
        min_abbrev: 3, // :nme[nu]
    },
    BuiltinCommand {
        name: "nnoremenu",
        description: "like \":noremenu\" but for Normal mode",
        availability: Availability::Common,
        min_abbrev: 7, // :nnoreme[nu]
    },
    BuiltinCommand {
        name: "noremenu",
        description: "enter a menu that will not be remapped",
        availability: Availability::Common,
        min_abbrev: 6, // :noreme[nu]
    },
    BuiltinCommand {
        name: "nunmenu",
        description: "remove menu for Normal mode",
        availability: Availability::Common,
        min_abbrev: 5, // :nunme[nu]
    },
    BuiltinCommand {
        name: "omenu",
        description: "add menu for Operator-pending mode",
        availability: Availability::Common,
        min_abbrev: 3, // :ome[nu]
    },
    BuiltinCommand {
        name: "onoremenu",
        description: "like \":noremenu\" but for Operator-pending mode",
        availability: Availability::Common,
        min_abbrev: 7, // :onoreme[nu]
    },
    BuiltinCommand {
        name: "ounmenu",
        description: "remove menu for Operator-pending mode",
        availability: Availability::Common,
        min_abbrev: 5, // :ounme[nu]
    },
    BuiltinCommand {
        name: "popup",
        description: "popup a menu by name",
        availability: Availability::Common,
        min_abbrev: 4, // :popu[p]
    },
    BuiltinCommand {
        name: "smenu",
        description: "add menu for Select mode",
        availability: Availability::Common,
        min_abbrev: 3, // :sme[nu]
    },
    BuiltinCommand {
        name: "snoremenu",
        description: "like \":noremenu\" but for Select mode",
        availability: Availability::Common,
        min_abbrev: 7, // :snoreme[nu]
    },
    BuiltinCommand {
        name: "sunmenu",
        description: "remove menu for Select mode",
        availability: Availability::Common,
        min_abbrev: 5, // :sunme[nu]
    },
    BuiltinCommand {
        name: "tearoff",
        description: "tear-off a menu",
        availability: Availability::Common,
        min_abbrev: 2, // :te[aroff]
    },
    BuiltinCommand {
        name: "tlmenu",
        description: "add menu for Terminal-Job mode",
        availability: Availability::Common,
        min_abbrev: 3, // :tlm[enu]
    },
    BuiltinCommand {
        name: "tlnoremenu",
        description: "like \":noremenu\" but for Terminal-Job mode",
        availability: Availability::Common,
        min_abbrev: 3, // :tln[oremenu]
    },
    BuiltinCommand {
        name: "tlunmenu",
        description: "remove menu for Terminal-Job mode",
        availability: Availability::Common,
        min_abbrev: 3, // :tlu[nmenu]
    },
    BuiltinCommand {
        name: "tmenu",
        description: "define menu tooltip",
        availability: Availability::Common,
        min_abbrev: 2, // :tm[enu]
    },
    BuiltinCommand {
        name: "tunmenu",
        description: "remove menu tooltip",
        availability: Availability::Common,
        min_abbrev: 2, // :tu[nmenu]
    },
    BuiltinCommand {
        name: "unmenu",
        description: "remove menu",
        availability: Availability::Common,
        min_abbrev: 4, // :unme[nu]
    },
    BuiltinCommand {
        name: "vmenu",
        description: "add menu for Visual+Select mode",
        availability: Availability::Common,
        min_abbrev: 3, // :vme[nu]
    },
    BuiltinCommand {
        name: "vnoremenu",
        description: "like \":noremenu\" but for Visual+Select mode",
        availability: Availability::Common,
        min_abbrev: 7, // :vnoreme[nu]
    },
    BuiltinCommand {
        name: "vunmenu",
        description: "remove menu for Visual+Select mode",
        availability: Availability::Common,
        min_abbrev: 5, // :vunme[nu]
    },
    BuiltinCommand {
        name: "xmenu",
        description: "add menu for Visual mode",
        availability: Availability::Common,
        min_abbrev: 3, // :xme[nu]
    },
    BuiltinCommand {
        name: "xnoremenu",
        description: "like \":noremenu\" but for Visual mode",
        availability: Availability::Common,
        min_abbrev: 7, // :xnoreme[nu]
    },
    BuiltinCommand {
        name: "xunmenu",
        description: "remove menu for Visual mode",
        availability: Availability::Common,
        min_abbrev: 5, // :xunme[nu]
    },
    BuiltinCommand {
        name: "cmapclear",
        description: "clear all mappings for Command-line mode",
        availability: Availability::Common,
        min_abbrev: 5, // :cmapc[lear]
    },
    BuiltinCommand {
        name: "imapclear",
        description: "like \":mapclear\" but for Insert mode",
        availability: Availability::Common,
        min_abbrev: 5, // :imapc[lear]
    },
    BuiltinCommand {
        name: "lmapclear",
        description: "like \":mapclear!\" but includes Lang-Arg mode",
        availability: Availability::Common,
        min_abbrev: 5, // :lmapc[lear]
    },
    BuiltinCommand {
        name: "nmapclear",
        description: "clear all mappings for Normal mode",
        availability: Availability::Common,
        min_abbrev: 5, // :nmapc[lear]
    },
    BuiltinCommand {
        name: "omapclear",
        description: "remove all mappings for Operator-pending mode",
        availability: Availability::Common,
        min_abbrev: 5, // :omapc[lear]
    },
    BuiltinCommand {
        name: "smapclear",
        description: "remove all mappings for Select mode",
        availability: Availability::Common,
        min_abbrev: 5, // :smapc[lear]
    },
    BuiltinCommand {
        name: "tmapclear",
        description: "remove all mappings for Terminal-Job mode",
        availability: Availability::Common,
        min_abbrev: 5, // :tmapc[lear]
    },
    BuiltinCommand {
        name: "vmapclear",
        description: "remove all mappings for Visual+Select mode",
        availability: Availability::Common,
        min_abbrev: 5, // :vmapc[lear]
    },
    BuiltinCommand {
        name: "xmapclear",
        description: "remove all mappings for Visual mode",
        availability: Availability::Common,
        min_abbrev: 5, // :xmapc[lear]
    },
    BuiltinCommand {
        name: "exusage",
        description: "overview of Ex commands",
        availability: Availability::Common,
        min_abbrev: 3, // :exu[sage]
    },
    BuiltinCommand {
        name: "help",
        description: "open a help window",
        availability: Availability::Common,
        min_abbrev: 1, // :h[elp]
    },
    BuiltinCommand {
        name: "helpclose",
        description: "close one help window",
        availability: Availability::Common,
        min_abbrev: 5, // :helpc[lose]
    },
    BuiltinCommand {
        name: "helpfind",
        description: "dialog to open a help window",
        availability: Availability::Common,
        min_abbrev: 5, // :helpf[ind]
    },
    BuiltinCommand {
        name: "helpgrep",
        description: "like \":grep\" but searches help files",
        availability: Availability::Common,
        min_abbrev: 5, // :helpg[rep]
    },
    BuiltinCommand {
        name: "helptags",
        description: "generate help tags for a directory",
        availability: Availability::Common,
        min_abbrev: 5, // :helpt[ags]
    },
    BuiltinCommand {
        name: "intro",
        description: "print the introductory message",
        availability: Availability::Common,
        min_abbrev: 3, // :int[ro]
    },
    BuiltinCommand {
        name: "messages",
        description: "view previously displayed messages",
        availability: Availability::Common,
        min_abbrev: 3, // :mes[sages]
    },
    BuiltinCommand {
        name: "version",
        description: "print version number and other info",
        availability: Availability::Common,
        min_abbrev: 2, // :ve[rsion]
    },
    BuiltinCommand {
        name: "viusage",
        description: "overview of Normal mode commands",
        availability: Availability::Common,
        min_abbrev: 3, // :viu[sage]
    },
    BuiltinCommand {
        name: "grep",
        description: "run 'grepprg' and jump to first match",
        availability: Availability::Common,
        min_abbrev: 2, // :gr[ep]
    },
    BuiltinCommand {
        name: "grepadd",
        description: "like :grep, but append to current list",
        availability: Availability::Common,
        min_abbrev: 5, // :grepa[dd]
    },
    BuiltinCommand {
        name: "make",
        description: "execute external command 'makeprg' and parse",
        availability: Availability::Common,
        min_abbrev: 3, // :mak[e]
    },
    BuiltinCommand {
        name: "nohlsearch",
        description: "suspend 'hlsearch' highlighting",
        availability: Availability::Common,
        min_abbrev: 3, // :noh[lsearch]
    },
    BuiltinCommand {
        name: "smagic",
        description: ":substitute with 'magic'",
        availability: Availability::Common,
        min_abbrev: 2, // :sm[agic]
    },
    BuiltinCommand {
        name: "snomagic",
        description: ":substitute with 'nomagic'",
        availability: Availability::Common,
        min_abbrev: 3, // :sno[magic]
    },
    BuiltinCommand {
        name: "vimgrep",
        description: "search for pattern in files",
        availability: Availability::Common,
        min_abbrev: 3, // :vim[grep]
    },
    BuiltinCommand {
        name: "vimgrepadd",
        description: "like :vimgrep, but append to current list",
        availability: Availability::Common,
        min_abbrev: 7, // :vimgrepa[dd]
    },
    BuiltinCommand {
        name: "checkpath",
        description: "list included files",
        availability: Availability::Common,
        min_abbrev: 3, // :che[ckpath]
    },
    BuiltinCommand {
        name: "djump",
        description: "jump to #define",
        availability: Availability::Common,
        min_abbrev: 2, // :dj[ump]
    },
    BuiltinCommand {
        name: "dl",
        description: "short for :delete with the 'l' flag",
        availability: Availability::Common,
        min_abbrev: 2, // :dl
    },
    BuiltinCommand {
        name: "dlist",
        description: "list #defines",
        availability: Availability::Common,
        min_abbrev: 3, // :dli[st]
    },
    BuiltinCommand {
        name: "dp",
        description: "short for :delete with the 'p' flag",
        availability: Availability::Common,
        min_abbrev: 2, // :dp
    },
    BuiltinCommand {
        name: "dsearch",
        description: "list one #define",
        availability: Availability::Common,
        min_abbrev: 2, // :ds[earch]
    },
    BuiltinCommand {
        name: "dsplit",
        description: "split window and jump to #define",
        availability: Availability::Common,
        min_abbrev: 3, // :dsp[lit]
    },
    BuiltinCommand {
        name: "ijump",
        description: "jump to definition of identifier",
        availability: Availability::Common,
        min_abbrev: 2, // :ij[ump]
    },
    BuiltinCommand {
        name: "ilist",
        description: "list lines where identifier matches",
        availability: Availability::Common,
        min_abbrev: 2, // :il[ist]
    },
    BuiltinCommand {
        name: "iput",
        description: "like :put, but adjust the indent",
        availability: Availability::Common,
        min_abbrev: 2, // :ip[ut]
    },
    BuiltinCommand {
        name: "isearch",
        description: "list one line where identifier matches",
        availability: Availability::Common,
        min_abbrev: 2, // :is[earch]
    },
    BuiltinCommand {
        name: "isplit",
        description: "split window and jump to definition of identifier",
        availability: Availability::Common,
        min_abbrev: 3, // :isp[lit]
    },
    BuiltinCommand {
        name: "mkspell",
        description: "produce .spl spell file",
        availability: Availability::Common,
        min_abbrev: 4, // :mksp[ell]
    },
    BuiltinCommand {
        name: "spelldump",
        description: "split window and fill with all correct words",
        availability: Availability::Common,
        min_abbrev: 6, // :spelld[ump]
    },
    BuiltinCommand {
        name: "spellgood",
        description: "add good word for spelling",
        availability: Availability::Common,
        min_abbrev: 3, // :spe[llgood]
    },
    BuiltinCommand {
        name: "spellinfo",
        description: "show info about loaded spell files",
        availability: Availability::Common,
        min_abbrev: 6, // :spelli[nfo]
    },
    BuiltinCommand {
        name: "spellrare",
        description: "add rare word for spelling",
        availability: Availability::Common,
        min_abbrev: 7, // :spellra[re]
    },
    BuiltinCommand {
        name: "spellrepall",
        description: "replace all bad words like last |z=|",
        availability: Availability::Common,
        min_abbrev: 6, // :spellr[epall]
    },
    BuiltinCommand {
        name: "spellundo",
        description: "remove good or bad word",
        availability: Availability::Common,
        min_abbrev: 6, // :spellu[ndo]
    },
    BuiltinCommand {
        name: "spellwrong",
        description: "add spelling mistake",
        availability: Availability::Common,
        min_abbrev: 6, // :spellw[rong]
    },
    BuiltinCommand {
        name: "breakadd",
        description: "add a debugger breakpoint",
        availability: Availability::Common,
        min_abbrev: 6, // :breaka[dd]
    },
    BuiltinCommand {
        name: "breakdel",
        description: "delete a debugger breakpoint",
        availability: Availability::Common,
        min_abbrev: 6, // :breakd[el]
    },
    BuiltinCommand {
        name: "breaklist",
        description: "list debugger breakpoints",
        availability: Availability::Common,
        min_abbrev: 6, // :breakl[ist]
    },
    BuiltinCommand {
        name: "debug",
        description: "run a command in debugging mode",
        availability: Availability::Common,
        min_abbrev: 3, // :deb[ug]
    },
    BuiltinCommand {
        name: "debuggreedy",
        description: "read debug mode commands from normal input",
        availability: Availability::Common,
        min_abbrev: 6, // :debugg[reedy]
    },
    BuiltinCommand {
        name: "profile",
        description: "profiling functions and scripts",
        availability: Availability::Common,
        min_abbrev: 4, // :prof[ile]
    },
    BuiltinCommand {
        name: "profdel",
        description: "stop profiling a function or script",
        availability: Availability::Common,
        min_abbrev: 5, // :profd[el]
    },
    BuiltinCommand {
        name: "delfunction",
        description: "delete a user function",
        availability: Availability::Common,
        min_abbrev: 4, // :delf[unction]
    },
    BuiltinCommand {
        name: "scriptencoding",
        description: "encoding used in sourced Vim script",
        availability: Availability::Common,
        min_abbrev: 7, // :scripte[ncoding]
    },
    BuiltinCommand {
        name: "scriptnames",
        description: "list names of all sourced Vim scripts",
        availability: Availability::Common,
        min_abbrev: 3, // :scr[iptnames]
    },
    BuiltinCommand {
        name: "scriptversion",
        description: "version of Vim script used",
        availability: Availability::Common,
        min_abbrev: 7, // :scriptv[ersion]
    },
    BuiltinCommand {
        name: "behave",
        description: "set mouse and selection behavior",
        availability: Availability::Common,
        min_abbrev: 2, // :be[have]
    },
    BuiltinCommand {
        name: "checktime",
        description: "check timestamp of loaded buffers",
        availability: Availability::Common,
        min_abbrev: 6, // :checkt[ime]
    },
    BuiltinCommand {
        name: "clipreset",
        description: "reset 'clipmethod'",
        availability: Availability::Common,
        min_abbrev: 4, // :clip[reset]
    },
    BuiltinCommand {
        name: "compiler",
        description: "do settings for a specific compiler",
        availability: Availability::Common,
        min_abbrev: 4, // :comp[iler]
    },
    BuiltinCommand {
        name: "defer",
        description: "call function when current function is done",
        availability: Availability::Common,
        min_abbrev: 4, // :defe[r]
    },
    BuiltinCommand {
        name: "digraphs",
        description: "show or enter digraphs",
        availability: Availability::Common,
        min_abbrev: 3, // :dig[raphs]
    },
    BuiltinCommand {
        name: "eval",
        description: "evaluate an expression and discard the result",
        availability: Availability::Common,
        min_abbrev: 2, // :ev[al]
    },
    BuiltinCommand {
        name: "filter",
        description: "filter output of following command",
        availability: Availability::Common,
        min_abbrev: 4, // :filt[er]
    },
    BuiltinCommand {
        name: "fixdel",
        description: "set key code of <Del>",
        availability: Availability::Common,
        min_abbrev: 3, // :fix[del]
    },
    BuiltinCommand {
        name: "goto",
        description: "go to byte in the buffer",
        availability: Availability::Common,
        min_abbrev: 2, // :go[to]
    },
    BuiltinCommand {
        name: "hardcopy",
        description: "send text to the printer",
        availability: Availability::Common,
        min_abbrev: 2, // :ha[rdcopy]
    },
    BuiltinCommand {
        name: "history",
        description: "print a history list",
        availability: Availability::Common,
        min_abbrev: 3, // :his[tory]
    },
    BuiltinCommand {
        name: "keepalt",
        description: "following command keeps the alternate file",
        availability: Availability::Common,
        min_abbrev: 5, // :keepa[lt]
    },
    BuiltinCommand {
        name: "keepjumps",
        description: "following command keeps jumplist and marks",
        availability: Availability::Common,
        min_abbrev: 5, // :keepj[umps]
    },
    BuiltinCommand {
        name: "keepmarks",
        description: "following command keeps marks where they are",
        availability: Availability::Common,
        min_abbrev: 3, // :kee[pmarks]
    },
    BuiltinCommand {
        name: "keeppatterns",
        description: "following command keeps search pattern history",
        availability: Availability::Common,
        min_abbrev: 5, // :keepp[atterns]
    },
    BuiltinCommand {
        name: "language",
        description: "set the language (locale)",
        availability: Availability::Common,
        min_abbrev: 3, // :lan[guage]
    },
    BuiltinCommand {
        name: "legacy",
        description: "make following command use legacy script syntax",
        availability: Availability::Common,
        min_abbrev: 3, // :leg[acy]
    },
    BuiltinCommand {
        name: "loadkeymap",
        description: "load the following keymaps until EOF",
        availability: Availability::Common,
        min_abbrev: 5, // :loadk[eymap]
    },
    BuiltinCommand {
        name: "lockmarks",
        description: "following command keeps marks where they are",
        availability: Availability::Common,
        min_abbrev: 3, // :loc[kmarks]
    },
    BuiltinCommand {
        name: "lsp",
        description: "LSP client command",
        availability: Availability::Common,
        min_abbrev: 3, // :lsp (no abbreviation in Vim docs)
    },
    BuiltinCommand {
        name: "match",
        description: "define a match to highlight",
        availability: Availability::Common,
        min_abbrev: 3, // :mat[ch]
    },
    BuiltinCommand {
        name: "mode",
        description: "show or change the screen mode",
        availability: Availability::Common,
        min_abbrev: 3, // :mod[e]
    },
    BuiltinCommand {
        name: "noautocmd",
        description: "following commands don't trigger autocommands",
        availability: Availability::Common,
        min_abbrev: 3, // :noa[utocmd]
    },
    BuiltinCommand {
        name: "noswapfile",
        description: "following commands don't create a swap file",
        availability: Availability::Common,
        min_abbrev: 3, // :nos[wapfile]
    },
    BuiltinCommand {
        name: "open",
        description: "start open mode (not implemented)",
        availability: Availability::Common,
        min_abbrev: 1, // :o[pen]
    },
    BuiltinCommand {
        name: "options",
        description: "open the options-window",
        availability: Availability::Common,
        min_abbrev: 3, // :opt[ions]
    },
    BuiltinCommand {
        name: "ownsyntax",
        description: "set new local syntax highlight for this window",
        availability: Availability::Common,
        min_abbrev: 2, // :ow[nsyntax]
    },
    BuiltinCommand {
        name: "packadd",
        description: "add a plugin from 'packpath'",
        availability: Availability::Common,
        min_abbrev: 2, // :pa[ckadd]
    },
    BuiltinCommand {
        name: "packloadall",
        description: "load all packages under 'packpath'",
        availability: Availability::Common,
        min_abbrev: 5, // :packl[oadall]
    },
    BuiltinCommand {
        name: "range",
        description: "go to last line in {range}",
        availability: Availability::Common,
        min_abbrev: 5, // :range (special command, no abbreviation)
    },
    BuiltinCommand {
        name: "redir",
        description: "redirect messages to a file or register",
        availability: Availability::Common,
        min_abbrev: 4, // :redi[r]
    },
    BuiltinCommand {
        name: "redrawstatus",
        description: "force a redraw of the status line(s)",
        availability: Availability::Common,
        min_abbrev: 7, // :redraws[tatus]
    },
    BuiltinCommand {
        name: "redrawtabline",
        description: "force a redraw of the tabline",
        availability: Availability::Common,
        min_abbrev: 7, // :redrawt[abline]
    },
    BuiltinCommand {
        name: "redrawtabpanel",
        description: "force a redraw of the tabpanel",
        availability: Availability::Common,
        min_abbrev: 10, // :redrawtabp[anel]
    },
    BuiltinCommand {
        name: "restart",
        description: "restart Vim",
        availability: Availability::Common,
        min_abbrev: 7, // :restart (no abbreviation in Vim docs)
    },
    BuiltinCommand {
        name: "sandbox",
        description: "execute a command in the sandbox",
        availability: Availability::Common,
        min_abbrev: 3, // :san[dbox]
    },
    BuiltinCommand {
        name: "setfiletype",
        description: "set 'filetype', unless it was set already",
        availability: Availability::Common,
        min_abbrev: 4, // :setf[iletype]
    },
    BuiltinCommand {
        name: "shell",
        description: "escape to a shell",
        availability: Availability::Common,
        min_abbrev: 2, // :sh[ell]
    },
    BuiltinCommand {
        name: "sign",
        description: "manipulate signs",
        availability: Availability::Common,
        min_abbrev: 3, // :sig[n]
    },
    BuiltinCommand {
        name: "sleep!",
        description: "do nothing for a few seconds, without the cursor visible",
        availability: Availability::Common,
        min_abbrev: 2, // :sl[eep]!
    },
    BuiltinCommand {
        name: "smile",
        description: "make the user happy",
        availability: Availability::Common,
        min_abbrev: 3, // :smi[le]
    },
    BuiltinCommand {
        name: "star",
        description: "use the last Visual area, like :'<,'>",
        availability: Availability::Common,
        min_abbrev: 4, // :* (special command)
    },
    BuiltinCommand {
        name: "startgreplace",
        description: "start Virtual Replace mode",
        availability: Availability::Common,
        min_abbrev: 6, // :startg[replace]
    },
    BuiltinCommand {
        name: "startinsert",
        description: "start Insert mode",
        availability: Availability::Common,
        min_abbrev: 4, // :star[tinsert]
    },
    BuiltinCommand {
        name: "startreplace",
        description: "start Replace mode",
        availability: Availability::Common,
        min_abbrev: 6, // :startr[eplace]
    },
    BuiltinCommand {
        name: "stop",
        description: "suspend the editor or escape to a shell",
        availability: Availability::Common,
        min_abbrev: 2, // :st[op]
    },
    BuiltinCommand {
        name: "stopinsert",
        description: "stop Insert mode",
        availability: Availability::Common,
        min_abbrev: 5, // :stopi[nsert]
    },
    BuiltinCommand {
        name: "suspend",
        description: "same as \":stop\"",
        availability: Availability::Common,
        min_abbrev: 3, // :sus[pend]
    },
    BuiltinCommand {
        name: "syncbind",
        description: "sync scroll binding",
        availability: Availability::Common,
        min_abbrev: 4, // :sync[bind]
    },
    BuiltinCommand {
        name: "syntime",
        description: "measure syntax highlighting speed",
        availability: Availability::Common,
        min_abbrev: 5, // :synti[me]
    },
    BuiltinCommand {
        name: "unsilent",
        description: "run a command not silently",
        availability: Availability::Common,
        min_abbrev: 3, // :uns[ilent]
    },
    BuiltinCommand {
        name: "verbose",
        description: "execute command with 'verbose' set",
        availability: Availability::Common,
        min_abbrev: 4, // :verb[ose]
    },
    BuiltinCommand {
        name: "echoconsole",
        description: "like :echomsg but write to stdout",
        availability: Availability::Common,
        min_abbrev: 5, // :echoc[onsole]
    },
    BuiltinCommand {
        name: "fclose",
        description: "close file dialog",
        availability: Availability::Common,
        min_abbrev: 6, // :fclose (no abbreviation in Vim docs)
    },
    BuiltinCommand {
        name: "gui",
        description: "start the GUI",
        availability: Availability::Common,
        min_abbrev: 2, // :gu[i]
    },
    BuiltinCommand {
        name: "gvim",
        description: "start the GUI",
        availability: Availability::Common,
        min_abbrev: 2, // :gv[im]
    },
    BuiltinCommand {
        name: "promptfind",
        description: "open GUI dialog for searching",
        availability: Availability::Common,
        min_abbrev: 3, // :pro[mptfind]
    },
    BuiltinCommand {
        name: "promptrepl",
        description: "open GUI dialog for search/replace",
        min_abbrev: 7, // :promptr[epl]
        availability: Availability::Common,
    },
    BuiltinCommand {
        name: "simalt",
        description: "Win32 GUI: simulate Windows ALT key",
        availability: Availability::Common,
        min_abbrev: 3, // :sim[alt]
    },
    BuiltinCommand {
        name: "wlrestore",
        description: "restore the Wayland compositor connection",
        availability: Availability::Common,
        min_abbrev: 2, // :wl[restore]
    },
    BuiltinCommand {
        name: "xrestore",
        description: "restores the X server connection",
        availability: Availability::Common,
        min_abbrev: 2, // :xr[estore]
    },
    BuiltinCommand {
        name: "perl",
        description: "execute Perl command",
        availability: Availability::Common,
        min_abbrev: 2, // :pe[rl]
    },
    BuiltinCommand {
        name: "perldo",
        description: "execute Perl command for each line",
        availability: Availability::Common,
        min_abbrev: 5, // :perld[o]
    },
    BuiltinCommand {
        name: "perlfile",
        description: "execute Perl script file",
        availability: Availability::Common,
        min_abbrev: 8, // :perlfile (no abbreviation in Vim docs)
    },
    BuiltinCommand {
        name: "py3",
        description: "execute Python 3 command",
        availability: Availability::Common,
        min_abbrev: 3, // :py3
    },
    BuiltinCommand {
        name: "py3do",
        description: "execute Python 3 command for each line",
        availability: Availability::Common,
        min_abbrev: 4, // :py3d[o]
    },
    BuiltinCommand {
        name: "py3file",
        description: "execute Python 3 script file",
        availability: Availability::Common,
        min_abbrev: 4, // :py3f[ile]
    },
    BuiltinCommand {
        name: "pydo",
        description: "execute Python command for each line",
        availability: Availability::Common,
        min_abbrev: 3, // :pyd[o]
    },
    BuiltinCommand {
        name: "pyfile",
        description: "execute Python script file",
        availability: Availability::Common,
        min_abbrev: 3, // :pyf[ile]
    },
    BuiltinCommand {
        name: "python",
        description: "execute Python command",
        availability: Availability::Common,
        min_abbrev: 2, // :py[thon]
    },
    BuiltinCommand {
        name: "python3",
        description: "same as :py3",
        availability: Availability::Common,
        min_abbrev: 7, // :python3
    },
    BuiltinCommand {
        name: "pythonx",
        description: "same as :pyx",
        availability: Availability::Common,
        min_abbrev: 7, // :pythonx
    },
    BuiltinCommand {
        name: "pyx",
        description: "execute python_x command",
        availability: Availability::Common,
        min_abbrev: 3, // :pyx
    },
    BuiltinCommand {
        name: "pyxdo",
        description: "execute python_x command for each line",
        availability: Availability::Common,
        min_abbrev: 4, // :pyxd[o]
    },
    BuiltinCommand {
        name: "pyxfile",
        description: "execute python_x script file",
        availability: Availability::Common,
        min_abbrev: 4, // :pyxf[ile]
    },
    BuiltinCommand {
        name: "ruby",
        description: "execute Ruby command",
        availability: Availability::Common,
        min_abbrev: 3, // :rub[y]
    },
    BuiltinCommand {
        name: "rubydo",
        description: "execute Ruby command for each line",
        availability: Availability::Common,
        min_abbrev: 5, // :rubyd[o]
    },
    BuiltinCommand {
        name: "rubyfile",
        description: "execute Ruby script file",
        availability: Availability::Common,
        min_abbrev: 5, // :rubyf[ile]
    },
    BuiltinCommand {
        name: "tcl",
        description: "execute Tcl command",
        availability: Availability::Common,
        min_abbrev: 3, // :tcl
    },
    BuiltinCommand {
        name: "tcldo",
        description: "execute Tcl command for each line",
        availability: Availability::Common,
        min_abbrev: 4, // :tcld[o]
    },
    BuiltinCommand {
        name: "tclfile",
        description: "execute Tcl script file",
        availability: Availability::Common,
        min_abbrev: 4, // :tclf[ile]
    },
    BuiltinCommand {
        name: "mzfile",
        description: "execute MzScheme script file",
        availability: Availability::Common,
        min_abbrev: 3, // :mzf[ile]
    },
    BuiltinCommand {
        name: "mzscheme",
        description: "execute MzScheme command",
        availability: Availability::Common,
        min_abbrev: 2, // :mz[scheme]
    },
    BuiltinCommand {
        name: "nbclose",
        description: "close the current Netbeans session",
        availability: Availability::Common,
        min_abbrev: 3, // :nbc[lose]
    },
    BuiltinCommand {
        name: "nbkey",
        description: "pass a key to Netbeans",
        availability: Availability::Common,
        min_abbrev: 2, // :nb[key]
    },
    BuiltinCommand {
        name: "nbstart",
        description: "start a new Netbeans session",
        availability: Availability::Common,
        min_abbrev: 3, // :nbs[tart]
    },
    BuiltinCommand {
        name: "checkhealth",
        description: "run health checks",
        availability: Availability::NeovimOnly,
        min_abbrev: 11, // :checkhealth (Neovim only)
    },
    BuiltinCommand {
        name: "terminal",
        description: "open a terminal window",
        availability: Availability::NeovimOnly,
        min_abbrev: 3, // :ter[minal]
    },
    BuiltinCommand {
        name: "rshada",
        description: "read from shada file",
        availability: Availability::NeovimOnly,
        min_abbrev: 6, // :rshada (Neovim only)
    },
    BuiltinCommand {
        name: "wshada",
        description: "write to shada file",
        availability: Availability::NeovimOnly,
        min_abbrev: 6, // :wshada (Neovim only)
    },
    BuiltinCommand {
        name: "detach",
        description: "detach the current UI",
        availability: Availability::NeovimOnly,
        min_abbrev: 6, // :detach (Neovim only)
    },
    BuiltinCommand {
        name: "trust",
        description: "manage trusted files",
        availability: Availability::NeovimOnly,
        min_abbrev: 5, // :trust (Neovim only)
    },
    BuiltinCommand {
        name: "abstract",
        description: "declare a Vim9 abstract class",
        availability: Availability::VimOnly,
        min_abbrev: 8, // :abstract
    },
    BuiltinCommand {
        name: "class",
        description: "start of a class declaration",
        availability: Availability::VimOnly,
        min_abbrev: 5, // :class
    },
    BuiltinCommand {
        name: "def",
        description: "define a Vim9 user function",
        availability: Availability::VimOnly,
        min_abbrev: 3, // :def
    },
    BuiltinCommand {
        name: "defcompile",
        description: "compile Vim9 user functions in current script",
        availability: Availability::VimOnly,
        min_abbrev: 4, // :defc[ompile]
    },
    BuiltinCommand {
        name: "disassemble",
        description: "disassemble Vim9 user function",
        availability: Availability::VimOnly,
        min_abbrev: 4, // :disa[ssemble]
    },
    BuiltinCommand {
        name: "endclass",
        description: "end of a class declaration",
        availability: Availability::VimOnly,
        min_abbrev: 8, // :endclass
    },
    BuiltinCommand {
        name: "enddef",
        description: "end of a user function started with :def",
        availability: Availability::VimOnly,
        min_abbrev: 6, // :enddef
    },
    BuiltinCommand {
        name: "endenum",
        description: "end of an enum declaration",
        availability: Availability::VimOnly,
        min_abbrev: 7, // :endenum
    },
    BuiltinCommand {
        name: "endinterface",
        description: "end of an interface declaration",
        availability: Availability::VimOnly,
        min_abbrev: 12, // :endinterface
    },
    BuiltinCommand {
        name: "enum",
        description: "start of an enum declaration",
        availability: Availability::VimOnly,
        min_abbrev: 4, // :enum
    },
    BuiltinCommand {
        name: "export",
        description: "Vim9: export an item from a script",
        availability: Availability::VimOnly,
        min_abbrev: 3, // :exp[ort]
    },
    BuiltinCommand {
        name: "final",
        description: "declare an immutable variable in Vim9",
        availability: Availability::VimOnly,
        min_abbrev: 5, // :final
    },
    BuiltinCommand {
        name: "import",
        description: "Vim9: import an item from another script",
        availability: Availability::VimOnly,
        min_abbrev: 3, // :imp[ort]
    },
    BuiltinCommand {
        name: "interface",
        description: "start of an interface declaration",
        availability: Availability::VimOnly,
        min_abbrev: 9, // :interface
    },
    BuiltinCommand {
        name: "public",
        description: "prefix for a class or object member",
        availability: Availability::VimOnly,
        min_abbrev: 6, // :public
    },
    BuiltinCommand {
        name: "static",
        description: "prefix for a class member or function",
        availability: Availability::VimOnly,
        min_abbrev: 6, // :static
    },
    BuiltinCommand {
        name: "this",
        description: "prefix for an object member during declaration",
        availability: Availability::VimOnly,
        min_abbrev: 4, // :this
    },
    BuiltinCommand {
        name: "type",
        description: "create a type alias",
        availability: Availability::VimOnly,
        min_abbrev: 4, // :type
    },
    BuiltinCommand {
        name: "var",
        description: "variable declaration in Vim9",
        availability: Availability::VimOnly,
        min_abbrev: 3, // :var
    },
    BuiltinCommand {
        name: "vim9cmd",
        description: "make following command use Vim9 script syntax",
        availability: Availability::VimOnly,
        min_abbrev: 4, // :vim9[cmd]
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

/// List of Vim/Neovim options
/// Reference: :help option-list
pub static BUILTIN_OPTIONS: &[BuiltinOption] = &[
    // ============================================================================
    // ============================================================================
    BuiltinOption {
        name: "allowrevins",
        short: Some("ari"),
        description: "Allow CTRL-_ in Insert mode for right-to-left",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "ambiwidth",
        short: Some("ambw"),
        description: "Width of ambiguous width characters",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "arabic",
        short: Some("arab"),
        description: "Enable Arabic language support",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "arabicshape",
        short: Some("arshape"),
        description: "Perform shaping of Arabic characters",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "autochdir",
        short: Some("acd"),
        description: "Auto change directory to file location",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "autocomplete",
        short: Some("ac"),
        description: "Enable automatic completion",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "autocompletedelay",
        short: Some("acl"),
        description: "Delay before auto completion starts",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "autocompletetimeout",
        short: Some("act"),
        description: "Timeout for auto completion",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "autoindent",
        short: Some("ai"),
        description: "Copy indent from current line when starting new line",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "autoread",
        short: Some("ar"),
        description: "Auto-read file when changed outside",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "autowrite",
        short: Some("aw"),
        description: "Auto-write file before certain commands",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "autowriteall",
        short: Some("awa"),
        description: "Like autowrite but for more commands",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "background",
        short: Some("bg"),
        description: "Background color brightness (dark/light)",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "backspace",
        short: Some("bs"),
        description: "How backspace works in Insert mode",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "backup",
        short: Some("bk"),
        description: "Keep backup file after overwriting",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "backupcopy",
        short: Some("bkc"),
        description: "How to create backup (copy/rename)",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "backupdir",
        short: Some("bdir"),
        description: "Directory for backup files",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "backupext",
        short: Some("bex"),
        description: "Extension for backup files",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "backupskip",
        short: Some("bsk"),
        description: "Patterns for files to skip backup",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "belloff",
        short: Some("bo"),
        description: "Events to not ring bell for",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "binary",
        short: Some("bin"),
        description: "Binary file editing mode",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "bomb",
        short: None,
        description: "Prepend BOM to file",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "breakat",
        short: Some("brk"),
        description: "Characters for line breaking",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "breakindent",
        short: Some("bri"),
        description: "Preserve indent on wrapped lines",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "breakindentopt",
        short: Some("briopt"),
        description: "Options for breakindent",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "bufhidden",
        short: Some("bh"),
        description: "What to do when buffer is no longer displayed",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "buflisted",
        short: Some("bl"),
        description: "Whether buffer shows in buffer list",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "buftype",
        short: Some("bt"),
        description: "Special type of buffer",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "casemap",
        short: Some("cmp"),
        description: "Case changing behavior",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "cdhome",
        short: Some("cdh"),
        description: ":cd without argument goes home",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "cdpath",
        short: Some("cd"),
        description: "Search path for :cd command",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "cedit",
        short: None,
        description: "Key to open command-line window",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "charconvert",
        short: Some("ccv"),
        description: "Expression for character encoding conversion",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "chistory",
        short: Some("chi"),
        description: "Number of command-lines to remember",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "cindent",
        short: Some("cin"),
        description: "Enable C-style indenting",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "cinkeys",
        short: Some("cink"),
        description: "Keys that trigger C-indent",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "cinoptions",
        short: Some("cino"),
        description: "Options for C-indenting",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "cinscopedecls",
        short: Some("cinsd"),
        description: "Scope declaration names for cindent",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "cinwords",
        short: Some("cinw"),
        description: "Words that start extra indent",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "clipboard",
        short: Some("cb"),
        description: "Use system clipboard",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "cmdheight",
        short: Some("ch"),
        description: "Height of command-line",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "cmdwinheight",
        short: Some("cwh"),
        description: "Height of command-line window",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "colorcolumn",
        short: Some("cc"),
        description: "Columns to highlight",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "columns",
        short: Some("co"),
        description: "Number of columns in display",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "comments",
        short: Some("com"),
        description: "Patterns for comment leaders",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "commentstring",
        short: Some("cms"),
        description: "Template for comments",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "complete",
        short: Some("cpt"),
        description: "Sources for keyword completion",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "completefunc",
        short: Some("cfu"),
        description: "Function for Insert mode completion",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "completeitemalign",
        short: Some("cia"),
        description: "Alignment of completion items",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "completeopt",
        short: Some("cot"),
        description: "Options for completion",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "completeslash",
        short: Some("csl"),
        description: "Slash style for completion",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "completetimeout",
        short: Some("cto"),
        description: "Timeout for completion",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "concealcursor",
        short: Some("cocu"),
        description: "Modes where text is concealed",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "conceallevel",
        short: Some("cole"),
        description: "How to show concealed text",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "confirm",
        short: Some("cf"),
        description: "Confirm dialog for unsaved changes",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "copyindent",
        short: Some("ci"),
        description: "Copy structure of existing indent",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "cpoptions",
        short: Some("cpo"),
        description: "Vi-compatible behavior flags",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "cursorbind",
        short: Some("crb"),
        description: "Bind cursor movement between windows",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "cursorcolumn",
        short: Some("cuc"),
        description: "Highlight cursor column",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "cursorline",
        short: Some("cul"),
        description: "Highlight cursor line",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "cursorlineopt",
        short: Some("culopt"),
        description: "Options for cursorline",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "debug",
        short: None,
        description: "Debug mode settings",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "define",
        short: Some("def"),
        description: "Pattern for macro definition",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "delcombine",
        short: Some("deco"),
        description: "Delete combining characters separately",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "dictionary",
        short: Some("dict"),
        description: "Files for keyword completion",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "diff",
        short: None,
        description: "Diff mode for window",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "diffanchors",
        short: Some("dia"),
        description: "Anchors for diff alignment",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "diffexpr",
        short: Some("dex"),
        description: "Expression for diff output",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "diffopt",
        short: Some("dip"),
        description: "Options for diff mode",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "digraph",
        short: Some("dg"),
        description: "Enable digraph entry",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "directory",
        short: Some("dir"),
        description: "Directory for swap files",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "display",
        short: Some("dy"),
        description: "How to display certain characters",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "eadirection",
        short: Some("ead"),
        description: "Direction for equalalways",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "emoji",
        short: Some("emo"),
        description: "Emoji characters are full width",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "encoding",
        short: Some("enc"),
        description: "Internal character encoding",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "endoffile",
        short: Some("eof"),
        description: "Write CTRL-Z at end of file",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "endofline",
        short: Some("eol"),
        description: "Write newline at end of file",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "equalalways",
        short: Some("ea"),
        description: "Make windows equal size after split",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "equalprg",
        short: Some("ep"),
        description: "External program for = command",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "errorbells",
        short: Some("eb"),
        description: "Ring bell on errors",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "errorfile",
        short: Some("ef"),
        description: "File for error messages",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "errorformat",
        short: Some("efm"),
        description: "Format for error messages",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "eventignore",
        short: Some("ei"),
        description: "Autocommand events to ignore",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "eventignorewin",
        short: Some("eiw"),
        description: "Window-local events to ignore",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "expandtab",
        short: Some("et"),
        description: "Use spaces instead of tabs",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "exrc",
        short: Some("ex"),
        description: "Read .vimrc/.nvimrc in current directory",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "fileencoding",
        short: Some("fenc"),
        description: "File encoding for current buffer",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "fileencodings",
        short: Some("fencs"),
        description: "Encoding detection order",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "fileformat",
        short: Some("ff"),
        description: "File format (unix/dos/mac)",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "fileformats",
        short: Some("ffs"),
        description: "File format detection order",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "fileignorecase",
        short: Some("fic"),
        description: "Ignore case in file names",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "filetype",
        short: Some("ft"),
        description: "File type for current buffer",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "fillchars",
        short: Some("fcs"),
        description: "Characters for window separators",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "findfunc",
        short: Some("ffu"),
        description: "Function for :find command",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "fixendofline",
        short: Some("fixeol"),
        description: "Fix missing EOL at end of file",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "foldclose",
        short: Some("fcl"),
        description: "When to close folds",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "foldcolumn",
        short: Some("fdc"),
        description: "Width of fold column",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "foldenable",
        short: Some("fen"),
        description: "Enable folding",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "foldexpr",
        short: Some("fde"),
        description: "Expression for fold level",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "foldignore",
        short: Some("fdi"),
        description: "Character for fold detection",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "foldlevel",
        short: Some("fdl"),
        description: "Initial fold level",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "foldlevelstart",
        short: Some("fdls"),
        description: "Fold level when starting to edit",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "foldmarker",
        short: Some("fmr"),
        description: "Markers for fold method marker",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "foldmethod",
        short: Some("fdm"),
        description: "Folding type (manual/indent/expr/marker/syntax/diff)",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "foldminlines",
        short: Some("fml"),
        description: "Minimum lines for fold",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "foldnestmax",
        short: Some("fdn"),
        description: "Maximum fold nesting level",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "foldopen",
        short: Some("fdo"),
        description: "Commands that open folds",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "foldtext",
        short: Some("fdt"),
        description: "Expression for fold text",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "formatexpr",
        short: Some("fex"),
        description: "Expression for formatting",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "formatlistpat",
        short: Some("flp"),
        description: "Pattern for list item",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "formatoptions",
        short: Some("fo"),
        description: "Auto-formatting options",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "formatprg",
        short: Some("fp"),
        description: "External program for formatting",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "fsync",
        short: Some("fs"),
        description: "Fsync after writing file",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "grepformat",
        short: Some("gfm"),
        description: "Format for :grep output",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "grepprg",
        short: Some("gp"),
        description: "Program for :grep command",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "guicursor",
        short: Some("gcr"),
        description: "Cursor shape and blinking",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "guifont",
        short: Some("gfn"),
        description: "Font for GUI",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "guifontwide",
        short: Some("gfw"),
        description: "Font for double-width characters",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "helpfile",
        short: Some("hf"),
        description: "Main help file name",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "helpheight",
        short: Some("hh"),
        description: "Minimum height of help window",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "helplang",
        short: Some("hlg"),
        description: "Preferred help languages",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "hidden",
        short: Some("hid"),
        description: "Allow hidden buffers",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "history",
        short: Some("hi"),
        description: "Number of command-lines to remember",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "hlsearch",
        short: Some("hls"),
        description: "Highlight search matches",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "icon",
        short: None,
        description: "Set icon text of window",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "iconstring",
        short: None,
        description: "String for window icon text",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "ignorecase",
        short: Some("ic"),
        description: "Ignore case in search patterns",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "iminsert",
        short: Some("imi"),
        description: "Input method state for Insert mode",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "imsearch",
        short: Some("ims"),
        description: "Input method state for search",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "include",
        short: Some("inc"),
        description: "Pattern for include command",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "includeexpr",
        short: Some("inex"),
        description: "Expression for include file name",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "incsearch",
        short: Some("is"),
        description: "Incremental search",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "indentexpr",
        short: Some("inde"),
        description: "Expression for indent",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "indentkeys",
        short: Some("indk"),
        description: "Keys that trigger indenting",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "infercase",
        short: Some("inf"),
        description: "Adjust case of completion match",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "isfname",
        short: Some("isf"),
        description: "Characters in file names",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "isident",
        short: Some("isi"),
        description: "Characters in identifiers",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "iskeyword",
        short: Some("isk"),
        description: "Characters in keywords",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "isprint",
        short: Some("isp"),
        description: "Printable characters",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "joinspaces",
        short: Some("js"),
        description: "Two spaces after period on join",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "jumpoptions",
        short: Some("jop"),
        description: "Options for jump commands",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "keymap",
        short: Some("kmp"),
        description: "Keyboard mapping name",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "keymodel",
        short: Some("km"),
        description: "Enable special keys behavior",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "keywordprg",
        short: Some("kp"),
        description: "Program for K command",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "langmap",
        short: Some("lmap"),
        description: "Map keyboard for langmap mode",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "langmenu",
        short: Some("lm"),
        description: "Language for menus",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "langremap",
        short: Some("lrm"),
        description: "Langmap applies to mapped chars",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "laststatus",
        short: Some("ls"),
        description: "When to show status line",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "lazyredraw",
        short: Some("lz"),
        description: "Do not redraw during macros",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "lhistory",
        short: Some("lhi"),
        description: "Number of input lines to remember",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "linebreak",
        short: Some("lbr"),
        description: "Wrap at word boundaries",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "lines",
        short: None,
        description: "Number of lines in display",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "linespace",
        short: Some("lsp"),
        description: "Pixels between lines",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "lisp",
        short: None,
        description: "Lisp mode for indenting",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "lispoptions",
        short: Some("lop"),
        description: "Options for Lisp indenting",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "lispwords",
        short: Some("lw"),
        description: "Words for Lisp indent",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "list",
        short: None,
        description: "Show tabs and trailing spaces",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "listchars",
        short: Some("lcs"),
        description: "Characters to use for list mode",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "loadplugins",
        short: Some("lpl"),
        description: "Load plugin scripts on startup",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "magic",
        short: None,
        description: "Special chars in search patterns",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "makeef",
        short: Some("mef"),
        description: "Name of error file for :make",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "makeencoding",
        short: Some("menc"),
        description: "Encoding of :make output",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "makeprg",
        short: Some("mp"),
        description: "Program for :make command",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "matchpairs",
        short: Some("mps"),
        description: "Pairs of matching characters",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "matchtime",
        short: Some("mat"),
        description: "Tenths of second to show match",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "maxfuncdepth",
        short: Some("mfd"),
        description: "Maximum function call depth",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "maxmapdepth",
        short: Some("mmd"),
        description: "Maximum mapping nesting",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "maxmempattern",
        short: Some("mmp"),
        description: "Maximum memory for pattern matching",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "maxsearchcount",
        short: Some("msc"),
        description: "Maximum search count message",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "menuitems",
        short: Some("mis"),
        description: "Maximum items in a menu",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "messagesopt",
        short: Some("mopt"),
        description: "Options for messages",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "mkspellmem",
        short: Some("msm"),
        description: "Memory used by :mkspell",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "modeline",
        short: Some("ml"),
        description: "Enable modeline processing",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "modelineexpr",
        short: Some("mle"),
        description: "Allow expressions in modelines",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "modelines",
        short: Some("mls"),
        description: "Lines to check for modelines",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "modifiable",
        short: Some("ma"),
        description: "Buffer can be modified",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "modified",
        short: Some("mod"),
        description: "Buffer has been modified",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "more",
        short: None,
        description: "Pause listings when screen fills",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "mouse",
        short: None,
        description: "Enable mouse support",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "mousefocus",
        short: Some("mousef"),
        description: "Focus follows mouse",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "mousehide",
        short: Some("mh"),
        description: "Hide mouse while typing",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "mousemodel",
        short: Some("mousem"),
        description: "Mouse button behavior",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "mousemoveevent",
        short: Some("mousemev"),
        description: "Report mouse move events",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "mousetime",
        short: Some("mouset"),
        description: "Maximum time between clicks",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "nrformats",
        short: Some("nf"),
        description: "Number formats for CTRL-A/CTRL-X",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "number",
        short: Some("nu"),
        description: "Show line numbers",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "numberwidth",
        short: Some("nuw"),
        description: "Minimum width of number column",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "omnifunc",
        short: Some("ofu"),
        description: "Function for omni completion",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "operatorfunc",
        short: Some("opfunc"),
        description: "Function for g@ operator",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "packpath",
        short: Some("pp"),
        description: "Search path for packages",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "paragraphs",
        short: Some("para"),
        description: "Nroff macros for paragraphs",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "patchexpr",
        short: Some("pex"),
        description: "Expression for patch output",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "patchmode",
        short: Some("pm"),
        description: "Keep oldest version of file",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "path",
        short: Some("pa"),
        description: "Search path for gf and :find",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "preserveindent",
        short: Some("pi"),
        description: "Preserve indent structure",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "previewheight",
        short: Some("pvh"),
        description: "Height of preview window",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "previewwindow",
        short: Some("pvw"),
        description: "Window is preview window",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "pumborder",
        short: Some("pb"),
        description: "Enable popup menu border",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "pumheight",
        short: Some("ph"),
        description: "Maximum popup menu height",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "pummaxwidth",
        short: Some("pmw"),
        description: "Maximum popup menu width",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "pumwidth",
        short: Some("pw"),
        description: "Minimum popup menu width",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "pyxversion",
        short: Some("pyx"),
        description: "Python version for pyx commands",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "quickfixtextfunc",
        short: Some("qftf"),
        description: "Function for quickfix text",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "quoteescape",
        short: Some("qe"),
        description: "Escape character in strings",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "readonly",
        short: Some("ro"),
        description: "Buffer is read-only",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "redrawtime",
        short: Some("rdt"),
        description: "Timeout for syntax highlighting",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "regexpengine",
        short: Some("re"),
        description: "Regexp engine to use",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "relativenumber",
        short: Some("rnu"),
        description: "Show relative line numbers",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "report",
        short: None,
        description: "Minimum lines to report changes",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "revins",
        short: Some("ri"),
        description: "Insert characters backwards",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "rightleft",
        short: Some("rl"),
        description: "Window is right-to-left",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "rightleftcmd",
        short: Some("rlc"),
        description: "Commands edited right-to-left",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "ruler",
        short: Some("ru"),
        description: "Show cursor position in status line",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "rulerformat",
        short: Some("ruf"),
        description: "Format for ruler",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "runtimepath",
        short: Some("rtp"),
        description: "Search path for runtime files",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "scroll",
        short: Some("scr"),
        description: "Lines to scroll with CTRL-U/D",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "scrollbind",
        short: Some("scb"),
        description: "Bind scroll to other windows",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "scrolljump",
        short: Some("sj"),
        description: "Minimum lines to scroll",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "scrolloff",
        short: Some("so"),
        description: "Lines to keep above/below cursor",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "scrollopt",
        short: Some("sbo"),
        description: "Options for scrollbind",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "sections",
        short: Some("sect"),
        description: "Nroff macros for sections",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "selection",
        short: Some("sel"),
        description: "What type of selection to use",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "selectmode",
        short: Some("slm"),
        description: "When to start Select mode",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "sessionoptions",
        short: Some("ssop"),
        description: "Options for :mksession",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "shell",
        short: Some("sh"),
        description: "Shell to use for :! commands",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "shellcmdflag",
        short: Some("shcf"),
        description: "Flag for shell to execute command",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "shellpipe",
        short: Some("sp"),
        description: "String for :make output",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "shellquote",
        short: Some("shq"),
        description: "Quote for shell command",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "shellredir",
        short: Some("srr"),
        description: "String for output redirection",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "shellslash",
        short: Some("ssl"),
        description: "Use forward slash in file names",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "shelltemp",
        short: Some("stmp"),
        description: "Use temp files for shell commands",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "shellxescape",
        short: Some("sxe"),
        description: "Characters to escape for shellxquote",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "shellxquote",
        short: Some("sxq"),
        description: "Like shellquote for :! commands",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "shiftround",
        short: Some("sr"),
        description: "Round indent to shiftwidth multiple",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "shiftwidth",
        short: Some("sw"),
        description: "Spaces for each indent step",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "shortmess",
        short: Some("shm"),
        description: "List of flags to shorten messages",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "showbreak",
        short: Some("sbr"),
        description: "String to put at start of wrapped lines",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "showcmd",
        short: Some("sc"),
        description: "Show partial command",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "showcmdloc",
        short: Some("sloc"),
        description: "Location of showcmd",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "showfulltag",
        short: Some("sft"),
        description: "Show full tag pattern in completion",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "showmatch",
        short: Some("sm"),
        description: "Briefly jump to matching bracket",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "showmode",
        short: Some("smd"),
        description: "Show mode in command line",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "showtabline",
        short: Some("stal"),
        description: "When to show tab line",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "sidescroll",
        short: Some("ss"),
        description: "Minimum columns to scroll horizontally",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "sidescrolloff",
        short: Some("siso"),
        description: "Columns to keep left/right of cursor",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "signcolumn",
        short: Some("scl"),
        description: "When to display sign column",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "smartcase",
        short: Some("scs"),
        description: "Override ignorecase if pattern has uppercase",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "smartindent",
        short: Some("si"),
        description: "Smart autoindenting",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "smarttab",
        short: Some("sta"),
        description: "Tab key respects shiftwidth",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "smoothscroll",
        short: Some("sms"),
        description: "Scroll by screen line",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "softtabstop",
        short: Some("sts"),
        description: "Spaces for tab while editing",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "spell",
        short: None,
        description: "Enable spell checking",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "spellcapcheck",
        short: Some("spc"),
        description: "Pattern for capital letter check",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "spellfile",
        short: Some("spf"),
        description: "Files for zg and zw commands",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "spelllang",
        short: Some("spl"),
        description: "Languages for spell checking",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "spelloptions",
        short: Some("spo"),
        description: "Options for spell checking",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "spellsuggest",
        short: Some("sps"),
        description: "Methods for spell suggestions",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "splitbelow",
        short: Some("sb"),
        description: "New window goes below current",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "splitkeep",
        short: Some("spk"),
        description: "Keep topline/cursor on split",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "splitright",
        short: Some("spr"),
        description: "New window goes right of current",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "startofline",
        short: Some("sol"),
        description: "Commands move cursor to first non-blank",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "statusline",
        short: Some("stl"),
        description: "Custom format for status line",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "suffixes",
        short: Some("su"),
        description: "Suffixes to ignore in file completion",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "suffixesadd",
        short: Some("sua"),
        description: "Suffixes added when searching for file",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "swapfile",
        short: Some("swf"),
        description: "Use a swap file for buffer",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "switchbuf",
        short: Some("swb"),
        description: "Window switching behavior",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "synmaxcol",
        short: Some("smc"),
        description: "Maximum column for syntax highlighting",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "syntax",
        short: Some("syn"),
        description: "Syntax to use for highlighting",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "tabclose",
        short: None,
        description: "Which tab to focus when closing",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "tabline",
        short: Some("tal"),
        description: "Custom format for tab line",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "tabpagemax",
        short: Some("tpm"),
        description: "Maximum tabs for -p and :tab all",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "tabstop",
        short: Some("ts"),
        description: "Spaces that a tab counts for",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "tagbsearch",
        short: Some("tbs"),
        description: "Use binary search in tags files",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "tagcase",
        short: Some("tc"),
        description: "How to handle case in tag search",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "tagfunc",
        short: Some("tfu"),
        description: "Function for tag search",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "taglength",
        short: Some("tl"),
        description: "Significant characters in tag name",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "tagrelative",
        short: Some("tr"),
        description: "File names in tags file are relative",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "tags",
        short: Some("tag"),
        description: "List of tag files",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "tagstack",
        short: Some("tgst"),
        description: "Push tags onto tag stack",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "termbidi",
        short: Some("tbidi"),
        description: "Terminal handles bidirectional text",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "termguicolors",
        short: Some("tgc"),
        description: "Use GUI colors in terminal",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "textwidth",
        short: Some("tw"),
        description: "Maximum width of inserted text",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "thesaurus",
        short: Some("tsr"),
        description: "Files for thesaurus completion",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "thesaurusfunc",
        short: Some("tsrfu"),
        description: "Function for thesaurus completion",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "tildeop",
        short: Some("top"),
        description: "Tilde command behaves as operator",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "timeout",
        short: Some("to"),
        description: "Timeout for mapped sequences",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "timeoutlen",
        short: Some("tm"),
        description: "Timeout in milliseconds",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "title",
        short: None,
        description: "Set window title",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "titlelen",
        short: Some("tsl"),
        description: "Percentage of columns for title",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "titleold",
        short: None,
        description: "Old title to restore when exiting",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "titlestring",
        short: None,
        description: "String for window title",
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
        name: "undodir",
        short: Some("udir"),
        description: "Directory for undo files",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "undofile",
        short: Some("udf"),
        description: "Save undo history to file",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "undolevels",
        short: Some("ul"),
        description: "Maximum number of undo changes",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "undoreload",
        short: Some("ur"),
        description: "Maximum lines to save for undo on reload",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "updatecount",
        short: Some("uc"),
        description: "Characters typed before swap file update",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "updatetime",
        short: Some("ut"),
        description: "Milliseconds for swap file update",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "varsofttabstop",
        short: Some("vsts"),
        description: "Variable soft tab stops",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "vartabstop",
        short: Some("vts"),
        description: "Variable tab stops",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "verbose",
        short: Some("vbs"),
        description: "Verbosity level",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "verbosefile",
        short: Some("vfile"),
        description: "File to write verbose messages",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "viewdir",
        short: Some("vdir"),
        description: "Directory for view files",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "viewoptions",
        short: Some("vop"),
        description: "Options for :mkview",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "virtualedit",
        short: Some("ve"),
        description: "Allow cursor past end of line",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "visualbell",
        short: Some("vb"),
        description: "Use visual bell instead of beeping",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "warn",
        short: None,
        description: "Warn for shell command in modified buffer",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "whichwrap",
        short: Some("ww"),
        description: "Allow cursor keys to wrap lines",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "wildchar",
        short: Some("wc"),
        description: "Character for command-line completion",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "wildcharm",
        short: Some("wcm"),
        description: "Like wildchar in mappings",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "wildignore",
        short: Some("wig"),
        description: "Patterns to ignore for file completion",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "wildignorecase",
        short: Some("wic"),
        description: "Ignore case in file completion",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "wildmenu",
        short: Some("wmnu"),
        description: "Enhanced command-line completion",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "wildmode",
        short: Some("wim"),
        description: "Mode for wildchar completion",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "wildoptions",
        short: Some("wop"),
        description: "Options for command-line completion",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "winaltkeys",
        short: Some("wak"),
        description: "How Alt key works with menus",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "window",
        short: Some("wi"),
        description: "Lines in window for CTRL-F/CTRL-B",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "winfixbuf",
        short: Some("wfb"),
        description: "Window shows specific buffer",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "winfixheight",
        short: Some("wfh"),
        description: "Keep window height fixed",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "winfixwidth",
        short: Some("wfw"),
        description: "Keep window width fixed",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "winheight",
        short: Some("wh"),
        description: "Minimum height for active window",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "winminheight",
        short: Some("wmh"),
        description: "Minimum height for any window",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "winminwidth",
        short: Some("wmw"),
        description: "Minimum width for any window",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "winwidth",
        short: Some("wiw"),
        description: "Minimum width for active window",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "wrap",
        short: None,
        description: "Long lines wrap",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "wrapmargin",
        short: Some("wm"),
        description: "Characters from edge to wrap",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "wrapscan",
        short: Some("ws"),
        description: "Search wraps around end of file",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "write",
        short: None,
        description: "Writing to file allowed",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "writeany",
        short: Some("wa"),
        description: "Write to any file without asking",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "writebackup",
        short: Some("wb"),
        description: "Make backup before overwriting",
        availability: Availability::Common,
    },
    BuiltinOption {
        name: "writedelay",
        short: Some("wd"),
        description: "Delay in ms for each char written",
        availability: Availability::Common,
    },
    // ============================================================================
    // ============================================================================
    BuiltinOption {
        name: "aleph",
        short: Some("al"),
        description: "ASCII code of letter Aleph",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "altkeymap",
        short: Some("akm"),
        description: "Alternative keyboard mapping",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "antialias",
        short: Some("anti"),
        description: "Use antialiased fonts in GUI",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "autoshelldir",
        short: Some("asd"),
        description: "Auto change shell directory",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "balloondelay",
        short: Some("bdlay"),
        description: "Delay for balloon popup",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "ballooneval",
        short: Some("beval"),
        description: "Enable balloon evaluation in GUI",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "balloonevalterm",
        short: Some("bevalterm"),
        description: "Enable balloon evaluation in terminal",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "balloonexpr",
        short: Some("bexpr"),
        description: "Expression for balloon text",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "bioskey",
        short: Some("biosk"),
        description: "Use BIOS for keyboard input",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "browsedir",
        short: Some("bsdir"),
        description: "Directory for file browser",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "clipmethod",
        short: Some("cpm"),
        description: "Method to use for clipboard",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "compatible",
        short: Some("cp"),
        description: "Behave Vi-compatible",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "completefuzzycollect",
        short: Some("cfc"),
        description: "Fuzzy collect for completion",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "completepopup",
        short: Some("cpp"),
        description: "Popup window options for completion",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "conskey",
        short: Some("consk"),
        description: "Directly read console keyboard",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "cryptmethod",
        short: Some("cm"),
        description: "Encryption method for file",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "cscopepathcomp",
        short: Some("cspc"),
        description: "Path components to show in cscope",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "cscopeprg",
        short: Some("csprg"),
        description: "Program for cscope command",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "cscopequickfix",
        short: Some("csqf"),
        description: "Use quickfix window for cscope",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "cscoperelative",
        short: Some("csre"),
        description: "Use relative paths for cscope",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "cscopetag",
        short: Some("cst"),
        description: "Use cscope for tag commands",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "cscopetagorder",
        short: Some("csto"),
        description: "Order of cscope and tag search",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "cscopeverbose",
        short: Some("csverb"),
        description: "Show cscope messages",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "edcompatible",
        short: Some("ed"),
        description: "Toggle flags for :substitute",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "esckeys",
        short: Some("ek"),
        description: "Recognize function keys in Insert mode",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "fkmap",
        short: Some("fk"),
        description: "Farsi keyboard mapping",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "gdefault",
        short: Some("gd"),
        description: "Substitute replaces all in line by default",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "guifontset",
        short: Some("gfs"),
        description: "List of fonts for multi-byte text",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "guiheadroom",
        short: Some("ghr"),
        description: "Pixels for GUI window decorations",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "guiligatures",
        short: Some("gli"),
        description: "Font ligatures for GUI",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "guioptions",
        short: Some("go"),
        description: "GUI option flags",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "guipty",
        short: None,
        description: "Use pseudo-tty for :! commands",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "guitablabel",
        short: Some("gtl"),
        description: "Custom format for GUI tab label",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "guitabtooltip",
        short: Some("gtt"),
        description: "Tooltip for GUI tabs",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "highlight",
        short: Some("hl"),
        description: "Highlight groups for various occasions",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "hkmap",
        short: Some("hk"),
        description: "Hebrew keyboard mapping",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "hkmapp",
        short: Some("hkp"),
        description: "Phonetic Hebrew keyboard mapping",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "imactivatefunc",
        short: Some("imaf"),
        description: "Function to activate input method",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "imactivatekey",
        short: Some("imak"),
        description: "Key to activate input method",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "imcmdline",
        short: Some("imc"),
        description: "Use IM when entering command line",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "imdisable",
        short: Some("imd"),
        description: "Disable input method",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "imstatusfunc",
        short: Some("imsf"),
        description: "Function for IM status",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "imstyle",
        short: Some("imst"),
        description: "Input method style",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "insertmode",
        short: Some("im"),
        description: "Start in Insert mode",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "key",
        short: None,
        description: "Encryption key for current file",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "keyprotocol",
        short: Some("kpc"),
        description: "Protocol for terminal keys",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "langnoremap",
        short: Some("lnr"),
        description: "Do not langmap langmap",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "luadll",
        short: None,
        description: "Name of Lua dynamic library",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "macatsui",
        short: None,
        description: "Use ATSUI text drawing on Mac",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "maxcombine",
        short: Some("mco"),
        description: "Maximum combining characters displayed",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "maxmem",
        short: Some("mm"),
        description: "Maximum memory in KB for one buffer",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "maxmemtot",
        short: Some("mmt"),
        description: "Maximum memory in KB for all buffers",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "mouseshape",
        short: Some("mouses"),
        description: "Shape of mouse pointer",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "mzquantum",
        short: Some("mzq"),
        description: "Interval for MzScheme threads",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "mzschemedll",
        short: None,
        description: "Name of MzScheme dynamic library",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "mzschemegcdll",
        short: None,
        description: "Name of MzScheme GC dynamic library",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "opendevice",
        short: Some("odev"),
        description: "Allow opening devices",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "osctimeoutlen",
        short: Some("ost"),
        description: "Timeout for terminal responses",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "osfiletype",
        short: Some("oft"),
        description: "File type for OS/2",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "paste",
        short: None,
        description: "Paste mode enabled",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "pastetoggle",
        short: Some("pt"),
        description: "Key to toggle paste mode",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "perldll",
        short: None,
        description: "Name of Perl dynamic library",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "previewpopup",
        short: Some("pvp"),
        description: "Use popup window for preview",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "printdevice",
        short: Some("pdev"),
        description: "Printer device name",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "printencoding",
        short: Some("penc"),
        description: "Encoding for printing",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "printexpr",
        short: Some("pexpr"),
        description: "Expression for printing PostScript",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "printfont",
        short: Some("pfn"),
        description: "Font for printing",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "printheader",
        short: Some("pheader"),
        description: "Format of header for printing",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "printmbcharset",
        short: Some("pmbcs"),
        description: "Multi-byte character set for printing",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "printmbfont",
        short: Some("pmbfn"),
        description: "Font names for multi-byte printing",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "printoptions",
        short: Some("popt"),
        description: "Options for printing",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "prompt",
        short: None,
        description: "Enable prompt in Ex mode",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "pythondll",
        short: None,
        description: "Name of Python 2 dynamic library",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "pythonhome",
        short: None,
        description: "Home directory for Python 2",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "pythonthreedll",
        short: None,
        description: "Name of Python 3 dynamic library",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "pythonthreehome",
        short: None,
        description: "Home directory for Python 3",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "remap",
        short: None,
        description: "Allow nested mappings",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "renderoptions",
        short: Some("rop"),
        description: "Options for text rendering",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "restorescreen",
        short: Some("rs"),
        description: "Restore screen when exiting",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "rubydll",
        short: None,
        description: "Name of Ruby dynamic library",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "scrollfocus",
        short: Some("scf"),
        description: "Scroll window under mouse",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "secure",
        short: None,
        description: "Secure mode for untrusted files",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "shelltype",
        short: Some("st"),
        description: "Type of shell for Amiga",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "shortname",
        short: Some("sn"),
        description: "Use old 8.3 file names",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "showtabpanel",
        short: Some("stpl"),
        description: "When to show tab panel",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "swapsync",
        short: Some("sws"),
        description: "Sync swap file with fsync",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "tabpanel",
        short: Some("tpl"),
        description: "Custom format for tab panel",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "tabpanelopt",
        short: Some("tplo"),
        description: "Options for tab panel",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "tcldll",
        short: None,
        description: "Name of Tcl dynamic library",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "term",
        short: None,
        description: "Name of terminal type",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "termencoding",
        short: Some("tenc"),
        description: "Encoding of terminal output",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "termwinkey",
        short: Some("twk"),
        description: "Key for terminal window commands",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "termwinscroll",
        short: Some("twsl"),
        description: "Scrollback lines for terminal",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "termwinsize",
        short: Some("tws"),
        description: "Size of terminal window",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "termwintype",
        short: Some("twt"),
        description: "Type of terminal window",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "terse",
        short: None,
        description: "Show shorter messages",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "textauto",
        short: Some("ta"),
        description: "Auto detect file format",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "textmode",
        short: Some("tx"),
        description: "File is in text mode",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "toolbar",
        short: Some("tb"),
        description: "Items shown in toolbar",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "toolbariconsize",
        short: Some("tbis"),
        description: "Size of toolbar icons",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "ttybuiltin",
        short: None,
        description: "Use builtin termcap entries first",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "ttyfast",
        short: Some("tf"),
        description: "Fast terminal connection",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "ttymouse",
        short: Some("ttym"),
        description: "Type of mouse for terminal",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "ttyscroll",
        short: None,
        description: "Maximum lines to scroll",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "ttytype",
        short: None,
        description: "Alias for term",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "viminfo",
        short: Some("vi"),
        description: "Use viminfo file",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "viminfofile",
        short: Some("vif"),
        description: "Name of viminfo file",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "weirdinvert",
        short: Some("wiv"),
        description: "Special handling for invert",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "wincolor",
        short: Some("wcr"),
        description: "Highlight group for window",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "winptydll",
        short: None,
        description: "Name of winpty dynamic library",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "wlseat",
        short: Some("wse"),
        description: "Wayland seat name",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "wlsteal",
        short: Some("wst"),
        description: "Steal focus in Wayland",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "wltimeoutlen",
        short: Some("wtm"),
        description: "Timeout for Wayland requests",
        availability: Availability::VimOnly,
    },
    BuiltinOption {
        name: "xtermcodes",
        short: None,
        description: "Request xterm-style codes",
        availability: Availability::VimOnly,
    },
    // ============================================================================
    // ============================================================================
    BuiltinOption {
        name: "busy",
        short: None,
        description: "Terminal busy indicator",
        availability: Availability::NeovimOnly,
    },
    BuiltinOption {
        name: "channel",
        short: None,
        description: "Channel connected to buffer",
        availability: Availability::NeovimOnly,
    },
    BuiltinOption {
        name: "inccommand",
        short: Some("icm"),
        description: "Live preview of :substitute",
        availability: Availability::NeovimOnly,
    },
    BuiltinOption {
        name: "mousescroll",
        short: None,
        description: "Mouse scroll wheel behavior",
        availability: Availability::NeovimOnly,
    },
    BuiltinOption {
        name: "pumblend",
        short: None,
        description: "Popup menu pseudo-transparency",
        availability: Availability::NeovimOnly,
    },
    BuiltinOption {
        name: "redrawdebug",
        short: Some("rdb"),
        description: "Debug flags for redrawing",
        availability: Availability::NeovimOnly,
    },
    BuiltinOption {
        name: "scrollback",
        short: Some("scbk"),
        description: "Lines for terminal scrollback",
        availability: Availability::NeovimOnly,
    },
    BuiltinOption {
        name: "shada",
        short: Some("sd"),
        description: "Use shada file",
        availability: Availability::NeovimOnly,
    },
    BuiltinOption {
        name: "shadafile",
        short: Some("sdf"),
        description: "Name of shada file",
        availability: Availability::NeovimOnly,
    },
    BuiltinOption {
        name: "statuscolumn",
        short: Some("stc"),
        description: "Custom format for status column",
        availability: Availability::NeovimOnly,
    },
    BuiltinOption {
        name: "termpastefilter",
        short: Some("tpf"),
        description: "Filter for terminal paste",
        availability: Availability::NeovimOnly,
    },
    BuiltinOption {
        name: "termsync",
        short: None,
        description: "Terminal synchronized output",
        availability: Availability::NeovimOnly,
    },
    BuiltinOption {
        name: "winbar",
        short: Some("wbr"),
        description: "Custom format for window bar",
        availability: Availability::NeovimOnly,
    },
    BuiltinOption {
        name: "winblend",
        short: None,
        description: "Window pseudo-transparency",
        availability: Availability::NeovimOnly,
    },
    BuiltinOption {
        name: "winborder",
        short: None,
        description: "Default border style for windows",
        availability: Availability::NeovimOnly,
    },
    BuiltinOption {
        name: "winhighlight",
        short: Some("winhl"),
        description: "Window-local highlight groups",
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
    MapOption {
        name: "<special>",
        description: "Use special keys even with 'cpoptions' containing '<'",
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
    HasFeature {
        name: "nvim",
        description: "Running on Neovim",
        availability: Availability::NeovimOnly,
    },
    HasFeature {
        name: "wsl",
        description: "Windows Subsystem for Linux",
        availability: Availability::NeovimOnly,
    },
    HasFeature {
        name: "acl",
        description: "ACL support",
        availability: Availability::Common,
    },
    HasFeature {
        name: "bsd",
        description: "BSD system (not macOS)",
        availability: Availability::Common,
    },
    HasFeature {
        name: "clipboard",
        description: "Clipboard support",
        availability: Availability::Common,
    },
    HasFeature {
        name: "fname_case",
        description: "Case in file names matters",
        availability: Availability::Common,
    },
    HasFeature {
        name: "gui_running",
        description: "GUI is running or will start soon",
        availability: Availability::Common,
    },
    HasFeature {
        name: "hurd",
        description: "GNU/Hurd system",
        availability: Availability::Common,
    },
    HasFeature {
        name: "iconv",
        description: "Can use iconv() for conversion",
        availability: Availability::Common,
    },
    HasFeature {
        name: "linux",
        description: "Linux system",
        availability: Availability::Common,
    },
    HasFeature {
        name: "mac",
        description: "macOS system",
        availability: Availability::Common,
    },
    HasFeature {
        name: "python3",
        description: "Python 3 interface available",
        availability: Availability::Common,
    },
    HasFeature {
        name: "pythonx",
        description: "Python 2.x and/or 3.x interface available",
        availability: Availability::Common,
    },
    HasFeature {
        name: "sun",
        description: "SunOS system",
        availability: Availability::Common,
    },
    HasFeature {
        name: "ttyin",
        description: "Input is a terminal (tty)",
        availability: Availability::Common,
    },
    HasFeature {
        name: "ttyout",
        description: "Output is a terminal (tty)",
        availability: Availability::Common,
    },
    HasFeature {
        name: "unix",
        description: "Unix system",
        availability: Availability::Common,
    },
    HasFeature {
        name: "vim_starting",
        description: "True during startup",
        availability: Availability::Common,
    },
    HasFeature {
        name: "win32",
        description: "Windows system (32 or 64 bit)",
        availability: Availability::Common,
    },
    HasFeature {
        name: "win64",
        description: "Windows system (64 bit)",
        availability: Availability::Common,
    },
    HasFeature {
        name: "all_builtin_terms",
        description: "Compiled with all builtin terminals enabled",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "amiga",
        description: "Amiga version of Vim",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "arabic",
        description: "Compiled with Arabic support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "arp",
        description: "Compiled with ARP support (Amiga)",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "autocmd",
        description: "Compiled with autocommand support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "autochdir",
        description: "Compiled with support for 'autochdir'",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "autoservername",
        description: "Automatically enable clientserver",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "balloon_eval",
        description: "Compiled with balloon-eval support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "balloon_multiline",
        description: "GUI supports multiline balloons",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "beos",
        description: "BeOS version of Vim",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "browse",
        description: "Compiled with :browse support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "browsefilter",
        description: "Compiled with support for browsefilter",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "builtin_terms",
        description: "Compiled with some builtin terminals",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "byte_offset",
        description: "Compiled with support for 'o' in 'statusline'",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "channel",
        description: "Compiled with support for channel and job",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "cindent",
        description: "Compiled with 'cindent' support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "clientserver",
        description: "Compiled with remote invocation support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "clipboard_working",
        description: "Clipboard is compiled and working",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "cmdline_compl",
        description: "Compiled with cmdline-completion support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "cmdline_hist",
        description: "Compiled with cmdline-history support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "cmdline_info",
        description: "Compiled with 'showcmd' and 'ruler' support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "comments",
        description: "Compiled with 'comments' support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "compatible",
        description: "Compiled to be very Vi compatible",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "conpty",
        description: "Platform where ConPTY can be used",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "cryptv",
        description: "Compiled with encryption support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "cscope",
        description: "Compiled with cscope support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "cursorbind",
        description: "Compiled with 'cursorbind' support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "debug",
        description: "Compiled with DEBUG defined",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "dialog_con",
        description: "Compiled with console dialog support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "dialog_con_gui",
        description: "Compiled with console and GUI dialog support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "dialog_gui",
        description: "Compiled with GUI dialog support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "diff",
        description: "Compiled with vimdiff and 'diff' support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "digraphs",
        description: "Compiled with support for digraphs",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "directx",
        description: "Compiled with support for DirectX",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "dnd",
        description: "Compiled with support for ~ register",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "drop_file",
        description: "Compiled with drop_file support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "ebcdic",
        description: "Compiled on a machine with ebcdic character set",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "emacs_tags",
        description: "Compiled with support for Emacs tags",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "eval",
        description: "Compiled with expression evaluation support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "ex_extra",
        description: "Extra Ex commands (always true)",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "extra_search",
        description: "Compiled with support for 'incsearch' and 'hlsearch'",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "farsi",
        description: "Support for Farsi was removed",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "file_in_path",
        description: "Compiled with support for gf and <cfile>",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "filterpipe",
        description: "Pipes used for shell commands when 'shelltemp' is off",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "find_in_path",
        description: "Compiled with support for include file searches",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "float",
        description: "Compiled with support for Float",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "folding",
        description: "Compiled with folding support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "footer",
        description: "Compiled with GUI footer support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "fork",
        description: "Compiled to use fork()/exec() instead of system()",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "gettext",
        description: "Compiled with message translation",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "gui",
        description: "Compiled with GUI enabled",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "gui_athena",
        description: "Compiled with Athena GUI (always false)",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "gui_gnome",
        description: "Compiled with Gnome support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "gui_gtk",
        description: "Compiled with GTK+ GUI (any version)",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "gui_gtk2",
        description: "Compiled with GTK+ 2 GUI",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "gui_gtk3",
        description: "Compiled with GTK+ 3 GUI",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "gui_haiku",
        description: "Compiled with Haiku GUI",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "gui_mac",
        description: "Compiled with Macintosh GUI",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "gui_motif",
        description: "Compiled with Motif GUI",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "gui_photon",
        description: "Compiled with Photon GUI",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "gui_win32",
        description: "Compiled with MS-Windows Win32 GUI",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "gui_win32s",
        description: "Compiled with Win32s system (Windows 3.1)",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "haiku",
        description: "Haiku version of Vim",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "hangul_input",
        description: "Compiled with Hangul input support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "hpux",
        description: "HP-UX version of Vim",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "insert_expand",
        description: "Compiled with CTRL-X expansion commands in Insert mode",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "job",
        description: "Compiled with support for channel and job",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "ipv6",
        description: "Compiled with support for IPv6 networking",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "jumplist",
        description: "Compiled with jumplist support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "keymap",
        description: "Compiled with 'keymap' support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "lambda",
        description: "Compiled with lambda support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "langmap",
        description: "Compiled with 'langmap' support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "libcall",
        description: "Compiled with libcall() support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "linebreak",
        description: "Compiled with 'linebreak', 'breakat', 'showbreak' support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "lispindent",
        description: "Compiled with support for lisp indenting",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "listcmds",
        description: "Compiled with commands for buffer and argument list",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "localmap",
        description: "Compiled with local mappings and abbr",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "lua",
        description: "Compiled with Lua interface",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "macunix",
        description: "Synonym for osxdarwin",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "menu",
        description: "Compiled with support for :menu",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "mksession",
        description: "Compiled with support for :mksession",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "modify_fname",
        description: "Compiled with file name modifiers",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "mouse",
        description: "Compiled with support for mouse",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "mouse_dec",
        description: "Compiled with support for Dec terminal mouse",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "mouse_gpm",
        description: "Compiled with support for gpm (Linux console mouse)",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "mouse_gpm_enabled",
        description: "GPM mouse is working",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "mouse_netterm",
        description: "Compiled with support for netterm mouse",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "mouse_pterm",
        description: "Compiled with support for qnx pterm mouse",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "mouse_sysmouse",
        description: "Compiled with support for sysmouse (*BSD console mouse)",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "mouse_sgr",
        description: "Compiled with support for sgr mouse",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "mouse_urxvt",
        description: "Compiled with support for urxvt mouse",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "mouse_xterm",
        description: "Compiled with support for xterm mouse",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "mouseshape",
        description: "Compiled with support for 'mouseshape'",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "multi_byte",
        description: "Compiled with support for 'encoding'",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "multi_byte_encoding",
        description: "'encoding' is set to a multibyte encoding",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "multi_byte_ime",
        description: "Compiled with support for IME input method",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "multi_lang",
        description: "Compiled with support for multiple languages",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "mzscheme",
        description: "Compiled with MzScheme interface",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "nanotime",
        description: "Compiled with sub-second time stamp checks",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "netbeans_enabled",
        description: "Compiled with support for netbeans and connected",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "netbeans_intg",
        description: "Compiled with support for netbeans",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "num64",
        description: "Compiled with 64-bit Number support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "ole",
        description: "Compiled with OLE automation support for Win32",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "osx",
        description: "Compiled for macOS",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "osxdarwin",
        description: "Compiled for macOS with mac-darwin-feature",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "packages",
        description: "Compiled with packages support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "path_extra",
        description: "Compiled with up/downwards search in 'path' and 'tags'",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "perl",
        description: "Compiled with Perl interface",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "persistent_undo",
        description: "Compiled with support for persistent undo history",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "postscript",
        description: "Compiled with PostScript file printing",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "printer",
        description: "Compiled with :hardcopy support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "profile",
        description: "Compiled with :profile support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "prof_nsec",
        description: "Profile results are in nanoseconds",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "python",
        description: "Python 2.x interface available",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "python_compiled",
        description: "Compiled with Python 2.x interface",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "python_dynamic",
        description: "Python 2.x interface is dynamically loaded",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "python3_compiled",
        description: "Compiled with Python 3.x interface",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "python3_dynamic",
        description: "Python 3.x interface is dynamically loaded",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "python3_stable",
        description: "Python 3.x interface is using Python Stable ABI",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "qnx",
        description: "QNX version of Vim",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "quickfix",
        description: "Compiled with quickfix support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "reltime",
        description: "Compiled with reltime() support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "rightleft",
        description: "Compiled with 'rightleft' support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "ruby",
        description: "Compiled with Ruby interface",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "scrollbind",
        description: "Compiled with 'scrollbind' support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "showcmd",
        description: "Compiled with 'showcmd' support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "signs",
        description: "Compiled with :sign support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "smartindent",
        description: "Compiled with 'smartindent' support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "socketserver",
        description: "Compiled with socket server functionality (Unix only)",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "sodium",
        description: "Compiled with libsodium for better crypt support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "sound",
        description: "Compiled with sound support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "spell",
        description: "Compiled with spell checking support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "startuptime",
        description: "Compiled with --startuptime support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "statusline",
        description: "Compiled with support for 'statusline' and 'rulerformat'",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "sun_workshop",
        description: "Support for Sun workshop has been removed",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "syntax",
        description: "Compiled with syntax highlighting support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "syntax_items",
        description: "There are active syntax highlighting items",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "system",
        description: "Compiled to use system() instead of fork()/exec()",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "tag_binary",
        description: "Compiled with binary searching in tags files",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "tag_old_static",
        description: "Support for old static tags was removed",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "tcl",
        description: "Compiled with Tcl interface",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "termguicolors",
        description: "Compiled with true color in terminal support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "terminal",
        description: "Compiled with terminal support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "terminfo",
        description: "Compiled with terminfo instead of termcap",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "termresponse",
        description: "Compiled with support for t_RV and v:termresponse",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "textobjects",
        description: "Compiled with support for text-objects",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "textprop",
        description: "Compiled with support for text-properties",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "tgetent",
        description: "Compiled with tgetent support, able to use termcap/terminfo",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "timers",
        description: "Compiled with timer_start() support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "title",
        description: "Compiled with window title support 'title'",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "toolbar",
        description: "Compiled with support for gui-toolbar",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "unnamedplus",
        description: "Compiled with support for unnamedplus in 'clipboard'",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "user_commands",
        description: "User-defined commands",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "vartabs",
        description: "Compiled with variable tabstop support 'vartabstop'",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "vcon",
        description: "Win32: Virtual console support is working",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "vertsplit",
        description: "Compiled with vertically split windows :vsplit",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "vim9script",
        description: "Compiled with Vim9 script support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "viminfo",
        description: "Compiled with viminfo support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "vimscript-1",
        description: "Compiled Vim script version 1 support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "vimscript-2",
        description: "Compiled Vim script version 2 support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "vimscript-3",
        description: "Compiled Vim script version 3 support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "vimscript-4",
        description: "Compiled Vim script version 4 support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "virtualedit",
        description: "Compiled with 'virtualedit' option",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "visual",
        description: "Compiled with Visual mode",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "visualextra",
        description: "Compiled with extra Visual mode commands",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "vms",
        description: "VMS version of Vim",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "vreplace",
        description: "Compiled with gR and gr commands",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "vtp",
        description: "Compiled for vcon support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "wayland",
        description: "Compiled with Wayland protocol support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "wayland_clipboard",
        description: "Compiled with support for Wayland clipboard",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "wayland_focus_steal",
        description: "Compiled with support for Wayland clipboard focus stealing",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "wildignore",
        description: "Compiled with 'wildignore' option",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "wildmenu",
        description: "Compiled with 'wildmenu' option",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "win16",
        description: "Old version for MS-Windows 3.1 (always false)",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "win32unix",
        description: "Win32 version of Vim, using Unix files (Cygwin)",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "win95",
        description: "Win32 version for MS-Windows 95/98/ME (always false)",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "winaltkeys",
        description: "Compiled with 'winaltkeys' option",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "windows",
        description: "Compiled with support for more than one window",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "writebackup",
        description: "Compiled with 'writebackup' default on",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "xattr",
        description: "Compiled with extended attributes support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "xfontset",
        description: "Compiled with X fontset support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "xim",
        description: "Compiled with X input method support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "xpm",
        description: "Compiled with pixmap support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "xpm_w32",
        description: "Compiled with pixmap support for Win32",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "xsmp",
        description: "Compiled with X session management support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "xsmp_interact",
        description: "Compiled with interactive X session management support",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "xterm_clipboard",
        description: "Compiled with support for xterm clipboard",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "xterm_save",
        description: "Compiled with support for saving and restoring xterm screen",
        availability: Availability::VimOnly,
    },
    HasFeature {
        name: "x11",
        description: "Compiled with X11 support",
        availability: Availability::VimOnly,
    },
];

/// List of Vim built-in variables (v: scope)
/// Reference: :help v:var (Vim), :help vvars (Neovim)
pub static BUILTIN_VARIABLES: &[BuiltinVariable] = &[
    // === Common (Vim/Neovim) ===
    BuiltinVariable {
        name: "v:argv",
        description: "Command line arguments Vim was invoked with",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:char",
        description: "Argument for evaluating 'formatexpr' and typed character in abbreviation",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:charconvert_from",
        description: "Encoding of file to be converted (valid in 'charconvert')",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:charconvert_to",
        description: "Encoding of file after conversion (valid in 'charconvert')",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:cmdarg",
        description: "Extra arguments (++p, ++enc=, ++ff=) given to file read/write command",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:cmdbang",
        description: "Set to 1 when '!' was used with file read/write command, otherwise 0",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:collate",
        description: "Current locale setting for collation order (LC_COLLATE)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:completed_item",
        description: "Dictionary containing complete-items for most recently completed word",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:count",
        description: "Count given for the last Normal mode command (0 if no count)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:count1",
        description: "Like v:count, but defaults to 1 when no count is used",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:ctype",
        description: "Current locale setting for characters (LC_CTYPE)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:dying",
        description: "Set to 1 when a deadly signal is caught, increases with each signal",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:echospace",
        description: "Number of screen cells available for :echo message in last line",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:errmsg",
        description: "Last error message that occurred (modifiable)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:errors",
        description: "List of errors found by assert functions",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:event",
        description: "Dictionary of event data for the current autocommand",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:exception",
        description: "Value of exception most recently caught and not finished",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:exiting",
        description: "Exit code, or v:null before VimLeavePre/VimLeave autocmds",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:false",
        description: "Special value for 'false' in JSON/msgpack (converts to 0 as Number)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:fcs_choice",
        description: "What should happen after FileChangedShell event (reload/edit/ask/empty)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:fcs_reason",
        description: "Reason why FileChangedShell was triggered (deleted/conflict/changed/mode/time)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:fname",
        description: "File name detected when evaluating 'includeexpr'",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:fname_diff",
        description: "Name of diff (patch) file (valid in 'patchexpr')",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:fname_in",
        description: "Name of input file (valid in 'charconvert', 'diffexpr', 'patchexpr')",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:fname_new",
        description: "Name of new version of file (valid in 'diffexpr')",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:fname_out",
        description: "Name of output file (valid in 'charconvert', 'diffexpr', 'patchexpr')",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:folddashes",
        description: "Dashes representing foldlevel of closed fold (for 'foldtext')",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:foldend",
        description: "Last line of closed fold (for 'foldtext')",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:foldlevel",
        description: "Foldlevel of closed fold (for 'foldtext')",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:foldstart",
        description: "First line of closed fold (for 'foldtext')",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:hlsearch",
        description: "Variable that indicates whether search highlighting is on",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:insertmode",
        description: "Mode for InsertEnter/InsertChange events (i/r/v)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:key",
        description: "Key of current Dictionary item (valid in map()/filter())",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:lang",
        description: "Current locale setting for messages (LC_MESSAGES)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:lc_time",
        description: "Current locale setting for time messages (LC_TIME)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:lnum",
        description: "Line number for 'foldexpr', 'formatexpr', 'indentexpr', 'statuscolumn'",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:maxcol",
        description: "Maximum line length (currently 2147483647)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:mouse_col",
        description: "Column number for mouse click from getchar()",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:mouse_lnum",
        description: "Line number for mouse click from getchar()",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:mouse_win",
        description: "Window number for mouse click from getchar()",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:mouse_winid",
        description: "Window ID for mouse click from getchar()",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:null",
        description: "Special value for 'null' in JSON/msgpack (converts to 0 as Number)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:numbermax",
        description: "Maximum value of a Number",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:numbermin",
        description: "Minimum value of a Number (negative)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:numbersize",
        description: "Number of bits in a Number (normally 64, sometimes 32)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:oldfiles",
        description: "List of file names loaded from shada/viminfo file on startup",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:operator",
        description: "Last operator given in Normal mode",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:option_command",
        description: "Command used to set option (setlocal/setglobal/set/modeline)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:option_new",
        description: "New value of option (valid in OptionSet)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:option_old",
        description: "Old value of option (valid in OptionSet)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:option_oldglobal",
        description: "Old global value of option (valid in OptionSet)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:option_oldlocal",
        description: "Old local value of option (valid in OptionSet)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:option_type",
        description: "Scope of set command: 'global' or 'local' (valid in OptionSet)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:prevcount",
        description: "Count given for last but one Normal mode command",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:profiling",
        description: "Set to 1 after using ':profile start'",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:progname",
        description: "Name by which Vim/Nvim was invoked (path removed)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:progpath",
        description: "Absolute path to current running Vim/Nvim",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:register",
        description: "Name of register in effect for current normal mode command",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:scrollstart",
        description: "String describing script/function that caused screen scroll",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:searchforward",
        description: "Search direction: 1 after forward search, 0 after backward search",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:servername",
        description: "Primary listen-address/server name of Vim/Nvim",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:shell_error",
        description: "Result of last shell command (-1 if command could not be executed)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:stacktrace",
        description: "Stack trace of exception most recently caught and not finished",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:statusmsg",
        description: "Last given status message (modifiable)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:swapchoice",
        description: "SwapExists autocommand choice for handling existing swapfile",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:swapcommand",
        description: "Normal mode command to execute after file opened from SwapExists",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:swapname",
        description: "Name of swapfile found (valid during SwapExists event)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:t_blob",
        description: "Value of Blob type for type()",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:t_bool",
        description: "Value of Boolean type for type()",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:t_dict",
        description: "Value of Dictionary type for type()",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:t_float",
        description: "Value of Float type for type()",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:t_func",
        description: "Value of Funcref type for type()",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:t_list",
        description: "Value of List type for type()",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:t_number",
        description: "Value of Number type for type()",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:t_string",
        description: "Value of String type for type()",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:termrequest",
        description: "Value of most recent OSC/DCS/APC control sequence from terminal",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:termresponse",
        description: "Escape sequence returned by terminal for t_RV termcap entry",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:testing",
        description: "Must be set before using test_garbagecollect_now()",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:this_session",
        description: "Full filename of last loaded or saved session file",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:throwpoint",
        description: "Point where exception most recently caught was thrown",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:true",
        description: "Special value for 'true' in JSON/msgpack (converts to 1 as Number)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:val",
        description: "Value of current List/Dictionary item (valid in map()/filter())",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:version",
        description: "Vim version number: major*100 + minor (e.g., 901 for Vim 9.1)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:versionlong",
        description: "Vim version including patchlevel (e.g., 9010123 for 9.1.123)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:vim_did_enter",
        description: "0 during startup, 1 just before VimEnter",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:warningmsg",
        description: "Last given warning message (modifiable)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "v:windowid",
        description: "Application-specific window handle (not Vim window-ID)",
        availability: Availability::Common,
    },
    // === Vim only ===
    BuiltinVariable {
        name: "v:beval_bufnr",
        description: "Buffer number over which mouse pointer is (for 'balloonexpr')",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:beval_col",
        description: "Column number (byte index) for balloon eval",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:beval_lnum",
        description: "Line number for balloon eval",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:beval_text",
        description: "Text under or after mouse pointer for balloon eval",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:beval_winid",
        description: "Window ID for balloon eval",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:beval_winnr",
        description: "Window number for balloon eval",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:clipmethod",
        description: "Current method of accessing clipboard (wayland/x11)",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:clipproviders",
        description: "List of available clipboard providers",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:colornames",
        description: "Dictionary that maps color names to hex color strings",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:none",
        description: "Special value for empty in JSON (evaluates to 'v:none' as String)",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:python3_version",
        description: "Version of Python 3 that Vim was built against",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:sizeofint",
        description: "Number of bytes in an int",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:sizeoflong",
        description: "Number of bytes in a long",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:sizeofpointer",
        description: "Number of bytes in a pointer",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:t_channel",
        description: "Value of Channel type for type()",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:t_class",
        description: "Value of class type for type()",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:t_enum",
        description: "Value of enum type for type()",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:t_enumvalue",
        description: "Value of enumvalue type for type()",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:t_job",
        description: "Value of Job type for type()",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:t_none",
        description: "Value of None type for type()",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:t_object",
        description: "Value of object type for type()",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:t_tuple",
        description: "Value of Tuple type for type()",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:t_typealias",
        description: "Value of typealias type for type()",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:termblinkresp",
        description: "Escape sequence returned by terminal for t_RC (cursor blink)",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:termda1",
        description: "Escape sequence returned by primary device attributes (DA1) query",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:termosc",
        description: "Escape sequence of most recent OSC response from terminal",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:termrbgresp",
        description: "Escape sequence returned by terminal for t_RB (background color)",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:termrfgresp",
        description: "Escape sequence returned by terminal for t_RF (foreground color)",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:termstyleresp",
        description: "Escape sequence returned by terminal for t_RS (cursor shape)",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:termu7resp",
        description: "Escape sequence returned by terminal for t_u7 (ambiguous width)",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:var",
        description: "Dummy variable used to access v: scope in expressions",
        availability: Availability::VimOnly,
    },
    BuiltinVariable {
        name: "v:wayland_display",
        description: "Name of Wayland display that Vim is connected to",
        availability: Availability::VimOnly,
    },
    // === Neovim only ===
    BuiltinVariable {
        name: "v:lua",
        description: "Prefix for calling Lua functions from Vimscript expressions",
        availability: Availability::NeovimOnly,
    },
    BuiltinVariable {
        name: "v:msgpack_types",
        description: "Dictionary containing msgpack types for msgpackparse()/msgpackdump()",
        availability: Availability::NeovimOnly,
    },
    BuiltinVariable {
        name: "v:relnum",
        description: "Relative line number for 'statuscolumn' expression",
        availability: Availability::NeovimOnly,
    },
    BuiltinVariable {
        name: "v:stderr",
        description: "Channel-id corresponding to stderr (always 2)",
        availability: Availability::NeovimOnly,
    },
    BuiltinVariable {
        name: "v:virtnum",
        description: "Virtual line number for 'statuscolumn' expression",
        availability: Availability::NeovimOnly,
    },
    BuiltinVariable {
        name: "v:vim_did_init",
        description: "0 during initialization, 1 after sourcing vimrc and before load-plugins",
        availability: Availability::NeovimOnly,
    },
    // === Buffer-local predefined variables (b:) ===
    BuiltinVariable {
        name: "b:changedtick",
        description: "Total number of changes to the current buffer",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "b:current_syntax",
        description: "Name of the current syntax (set by syntax files)",
        availability: Availability::Common,
    },
    // === Global predefined variables (g:) ===
    // Common (Vim and Neovim)
    BuiltinVariable {
        name: "g:colors_name",
        description: "Name of the currently active color scheme",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "g:syntax_on",
        description: "Set when syntax highlighting is enabled",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "g:actual_curbuf",
        description: "Buffer number of the actual current buffer during statusline evaluation",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "g:actual_curwin",
        description: "Window ID of the actual current window during statusline evaluation",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "g:statusline_winid",
        description: "Window ID of the window the statusline belongs to",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "g:mapleader",
        description: "Key used as <Leader> in mappings (default: backslash)",
        availability: Availability::Common,
    },
    BuiltinVariable {
        name: "g:maplocalleader",
        description: "Key used as <LocalLeader> in buffer-local mappings",
        availability: Availability::Common,
    },
    // Vim only
    BuiltinVariable {
        name: "g:actual_curtabpage",
        description: "Tab page number of the actual current tab during tabpanel evaluation",
        availability: Availability::VimOnly,
    },
    // Neovim only
    BuiltinVariable {
        name: "g:health",
        description: "Dictionary for :checkhealth configuration (e.g., style='float')",
        availability: Availability::NeovimOnly,
    },
    BuiltinVariable {
        name: "g:clipboard",
        description: "Clipboard tool configuration (name or dict with copy/paste commands)",
        availability: Availability::NeovimOnly,
    },
    BuiltinVariable {
        name: "g:termfeatures",
        description: "Dictionary of terminal feature flags (e.g., osc52)",
        availability: Availability::NeovimOnly,
    },
    BuiltinVariable {
        name: "g:editorconfig",
        description: "Enable/disable EditorConfig integration (default: true)",
        availability: Availability::NeovimOnly,
    },
    BuiltinVariable {
        name: "g:did_load_filetypes",
        description: "Set to disable the builtin filetype detection",
        availability: Availability::NeovimOnly,
    },
    BuiltinVariable {
        name: "g:python3_host_prog",
        description: "Path to Python 3 executable for the Python provider",
        availability: Availability::NeovimOnly,
    },
    BuiltinVariable {
        name: "g:loaded_python3_provider",
        description: "Set to 0 to disable Python 3 provider",
        availability: Availability::NeovimOnly,
    },
    BuiltinVariable {
        name: "g:ruby_host_prog",
        description: "Path to Ruby executable for the Ruby provider",
        availability: Availability::NeovimOnly,
    },
    BuiltinVariable {
        name: "g:loaded_ruby_provider",
        description: "Set to 0 to disable Ruby provider",
        availability: Availability::NeovimOnly,
    },
    BuiltinVariable {
        name: "g:perl_host_prog",
        description: "Path to Perl executable for the Perl provider",
        availability: Availability::NeovimOnly,
    },
    BuiltinVariable {
        name: "g:loaded_perl_provider",
        description: "Set to 0 to disable Perl provider",
        availability: Availability::NeovimOnly,
    },
    BuiltinVariable {
        name: "g:node_host_prog",
        description: "Path to Node.js executable for the Node provider",
        availability: Availability::NeovimOnly,
    },
    BuiltinVariable {
        name: "g:loaded_node_provider",
        description: "Set to 0 to disable Node.js provider",
        availability: Availability::NeovimOnly,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_availability_is_compatible() {
        assert!(Availability::Common.is_compatible(EditorMode::Both));
        assert!(Availability::VimOnly.is_compatible(EditorMode::Both));
        assert!(Availability::NeovimOnly.is_compatible(EditorMode::Both));

        assert!(Availability::Common.is_compatible(EditorMode::VimOnly));
        assert!(Availability::VimOnly.is_compatible(EditorMode::VimOnly));
        assert!(!Availability::NeovimOnly.is_compatible(EditorMode::VimOnly));

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
