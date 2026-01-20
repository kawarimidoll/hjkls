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
];
