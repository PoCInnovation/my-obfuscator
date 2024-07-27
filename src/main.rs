use rand::{distributions::DistString, Rng};
use std::env;

use tree_sitter::{self, Parser, Tree, TreeCursor};

trait Shiftable {
    fn shift(self, shift: i32) -> Self
    where
        Self: Sized;
}

impl Shiftable for std::ops::Range<usize> {
    fn shift(self, shift: i32) -> Self {
        let sign = shift < 0;
        let shift: usize = shift.abs().try_into().unwrap();

        if sign {
            (self.start - shift)..(self.end - shift)
        } else {
            (self.start + shift)..(self.end + shift)
        }
    }
}

#[derive(Debug)]
struct StringRange<'code> {
    range: std::ops::Range<usize>,
    string: &'code str,
    escapes: Vec<std::ops::Range<usize>>,
}

fn get_strings<'code>(tree: &Tree, code: &'code str) -> Vec<StringRange<'code>> {
    fn go<'code>(cursor: &mut TreeCursor, code: &'code str, vec: &mut Vec<StringRange<'code>>) {
        fn get_escapes<'code>(node: &tree_sitter::Node) -> Vec<std::ops::Range<usize>> {
            let mut vec = vec![];
            let mut cursor = node.walk();
            if !cursor.goto_first_child() {
                return vec;
            }
            while cursor.goto_next_sibling() {
                let node = cursor.node();
                let node_type = node.kind();
                if node_type == "escape_sequence" {
                    vec.push(node.byte_range());
                }
                cursor.goto_next_sibling();
            }
            vec
        }

        let node = cursor.node();
        let node_type = node.kind();

        if node_type == "string" {
            vec.push(StringRange {
                range: node.byte_range(),
                string: &code[node.byte_range()],
                escapes: get_escapes(&node),
            })
        }

        if cursor.goto_first_child() {
            go(cursor, code, vec);
            cursor.goto_parent();
        }
        while cursor.goto_next_sibling() {
            go(cursor, code, vec);
        }
    }
    let mut v = vec![];
    go(&mut tree.walk(), code, &mut v);
    v
}

fn get_fn(tree: &Tree, code: &str) -> Vec<String> {
    fn go(cursor: &mut TreeCursor, code: &str, vec: &mut Vec<String>) {
        let node = cursor.node();
        let node_type = node.kind();

        if node_type == "function_definition" {
            cursor.goto_first_child();
            cursor.goto_next_sibling();
            vec.push(code[cursor.node().byte_range()].to_owned());
            cursor.goto_parent();
        }

        if cursor.goto_first_child() {
            go(cursor, code, vec);
            cursor.goto_parent();
        }
        while cursor.goto_next_sibling() {
            go(cursor, code, vec);
        }
    }
    let mut v = vec![];
    go(&mut tree.walk(), code, &mut v);
    v
}

fn print_tree(tree: &Tree, code: &str) {
    fn go(cursor: &mut TreeCursor, code: &str, indent: usize) {
        let node = cursor.node();
        let node_type = node.kind();
        let node_code: &str = &code[node.byte_range()];

        eprintln!(
            "{:indent$}{node_type}: \"{node_code}\"",
            "",
            indent = indent
        );

        if cursor.goto_first_child() {
            go(cursor, code, indent + 2);
            cursor.goto_parent();
        }
        while cursor.goto_next_sibling() {
            go(cursor, code, indent);
        }
    }
    go(&mut tree.walk(), code, 0);
}

fn replace_fn(tree: &mut Tree, code: &str, replace: &str, replacement: &str) -> String {
    fn go(
        cursor: &mut TreeCursor,
        code: &mut String,
        replace: &str,
        replacement: &str,
        mut shift: i32,
    ) -> i32 {
        let node = cursor.node();
        let node_type = node.kind();
        let range = node.byte_range().shift(shift);
        let node_code: &str = &code[range.clone()];

        if node_type == "identifier" && node_code == replace {
            if node_code == replace {
                code.replace_range(range, replacement);
                shift += replacement.len() as i32 - replace.len() as i32;
            }
            cursor.goto_parent();
        }
        if cursor.goto_first_child() {
            shift = go(cursor, code, replace, replacement, shift);
            cursor.goto_parent();
        }
        while cursor.goto_next_sibling() {
            shift = go(cursor, code, replace, replacement, shift);
        }
        shift
    }
    let mut code = code.to_string();
    go(&mut tree.walk(), &mut code, replace, replacement, 0);
    code
}

fn rand_str() -> String {
    rand::distributions::Alphanumeric.sample_string(
        &mut rand::thread_rng(),
        rand::thread_rng().gen_range(10..30),
    )
}

const STRING_OBFUSCATOR_HELPER: &str = r#"
def string_decode(string):
    string = list(string)
    if string == []:
        return str()
    for i in range(len(string)):
        if ord(string[i]) >= 97 and ord(string[i]) <= 122:
            string[i] = chr(ord(string[i]) - 1)
    return ''.join(string)
"#;

fn string_encode(string: &str) -> String {
    let mut encoded = string[1..string.len() - 1].to_string().into_bytes();

    encoded.iter_mut().skip(1).for_each(|e| {
        if *e >= 97  && *e <= 122 {
            *e = *e + 1;
        } 
    });
    encoded.iter().map(|e| *e as char).collect()
}

fn main() {
    let mut code = std::fs::read_to_string(&env::args().collect::<Vec<String>>()[1])
        .expect("INCORECT FILE PATH");

    code.insert_str(0, STRING_OBFUSCATOR_HELPER);

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_python::language())
        .expect("error setting language");

    let tree = parser.parse(&code, None).expect("error parsing the code²");

    eprintln!("original code tree");
    print_tree(&tree, &code);

    // strings
    eprintln!("strings");
    let code = {
        let strings = get_strings(&tree, &code);
        dbg!(&strings);
        let mut code = code.clone();
        let mut shift = 0;
        for e in strings {
            let formated = format!("string_decode(\"{}\")", string_encode(e.string));
            code.replace_range(e.range.shift(shift), &formated);
            shift += formated.len() as i32 - e.string.len() as i32;
        }
        code
    };

    let mut tree = parser.parse(&code, None).expect("error parsing the code²");

    let fns = get_fn(&tree, &code);
    let mut new = code.clone();
    for e in &fns {
        new = replace_fn(&mut tree, &new, e, &rand_str());
        tree = parser
            .parse(&new, None)
            .expect("something wrong happen in parse loop");
    }
    //println!("\nnew code:\n\n{new}");
    println!("{new}");
}
