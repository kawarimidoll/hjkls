use tree_sitter::Parser;

fn main() {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_vim::language()).unwrap();

    let test_cases = vec![
        ("lambda call", "let F = { x -> x + 1 }\necho F(5)"),
        (
            "dict method",
            "let dict = {}\nfunction! dict.method() abort\n  return 'ok'\nendfunction\necho dict.method()",
        ),
        (
            "callback in a:",
            "function! s:run(callback) abort\n  return a:callback()\nendfunction",
        ),
        ("member access call", "echo self.method()"),
        (
            "funcref call",
            "let G = function('strlen')\necho G('hello')",
        ),
        ("dict subscript call", "echo a:opts['callback']()"),
        (
            "local scope call",
            "function! Test() abort\n  let l:Callback = { -> 'test' }\n  return l:Callback()\nendfunction",
        ),
    ];

    for (name, code) in test_cases {
        println!("\n=== {} ===", name);
        println!("Code: {}", code.replace('\n', " | "));
        let tree = parser.parse(code, None).unwrap();
        print_call_expressions(&tree.root_node(), code, 0);
    }
}

fn print_call_expressions(node: &tree_sitter::Node, source: &str, depth: usize) {
    let indent = "  ".repeat(depth);

    if node.kind() == "call_expression" {
        println!("{}call_expression:", indent);
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                let text = child.utf8_text(source.as_bytes()).unwrap_or("");
                println!("{}  child[{}]: {} = {:?}", indent, i, child.kind(), text);
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        print_call_expressions(&child, source, depth + 1);
    }
}
