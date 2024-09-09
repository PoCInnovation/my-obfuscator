mod strings;
mod print;
mod functions;
mod init;
mod intergers;
mod boolean;
mod dead_code_entry_points;
mod random_identifiers;

use tree_sitter::{Parser, Tree};

pub struct Obfuscator {
    code: String,
    parser: Parser,
    tree: Tree,
}

trait Shiftable {
    fn shift(&self, shift: i32) -> Self
    where
        Self: Sized;
}

impl Shiftable for std::ops::Range<usize> {
    fn shift(&self, shift: i32) -> Self {
        let sign = shift < 0;
        let shift: usize = shift.abs().try_into().unwrap();

        if sign {
            if shift > self.start {
                return 0..0;
            }
            (self.start - shift)..(self.end - shift)
        } else {
            (self.start + shift)..(self.end + shift)
        }
    }
}
