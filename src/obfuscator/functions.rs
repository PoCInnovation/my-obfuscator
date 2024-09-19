use super::error::ObfuscatorError;
use super::error::Result;
use super::random_identifiers::rand_str;
use super::Obfuscator;
use super::Shiftable;
use tree_sitter::{Tree, TreeCursor};

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

impl Obfuscator {
    pub fn obfuscate_functions(&mut self) -> Result<()> {
        let _ = self.reparse(ObfuscatorError::Booleans);
        let fns = get_fn(&self.tree, &self.code);
        for e in &fns {
            self.code = replace_fn(&mut self.tree, &self.code, e, &rand_str());
            self.reparse(ObfuscatorError::Functions(e.clone()))?;
        }
        Ok(())
    }
}
