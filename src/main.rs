mod obfuscator;
use std::env;

use obfuscator::Obfuscator;

fn main() {
    let code = {
        let args = env::args().collect::<Vec<String>>();
        if args.len() < 2 {
            eprintln!("Usage: {} <file>", args[0]);
            std::process::exit(1);
        } else {
            std::fs::read_to_string(&args[1]).expect("error reading file")
        }
    };
    let mut obfuscator = Obfuscator::new(code);

    obfuscator.instert_dead_branches();
    obfuscator.obfuscate_strings();
    obfuscator.obfuctate_functions();
    //    obfuscator.print_tree();
    println!("{}", obfuscator);
}
