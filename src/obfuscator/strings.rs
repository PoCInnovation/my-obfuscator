use super::Obfuscator;
use super::Result;
use super::Shiftable;
use crate::obfuscator::error::ObfuscatorError;
use std::ops::Range;
use tree_sitter::{Tree, TreeCursor};

type RangeVec = Vec<Range<usize>>;

#[derive(Debug)]
struct StringRange {
    string_start_skip: usize,
    string_end_skip: usize,
    range: std::ops::Range<usize>,
    escapes: RangeVec,
}

fn string_encode(sr: &StringRange, s: &str) -> String {
    let mut new = String::with_capacity(s.len() * 3);

    'outer: for (i, c) in s.char_indices() {
        for esc in &sr.escapes {
            if esc.contains(&(i + sr.range.start + sr.string_start_skip)) {
                new.push(c);
                continue 'outer;
            }
        }
        let new_letter = format!("\\x{:x}", c as u32);
        new.push_str(&new_letter);
    }
    new
}

fn get_strings(tree: &Tree) -> Vec<StringRange> {
    let mut v = vec![];
    fn go(cursor: &mut TreeCursor, vec: &mut Vec<StringRange>) {
        fn get_node_within_siblings(node: &tree_sitter::Node, nodetype: &str) -> RangeVec {
            let mut vec = vec![];
            let mut cursor = node.walk();
            if !cursor.goto_first_child() {
                return vec;
            }
            loop {
                let node = cursor.node();
                let node_type = node.kind();

                if node_type == nodetype {
                    vec.push(node.byte_range());
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            vec
        }

        fn get_escapes(node: &tree_sitter::Node) -> RangeVec {
            let mut vec = get_node_within_siblings(node, "interpolation");

            for node in node.children(&mut node.walk()) {
                if node.kind() == "string_content" {
                    vec.append(&mut get_node_within_siblings(&node, "escape_sequence"));
                }
            }
            vec
        }

        let node = cursor.node();
        let node_type = node.kind();

        if node_type == "string" {
            vec.push(StringRange {
                string_end_skip: get_node_within_siblings(&node, "string_end")[0].len(),
                string_start_skip: get_node_within_siblings(&node, "string_start")[0].len(),
                range: node.byte_range(),
                escapes: get_escapes(&node),
            })
        }

        if cursor.goto_first_child() {
            go(cursor, vec);
            cursor.goto_parent();
        }
        while cursor.goto_next_sibling() {
            go(cursor, vec);
        }
    }
    go(&mut tree.walk(), &mut v);
    v
}

impl Obfuscator {
    pub fn obfuscate_strings(&mut self) -> Result<()> {
        // strings
        self.code = {
            let strings = get_strings(&self.tree);
            let mut code = self.code.clone();
            let mut shift = 0;
            for e in strings {
                let str = &self.code
                    [e.range.start + e.string_start_skip..e.range.end - e.string_end_skip];

                let formated = format!("string_decode(f\"{}\")", string_encode(&e, str));

                let len = e.range.len();
                let range = e.range.shift(shift);
                code.replace_range(range, &formated);
                shift += formated.len() as i32 - len as i32;
            }
            code
        };
        self.reparse(ObfuscatorError::Strings)
    }
}
