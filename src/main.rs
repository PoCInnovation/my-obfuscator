mod obfuscator;
use std::env;

use obfuscator::Obfuscator;

fn run_obfuscation(mut obfuscator: Obfuscator) -> obfuscator::Result<()> {
    obfuscator.insert_dead_branches()?;
    obfuscator.obfuscate_booleans()?;
    obfuscator.obfuscate_strings()?;
    obfuscator.obfuctate_functions()?;
    obfuscator.obfuscate_integers()?;
    obfuscator.print_tree();
    println!("{}", obfuscator);
    Ok(())
}

fn main() -> obfuscator::Result<()> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    } else {
        run_obfuscation(Obfuscator::new(
            std::fs::read_to_string(&args[1]).expect("error reading file"),
        ))
    }
}
