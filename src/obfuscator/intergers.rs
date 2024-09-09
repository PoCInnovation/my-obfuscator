use rand::{random, thread_rng, Rng};
use tree_sitter::{Tree, TreeCursor};

use super::{Obfuscator, Shiftable};

fn get_ints(tree: &Tree) -> Vec<std::ops::Range<usize>> {
    fn go(cursor: &mut TreeCursor, ints: &mut Vec<std::ops::Range<usize>>) {
        let node = cursor.node();
        if node.kind() == "integer" {
            ints.push(node.start_byte()..node.end_byte());
        }
        if cursor.goto_first_child() {
            go(cursor, ints);
            cursor.goto_parent();
        }
        while cursor.goto_next_sibling() {
            go(cursor, ints);
        }
    }
    let mut ints = Vec::new();
    let mut cursor = tree.walk();

    go(&mut cursor, &mut ints);

    ints
}

fn encode_int(int: &str) -> String {
    let int = int.parse::<i128>().expect("int litteral might be too big =(");
    let mut rng = thread_rng();
    let shift_key: i128 = rng.gen_range(1..16);
    let xor_key: i128 = rng.gen_range(1..u32::MAX as i128);
    let shift_mult: i128 = rng.gen_range(1..u32::MAX as i128);

    let encoded = int ^ (xor_key);
    let xor_key = xor_key << shift_key;
    let shift_key = shift_key * shift_mult;

    format!("({encoded} ^ (({xor_key} ) >> ({shift_key} // {shift_mult})))")
}

impl Obfuscator {
    pub fn obfuscate_integers(&mut self) {
        let ints = get_ints(&self.tree);
        let mut shift = 0;

        ints.into_iter().for_each(|int| {
            let int = int.shift(shift);
            let val = &self.code[int.clone()];
            shift -= val.len() as i32;
            let encoded = encode_int(val);

            self.code.replace_range(int, &encoded);
            shift += encoded.len() as i32;
        });
        self.reparse();
    }
}
