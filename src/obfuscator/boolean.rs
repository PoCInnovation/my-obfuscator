use super::error::ObfuscatorError;
use super::error::Result;
use super::Obfuscator;
use super::Shiftable;
use rand::thread_rng;
use rand::Rng;
use tree_sitter::{Tree, TreeCursor};

fn get_bools(tree: &Tree) -> Vec<std::ops::Range<usize>> {
    fn go(cursor: &mut TreeCursor, bools: &mut Vec<std::ops::Range<usize>>) {
        let node = cursor.node();
        if node.kind() == "true" || node.kind() == "false" {
            bools.push(node.start_byte()..node.end_byte());
        }
        if cursor.goto_first_child() {
            go(cursor, bools);
            cursor.goto_parent();
        }
        while cursor.goto_next_sibling() {
            go(cursor, bools);
        }
    }
    let mut bools = Vec::new();

    go(&mut tree.walk(), &mut bools);

    bools
}

fn random_args() -> String {
    let mut rng = thread_rng();
    let args = rng.gen_range(1..10);
    (0..args)
        .map(|_| {
            let arg: i32 = rng.gen();
            if rng.gen_bool(0.5) {
                arg.to_string()
            } else {
                format!("\"{}\"", arg)
            }
        })
        .collect::<Vec<String>>()
        .join(", ")
}

fn obfuctated_boolean(val: &str) -> String {
    let val = match val {
        "True" => format!("ohe_thruthy({})", random_args()),
        "False" => format!("ohe_falsy({})", random_args()),
        _ => unreachable!("boolean obfuscation failing most likely due to unparsed tree"),
    };
    format!(
        "({} {})",
        "not not ".repeat(thread_rng().gen_range(0..2)),
        val
    )
}

impl Obfuscator {
    pub fn obfuscate_booleans(&mut self) -> Result<()> {
        let bools = get_bools(&self.tree);
        let mut shift = 0;

        bools.into_iter().for_each(|boolean| {
            let boolean = boolean.shift(shift);
            let val = &self.code[boolean.clone()];
            shift -= val.len() as i32;
            let encoded = obfuctated_boolean(val);

            self.code.replace_range(boolean, &encoded);
            shift += encoded.len() as i32;
        });
        self.reparse(ObfuscatorError::Booleans)
    }
}
