//! Vim script built-in functions database

/// Information about a built-in function
pub struct BuiltinFunction {
    pub name: &'static str,
    pub signature: &'static str,
    pub description: &'static str,
}

/// List of Vim built-in functions
/// Reference: :help function-list
pub static BUILTIN_FUNCTIONS: &[BuiltinFunction] = &[
    // String functions
    BuiltinFunction {
        name: "strlen",
        signature: "strlen({string})",
        description: "Return the number of bytes in {string}",
    },
    BuiltinFunction {
        name: "strchars",
        signature: "strchars({string} [, {skipcc}])",
        description: "Return the number of characters in {string}",
    },
    BuiltinFunction {
        name: "strwidth",
        signature: "strwidth({string})",
        description: "Return the display width of {string}",
    },
    BuiltinFunction {
        name: "strdisplaywidth",
        signature: "strdisplaywidth({string} [, {col}])",
        description: "Return the display width of {string} starting at {col}",
    },
    BuiltinFunction {
        name: "substitute",
        signature: "substitute({string}, {pat}, {sub}, {flags})",
        description: "Replace {pat} with {sub} in {string}",
    },
    BuiltinFunction {
        name: "submatch",
        signature: "submatch({nr} [, {list}])",
        description: "Return a specific match in substitute",
    },
    BuiltinFunction {
        name: "strpart",
        signature: "strpart({string}, {start} [, {len} [, {chars}]])",
        description: "Return part of a string",
    },
    BuiltinFunction {
        name: "stridx",
        signature: "stridx({haystack}, {needle} [, {start}])",
        description: "Return index of {needle} in {haystack}",
    },
    BuiltinFunction {
        name: "strridx",
        signature: "strridx({haystack}, {needle} [, {start}])",
        description: "Return last index of {needle} in {haystack}",
    },
    BuiltinFunction {
        name: "split",
        signature: "split({string} [, {pattern} [, {keepempty}]])",
        description: "Split {string} into a List",
    },
    BuiltinFunction {
        name: "join",
        signature: "join({list} [, {sep}])",
        description: "Join {list} items into a string",
    },
    BuiltinFunction {
        name: "trim",
        signature: "trim({string} [, {mask} [, {dir}]])",
        description: "Remove characters from {string}",
    },
    BuiltinFunction {
        name: "tolower",
        signature: "tolower({string})",
        description: "Convert {string} to lowercase",
    },
    BuiltinFunction {
        name: "toupper",
        signature: "toupper({string})",
        description: "Convert {string} to uppercase",
    },
    BuiltinFunction {
        name: "tr",
        signature: "tr({string}, {fromstr}, {tostr})",
        description: "Translate characters in {string}",
    },
    BuiltinFunction {
        name: "printf",
        signature: "printf({fmt}, {expr1}...)",
        description: "Format a string like sprintf()",
    },
    BuiltinFunction {
        name: "escape",
        signature: "escape({string}, {chars})",
        description: "Escape {chars} in {string} with backslash",
    },
    BuiltinFunction {
        name: "shellescape",
        signature: "shellescape({string} [, {special}])",
        description: "Escape {string} for use as shell argument",
    },
    BuiltinFunction {
        name: "fnameescape",
        signature: "fnameescape({string})",
        description: "Escape {string} for use as file name",
    },
    BuiltinFunction {
        name: "match",
        signature: "match({string}, {pattern} [, {start} [, {count}]])",
        description: "Return index of {pattern} match in {string}",
    },
    BuiltinFunction {
        name: "matchend",
        signature: "matchend({string}, {pattern} [, {start} [, {count}]])",
        description: "Return end index of {pattern} match",
    },
    BuiltinFunction {
        name: "matchstr",
        signature: "matchstr({string}, {pattern} [, {start} [, {count}]])",
        description: "Return matched string",
    },
    BuiltinFunction {
        name: "matchlist",
        signature: "matchlist({string}, {pattern} [, {start} [, {count}]])",
        description: "Return match and submatches as List",
    },
    // List functions
    BuiltinFunction {
        name: "len",
        signature: "len({expr})",
        description: "Return the length of {expr}",
    },
    BuiltinFunction {
        name: "empty",
        signature: "empty({expr})",
        description: "Return TRUE if {expr} is empty",
    },
    BuiltinFunction {
        name: "get",
        signature: "get({list}, {idx} [, {default}])",
        description: "Get item {idx} from {list}",
    },
    BuiltinFunction {
        name: "add",
        signature: "add({list}, {expr})",
        description: "Append {expr} to {list}",
    },
    BuiltinFunction {
        name: "insert",
        signature: "insert({list}, {item} [, {idx}])",
        description: "Insert {item} into {list}",
    },
    BuiltinFunction {
        name: "remove",
        signature: "remove({list}, {idx} [, {end}])",
        description: "Remove items from {list}",
    },
    BuiltinFunction {
        name: "copy",
        signature: "copy({expr})",
        description: "Make a shallow copy of {expr}",
    },
    BuiltinFunction {
        name: "deepcopy",
        signature: "deepcopy({expr} [, {noref}])",
        description: "Make a deep copy of {expr}",
    },
    BuiltinFunction {
        name: "extend",
        signature: "extend({list1}, {list2} [, {idx}])",
        description: "Append {list2} to {list1}",
    },
    BuiltinFunction {
        name: "filter",
        signature: "filter({expr}, {func})",
        description: "Filter items in {expr} using {func}",
    },
    BuiltinFunction {
        name: "map",
        signature: "map({expr}, {func})",
        description: "Transform items in {expr} using {func}",
    },
    BuiltinFunction {
        name: "sort",
        signature: "sort({list} [, {func} [, {dict}]])",
        description: "Sort {list} in-place",
    },
    BuiltinFunction {
        name: "reverse",
        signature: "reverse({list})",
        description: "Reverse {list} in-place",
    },
    BuiltinFunction {
        name: "uniq",
        signature: "uniq({list} [, {func} [, {dict}]])",
        description: "Remove duplicate adjacent items",
    },
    BuiltinFunction {
        name: "index",
        signature: "index({list}, {expr} [, {start} [, {ic}]])",
        description: "Return index of {expr} in {list}",
    },
    BuiltinFunction {
        name: "count",
        signature: "count({list}, {expr} [, {ic} [, {max}]])",
        description: "Count occurrences of {expr} in {list}",
    },
    BuiltinFunction {
        name: "range",
        signature: "range({expr} [, {max} [, {stride}]])",
        description: "Return a List of numbers",
    },
    BuiltinFunction {
        name: "repeat",
        signature: "repeat({expr}, {count})",
        description: "Repeat {expr} {count} times",
    },
    BuiltinFunction {
        name: "flatten",
        signature: "flatten({list} [, {maxdepth}])",
        description: "Flatten nested lists",
    },
    // Dictionary functions
    BuiltinFunction {
        name: "keys",
        signature: "keys({dict})",
        description: "Return List of keys in {dict}",
    },
    BuiltinFunction {
        name: "values",
        signature: "values({dict})",
        description: "Return List of values in {dict}",
    },
    BuiltinFunction {
        name: "items",
        signature: "items({dict})",
        description: "Return List of [key, value] pairs",
    },
    BuiltinFunction {
        name: "has_key",
        signature: "has_key({dict}, {key})",
        description: "Return TRUE if {dict} has {key}",
    },
    // Type checking
    BuiltinFunction {
        name: "type",
        signature: "type({expr})",
        description: "Return the type of {expr}",
    },
    BuiltinFunction {
        name: "typename",
        signature: "typename({expr})",
        description: "Return the type name of {expr}",
    },
    // Buffer/Window/Tab functions
    BuiltinFunction {
        name: "bufnr",
        signature: "bufnr([{expr} [, {create}]])",
        description: "Return buffer number",
    },
    BuiltinFunction {
        name: "bufname",
        signature: "bufname([{expr}])",
        description: "Return buffer name",
    },
    BuiltinFunction {
        name: "bufexists",
        signature: "bufexists({expr})",
        description: "Return TRUE if buffer exists",
    },
    BuiltinFunction {
        name: "buflisted",
        signature: "buflisted({expr})",
        description: "Return TRUE if buffer is listed",
    },
    BuiltinFunction {
        name: "bufloaded",
        signature: "bufloaded({expr})",
        description: "Return TRUE if buffer is loaded",
    },
    BuiltinFunction {
        name: "getbufline",
        signature: "getbufline({buf}, {lnum} [, {end}])",
        description: "Return lines from buffer",
    },
    BuiltinFunction {
        name: "setbufline",
        signature: "setbufline({buf}, {lnum}, {text})",
        description: "Set lines in buffer",
    },
    BuiltinFunction {
        name: "appendbufline",
        signature: "appendbufline({buf}, {lnum}, {text})",
        description: "Append lines to buffer",
    },
    BuiltinFunction {
        name: "deletebufline",
        signature: "deletebufline({buf}, {first} [, {last}])",
        description: "Delete lines from buffer",
    },
    BuiltinFunction {
        name: "winnr",
        signature: "winnr([{arg}])",
        description: "Return window number",
    },
    BuiltinFunction {
        name: "winbufnr",
        signature: "winbufnr({nr})",
        description: "Return buffer number of window {nr}",
    },
    BuiltinFunction {
        name: "tabpagenr",
        signature: "tabpagenr([{arg}])",
        description: "Return tab page number",
    },
    BuiltinFunction {
        name: "tabpagebuflist",
        signature: "tabpagebuflist([{arg}])",
        description: "Return List of buffer numbers in tab",
    },
    // Cursor/Position functions
    BuiltinFunction {
        name: "line",
        signature: "line({expr} [, {winid}])",
        description: "Return line number of {expr}",
    },
    BuiltinFunction {
        name: "col",
        signature: "col({expr} [, {winid}])",
        description: "Return column number of {expr}",
    },
    BuiltinFunction {
        name: "virtcol",
        signature: "virtcol({expr} [, {list} [, {winid}]])",
        description: "Return screen column of {expr}",
    },
    BuiltinFunction {
        name: "getpos",
        signature: "getpos({expr})",
        description: "Return position of {expr}",
    },
    BuiltinFunction {
        name: "setpos",
        signature: "setpos({expr}, {list})",
        description: "Set position of {expr}",
    },
    BuiltinFunction {
        name: "cursor",
        signature: "cursor({lnum}, {col} [, {off}])",
        description: "Move cursor to position",
    },
    BuiltinFunction {
        name: "getcurpos",
        signature: "getcurpos([{winnr}])",
        description: "Return cursor position",
    },
    BuiltinFunction {
        name: "getline",
        signature: "getline({lnum} [, {end}])",
        description: "Return line(s) from current buffer",
    },
    BuiltinFunction {
        name: "setline",
        signature: "setline({lnum}, {text})",
        description: "Set line {lnum} to {text}",
    },
    BuiltinFunction {
        name: "append",
        signature: "append({lnum}, {text})",
        description: "Append {text} after line {lnum}",
    },
    // Search functions
    BuiltinFunction {
        name: "search",
        signature: "search({pattern} [, {flags} [, {stopline} [, {timeout} [, {skip}]]]])",
        description: "Search for {pattern}, return line number of match",
    },
    BuiltinFunction {
        name: "searchpos",
        signature: "searchpos({pattern} [, {flags} [, {stopline} [, {timeout} [, {skip}]]]])",
        description: "Search for {pattern}, return [lnum, col] of match",
    },
    BuiltinFunction {
        name: "searchpair",
        signature: "searchpair({start}, {middle}, {end} [, {flags} [, {skip} [, {stopline} [, {timeout}]]]])",
        description: "Search for matching pair of start/end patterns",
    },
    BuiltinFunction {
        name: "searchpairpos",
        signature: "searchpairpos({start}, {middle}, {end} [, {flags} [, {skip} [, {stopline} [, {timeout}]]]])",
        description: "Search for matching pair, return [lnum, col]",
    },
    // File functions
    BuiltinFunction {
        name: "expand",
        signature: "expand({string} [, {nosuf} [, {list}]])",
        description: "Expand wildcards and special keywords",
    },
    BuiltinFunction {
        name: "glob",
        signature: "glob({expr} [, {nosuf} [, {list} [, {alllinks}]]])",
        description: "Expand file wildcards",
    },
    BuiltinFunction {
        name: "globpath",
        signature: "globpath({path}, {expr} [, {nosuf} [, {list} [, {alllinks}]]])",
        description: "Expand file wildcards in {path}",
    },
    BuiltinFunction {
        name: "filereadable",
        signature: "filereadable({file})",
        description: "Return TRUE if {file} is readable",
    },
    BuiltinFunction {
        name: "filewritable",
        signature: "filewritable({file})",
        description: "Return TRUE if {file} is writable",
    },
    BuiltinFunction {
        name: "isdirectory",
        signature: "isdirectory({directory})",
        description: "Return TRUE if {directory} is a directory",
    },
    BuiltinFunction {
        name: "fnamemodify",
        signature: "fnamemodify({fname}, {mods})",
        description: "Modify file name according to {mods}",
    },
    BuiltinFunction {
        name: "readfile",
        signature: "readfile({fname} [, {type} [, {max}]])",
        description: "Read file into a List",
    },
    BuiltinFunction {
        name: "writefile",
        signature: "writefile({list}, {fname} [, {flags}])",
        description: "Write List to file",
    },
    BuiltinFunction {
        name: "delete",
        signature: "delete({fname} [, {flags}])",
        description: "Delete file or directory",
    },
    BuiltinFunction {
        name: "rename",
        signature: "rename({from}, {to})",
        description: "Rename file",
    },
    BuiltinFunction {
        name: "mkdir",
        signature: "mkdir({name} [, {path} [, {prot}]])",
        description: "Create directory",
    },
    BuiltinFunction {
        name: "getcwd",
        signature: "getcwd([{winnr} [, {tabnr}]])",
        description: "Return current working directory",
    },
    BuiltinFunction {
        name: "chdir",
        signature: "chdir({dir})",
        description: "Change current directory",
    },
    // System functions
    BuiltinFunction {
        name: "system",
        signature: "system({cmd} [, {input}])",
        description: "Execute shell command and return output",
    },
    BuiltinFunction {
        name: "systemlist",
        signature: "systemlist({cmd} [, {input} [, {keepempty}]])",
        description: "Execute shell command and return List",
    },
    BuiltinFunction {
        name: "executable",
        signature: "executable({expr})",
        description: "Return TRUE if {expr} is executable",
    },
    BuiltinFunction {
        name: "exepath",
        signature: "exepath({expr})",
        description: "Return full path to executable",
    },
    BuiltinFunction {
        name: "environ",
        signature: "environ()",
        description: "Return Dict of environment variables",
    },
    BuiltinFunction {
        name: "getenv",
        signature: "getenv({name})",
        description: "Return environment variable value",
    },
    BuiltinFunction {
        name: "setenv",
        signature: "setenv({name}, {val})",
        description: "Set environment variable",
    },
    // Misc functions
    BuiltinFunction {
        name: "exists",
        signature: "exists({expr})",
        description: "Return TRUE if {expr} exists",
    },
    BuiltinFunction {
        name: "has",
        signature: "has({feature} [, {check}])",
        description: "Return TRUE if feature is supported",
    },
    BuiltinFunction {
        name: "eval",
        signature: "eval({string})",
        description: "Evaluate {string} as expression",
    },
    BuiltinFunction {
        name: "execute",
        signature: "execute({command} [, {silent}])",
        description: "Execute Ex command and return output",
    },
    BuiltinFunction {
        name: "input",
        signature: "input({prompt} [, {text} [, {completion}]])",
        description: "Get input from user",
    },
    BuiltinFunction {
        name: "confirm",
        signature: "confirm({msg} [, {choices} [, {default} [, {type}]]])",
        description: "Show confirmation dialog",
    },
    BuiltinFunction {
        name: "feedkeys",
        signature: "feedkeys({string} [, {mode}])",
        description: "Add keys to input buffer",
    },
    BuiltinFunction {
        name: "mode",
        signature: "mode([{expr}])",
        description: "Return current mode",
    },
    BuiltinFunction {
        name: "visualmode",
        signature: "visualmode([{expr}])",
        description: "Return last visual mode",
    },
    BuiltinFunction {
        name: "echo",
        signature: "echo {expr1} ..",
        description: "Echo expressions",
    },
    BuiltinFunction {
        name: "echomsg",
        signature: "echomsg {expr1} ..",
        description: "Echo as message",
    },
    BuiltinFunction {
        name: "echoerr",
        signature: "echoerr {expr1} ..",
        description: "Echo as error message",
    },
    // Function-related
    BuiltinFunction {
        name: "call",
        signature: "call({func}, {arglist} [, {dict}])",
        description: "Call {func} with arguments from {arglist}",
    },
    BuiltinFunction {
        name: "function",
        signature: "function({name} [, {arglist}] [, {dict}])",
        description: "Return Funcref to function {name}",
    },
    BuiltinFunction {
        name: "funcref",
        signature: "funcref({name} [, {arglist}] [, {dict}])",
        description: "Return Funcref like function()",
    },
    // JSON
    BuiltinFunction {
        name: "json_encode",
        signature: "json_encode({expr})",
        description: "Encode {expr} as JSON",
    },
    BuiltinFunction {
        name: "json_decode",
        signature: "json_decode({string})",
        description: "Decode JSON {string}",
    },
    // Timer
    BuiltinFunction {
        name: "timer_start",
        signature: "timer_start({time}, {callback} [, {options}])",
        description: "Create a timer",
    },
    BuiltinFunction {
        name: "timer_stop",
        signature: "timer_stop({timer})",
        description: "Stop a timer",
    },
    BuiltinFunction {
        name: "timer_stopall",
        signature: "timer_stopall()",
        description: "Stop all timers",
    },
];
