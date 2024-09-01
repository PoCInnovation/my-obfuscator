use std::fmt::Display;

use tree_sitter::{Tree, TreeCursor};

use super::Obfuscator;

#[allow(dead_code)]
impl Display for Obfuscator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

#[allow(dead_code)]
impl Obfuscator {
    pub fn print_tree(&self) {
        print_tree(&self.tree, &self.code);
    }
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

