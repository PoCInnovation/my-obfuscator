use std::{env, io::Read};
use rand::{Rng, distributions::DistString};

use tree_sitter::{self, Parser, Tree, TreeCursor};

trait Shiftable {
    fn shift(self, shift: i32) -> Self where Self : Sized;
}

impl Shiftable for std::ops::Range<usize> {
    fn shift(self, shift: i32) -> Self {
        let sign = shift < 0;
        let shift : usize = shift.abs().try_into().unwrap();

        if sign {
            (self.start - shift)..(self.end - shift)
        } else {
            (self.start + shift)..(self.end + shift)
        }
    }
}

fn get_fn(tree: &Tree, code: &str) -> Vec<String>
{
    fn go(cursor:&mut TreeCursor, code: &str, vec: &mut Vec<String>) {
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

fn print_tree(tree: &Tree, code: &str)
{
    fn go(cursor:&mut TreeCursor, code: &str, indent: usize) {
        let node = cursor.node();
        let node_type = node.kind();
        let node_code: &str = &code[node.byte_range()];

        println!("{:indent$}{node_type}: \"{node_code}\"", "", indent=indent);

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

fn replace_fn(tree: &mut Tree, code: &str, replace: &str, replacement: &str) -> String
{
    fn go(cursor: &mut TreeCursor, code: &mut String, replace: &str, replacement: &str, mut shift: i32) -> i32 {
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

// most strings will be 1/5 / 1/4 of this size
const MAX_RANDOM_STRING_SIZE: usize = 200;

fn rand_str() -> String {
    rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(10..30))
}

fn main() {
    let code = &std::fs::read_to_string(&env::args().collect::<Vec<String>>()[1]).expect("INCORECT FILE PATH");
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_python::language()).expect("error setting language");


    let mut tree = parser.parse(code, None).expect("error parsing the code²");

    println!("original code tree");
    print_tree(&tree, code);

    let fns = get_fn(&tree, code);
    let mut new = code.clone();
    for e in &fns {
        println!("{}", e);
        new = replace_fn(&mut tree, &new, e, &rand_str());
        tree = parser.parse(&new, None).expect("something wrong happen in parse loop");
    }
    println!("\nnew code:\n\n{new}");
}
