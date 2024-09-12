use crate::obfuscator::init;

use super::Obfuscator;
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

impl Obfuscator {
    pub fn instert_dead_branches(&mut self) {
        let lines = self.code.lines().count();
        let mut rng = rand::thread_rng();
        let iterations = rng.gen_range(1..lines / 3);

        for _ in 0..iterations {
            let line = rng.gen_range(init::OBFUSCATOR_HELPER_FUNCTIONS.lines().count()..lines); // skip the helper function
            let code = self.code.clone();

            self.code = code
                .lines()
                .enumerate()
                .map(|(i, l)| {
                    if i == line {
                        let indent = figure_out_indentation(l);
                        let mut line = DEAD_CODE_ENTRY_POINT
                            [rng.gen_range(0..DEAD_CODE_ENTRY_POINT.len())]
                        .to_string();
                        line.insert_str(0, " ".repeat(indent).as_str());
                        line.push('\n');
                        line.push_str(" ".repeat(indent + 4).as_str());
                        line.push_str(
                            RANDOM_USELESS_CODE[rng.gen_range(0..RANDOM_USELESS_CODE.len())],
                        );
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
            eprintln!("self.code: {}", self.code);
        }
        self.tree = self
            .parser
            .parse(&self.code, None)
            .expect("error reparsing after dead code insertion");
    }
}
