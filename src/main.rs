mod obfuscator;
use clap::{arg, value_parser, ArgAction, ArgMatches, Command, ValueHint};

use obfuscator::Obfuscator;
const AVAILABLE_OBFUSCATION_METHODS: [&str; 5] = ["int", "string", "fn", "bools", "dead"];

fn run_obfuscation(mut obfuscator: Obfuscator, matches: ArgMatches) -> obfuscator::Result<()> {
    let run_all = !matches.contains_id("set");
    let set_options = matches
        .get_many::<String>("set")
        .map(|values| values.collect::<Vec<_>>())
        .unwrap_or_default(); // Default to empty if --set not provided

    if run_all || set_options.contains(&&"dead".to_string()) {
        obfuscator.insert_dead_branches()?;
    }
    if run_all || set_options.contains(&&"bools".to_string()) {
        obfuscator.obfuscate_booleans()?;
    }
    if run_all || set_options.contains(&&"string".to_string()) {
        obfuscator.obfuscate_strings()?;
    }
    if run_all || set_options.contains(&&"fn".to_string()) {
        obfuscator.obfuscate_functions()?;
    }
    if run_all || set_options.contains(&&"int".to_string()) {
        obfuscator.obfuscate_integers()?;
    }

    if matches.get_flag("debug") {
        obfuscator.print_tree();
    }
    println!("{}", obfuscator);
    Ok(())
}

fn main() -> obfuscator::Result<()> {
    let matches = Command::new("Obfuscator")
        .version("0.0")
        .about("static obfuscation of python code")
        .arg(
            arg!(<script> "Script to obfuscate")
                .required(true)
                .value_parser(value_parser!(std::path::PathBuf))
                .value_hint(ValueHint::FilePath),
        )
        .arg(
            arg!(-d --debug "dumps more debug stuff, for now simply prints the syntax tree")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(-s --set [list] "comma separated list of options")
                .required(false)
                .num_args(0..=AVAILABLE_OBFUSCATION_METHODS.len())
                .value_parser(AVAILABLE_OBFUSCATION_METHODS),
        )
        .get_matches();

    let file_name = matches.get_one::<std::path::PathBuf>("script").unwrap();
    run_obfuscation(
        Obfuscator::new(std::fs::read_to_string(file_name).expect("Must be a real file"))?,
        matches,
    )
}
