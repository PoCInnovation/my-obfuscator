use super::Obfuscator;
use super::Result;
use super::Shiftable;
use crate::obfuscator::error::ObfuscatorError;
use tree_sitter::{Tree, TreeCursor};

#[derive(Debug)]
struct StringRange {
    range: std::ops::Range<usize>,
    escapes: Vec<std::ops::Range<usize>>,
}

fn string_encode(sr: &StringRange, s: &str) -> String {
    let mut new = String::with_capacity(s.len() - 2);

    for (i, c) in s.char_indices() {
        if i == 0 || i == s.len() - 1 {
            continue;
        }
        if {
            let mut ctn = false;
            for esc in &sr.escapes {
                if esc.contains(&i) {
                    ctn = true;
                    break;
                }
            }
            ctn
        }
        {
            new.push(c);
            continue;
        }
        eprint!("from {c}, {} ", c as u32);
        let new_letter = format!("\\x{:x}", c as u32);
        eprintln!("to {new_letter}");
        new.push_str(&new_letter);
    }
    new
}

fn get_strings(tree: &Tree) -> Vec<StringRange> {
    let mut v = vec![];
    fn go(cursor: &mut TreeCursor, vec: &mut Vec<StringRange>) {
        fn get_escapes(node: &tree_sitter::Node) -> Vec<std::ops::Range<usize>> {
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
        eprintln!("strings");
        self.code = {
            let strings = get_strings(&self.tree);
            dbg!(&strings);
            for e in &strings {
                eprintln!("string = {}", &self.code[e.range.clone()]);
            }
            let mut code = self.code.clone();
            let mut shift = 0;
            for e in strings {
                eprintln!("loop string");
                let str = &self.code[e.range.clone()];
                eprintln!("string = {str}");
                dbg!(&e);

                let formated = format!("string_decode(\"{}\")", string_encode(&e, str));
                eprintln!("encoded {}", formated);

                let len = e.range.len();
                let range = e.range.shift(shift);
                code.replace_range(range, &formated);
                eprintln!("old len = {}, new len = {}", len, formated.len());
                shift += formated.len() as i32 - len as i32;
            }
            code
        };
        self.reparse(ObfuscatorError::Strings)
    }
}
