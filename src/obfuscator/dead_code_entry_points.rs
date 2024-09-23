use crate::obfuscator::init;

use super::{
    error::{ObfuscatorError, Result},
    Obfuscator,
};
use rand::Rng;

const RANDOM_USELESS_CODE: [&str; 5] = [
    "a = 1",
    "\"astring\"",
    "print('hello' + 'world')",
    "print('hello' * 3)",
    "print('hello' * 3)",
];

const DEAD_CODE_ENTRY_POINT: [&str; 6] = [
    "if False:",
    "if not True:",
    "if 0 == 1:",
    "if 1 == 0:",
    "if 1 != 1:",
    "if 0 != 0:",
];

fn figure_out_indentation(line: &str) -> usize {
    let mut indent = 0;

    for c in line.chars() {
        if c == ' ' {
            indent += 1;
        } else {
            break;
        }
    }
    indent
}

fn insert_dead_branches(code: &str, attempt: usize) -> String {
    let lines = code.lines().count();
    let mut rng = rand::thread_rng();
    let iterations = rng.gen_range(1..(lines / 3) - attempt);
    let mut new_code: String = code.to_string();

    for _ in 0..iterations {
        let line = rng.gen_range(init::OBFUSCATOR_HELPER_FUNCTIONS.lines().count()..lines);

        new_code = new_code.lines().enumerate().map(|(i, l)| {
            if i == line {
                let indent = figure_out_indentation(l);
                let mut line = DEAD_CODE_ENTRY_POINT[rng.gen_range(0..DEAD_CODE_ENTRY_POINT.len())]
                    .to_string();
                line.insert_str(0, " ".repeat(indent).as_str());
                line.push('\n');
                line.push_str(" ".repeat(indent + 4).as_str());
                line.push_str(RANDOM_USELESS_CODE[rng.gen_range(0..RANDOM_USELESS_CODE.len())]);
                line.push('\n');
                line.push_str(l);
                line.push('\n');
                line
            } else {
                let mut l = l.to_string();
                l.push('\n');
                l
            }
        })
        .collect::<String>();
    }
    new_code
}

impl Obfuscator {
    pub fn insert_dead_branches(&mut self) -> Result<()> {
        let attempts :usize = 5;
        let old = self.code.clone();

        self.code = insert_dead_branches(&self.code, 0);
        for i in 1..attempts {
            if self.is_syntax_ok()? {
                self.reparse(ObfuscatorError::DeadCode)?;
                return Ok(());
            } else {
                self.code = insert_dead_branches(&old, i);
            }
        }
        if self.is_syntax_ok()? {
            self.reparse(ObfuscatorError::DeadCode)?;
            Ok(())
        } else {
            Err(ObfuscatorError::DeadCode)
        }
    }
}
