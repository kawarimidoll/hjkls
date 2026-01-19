use tree_sitter::Parser;

fn main() {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_vim::language()).unwrap();

    let code = r#"
let g:global_var = 1
let s:script_var = 2

function! MyFunction(arg1, arg2)
  let l:local_var = a:arg1 + a:arg2
  return l:local_var
endfunction

function! s:PrivateFunc()
  echo "private"
endfunction
"#;

    let tree = parser.parse(code, None).unwrap();
    print_tree(&tree.root_node(), code, 0);
}

fn print_tree(node: &tree_sitter::Node, source: &str, indent: usize) {
    let prefix = "  ".repeat(indent);
    let kind = node.kind();
    let pos = format!(
        "{}:{}-{}:{}",
        node.start_position().row,
        node.start_position().column,
        node.end_position().row,
        node.end_position().column
    );

    if node.child_count() == 0 {
        let text = node.utf8_text(source.as_bytes()).unwrap_or("");
        println!("{}{} [{}] \"{}\"", prefix, kind, pos, text);
    } else {
        println!("{}{} [{}]", prefix, kind, pos);
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        print_tree(&child, source, indent + 1);
    }
}
