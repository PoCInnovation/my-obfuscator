use super::error::ObfuscatorError;
use super::error::Result;
use super::random_identifiers::rand_str;
use super::Obfuscator;
use super::Shiftable;
use std::ops::Range;
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

fn get_fn_calls(tree: &Tree) -> Vec<Range<usize>> {
    fn go(cursor: &mut TreeCursor, vec: &mut Vec<Range<usize>>) {
        let node = cursor.node();
        let node_type = node.kind();

        if node_type == "call" {
            vec.push(cursor.node().byte_range());
        }

        if cursor.goto_first_child() {
            go(cursor, vec);
            cursor.goto_parent();
        }
        while cursor.goto_next_sibling() {
            go(cursor, vec);
        }
    }
    let mut v = vec![];
    go(&mut tree.walk(), &mut v);
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

fn hide_call(obfuscator: &Obfuscator, call: Range<usize>) -> String {
    let old_call = &obfuscator.code[call];
    if old_call.starts_with("ohe") || old_call.starts_with("eval") {
        return old_call.to_owned();
    }
    let new_call = format!("ohe_call_function(r'''{old_call}''')");

    new_call
}

impl Obfuscator {
    pub fn obfuscate_functions(&mut self) -> Result<()> {
        let fns = get_fn(&self.tree, &self.code);
        for e in &fns {
            self.code = replace_fn(&mut self.tree, &self.code, e, &rand_str());
            self.reparse(ObfuscatorError::Functions(e.clone()))?;
        }
        Ok(())
    }

    pub fn obfuscate_function_calls(&mut self) -> Result<()> {
        let mut shift = 0;
        let calls = get_fn_calls(&self.tree).into_iter().skip(7);
        'outer: for (i, call) in calls.clone().enumerate() {
            for cn in calls.clone().take(i) {
                if cn.end > call.start {
                    continue 'outer;
                }
            }
            let call = call.shift(shift);
            let len = call.len();

            let hidden = hide_call(self, call.clone());

            shift += hidden.len() as i32 - len as i32;
            self.code.replace_range(call, &hidden);
            self.reparse(ObfuscatorError::Functions(
                "call replace lead to syntactical error".to_string(),
            ))?;
        }
        Ok(())
    }
}
