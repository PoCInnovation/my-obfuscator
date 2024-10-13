mod obfuscator;
use std::process::ExitCode;

use clap::{arg, value_parser, ArgAction, ArgMatches, Command, ValueHint};

use obfuscator::Obfuscator;
const AVAILABLE_OBFUSCATION_METHODS: [&str; 7] =
    ["int", "string", "fn", "bools", "dead", "rm_cmt", "call"];

fn run_obfuscator(mut obfuscator: Obfuscator, matches: ArgMatches) -> obfuscator::Result<()> {
    let run_all = !matches.contains_id("set");
    let set_options = matches
        .get_many::<String>("set")
        .map(|values| values.collect::<Vec<_>>())
        .unwrap_or_default(); // Default to empty if --set not provided

    if !obfuscator.is_syntax_ok()? {
        Err(obfuscator::error::ObfuscatorError::InvalidCode)?;
    }
    if run_all || set_options.contains(&&"rm_cmt".to_string()) {
        obfuscator.remove_comments_and_empty_lines()?;
    }
    if run_all || set_options.contains(&&"dead".to_string()) {
        obfuscator.insert_dead_branches()?;
    }
    if run_all || set_options.contains(&&"bools".to_string()) {
        obfuscator.obfuscate_booleans()?;
    }
    if set_options.contains(&&"call".to_string()) {
        obfuscator.obfuscate_function_calls()?;
    }
    if run_all || set_options.contains(&&"string".to_string()) {
        obfuscator.obfuscate_strings()?;
    }
    if run_all || set_options.contains(&&"fn".to_string()) {
        if set_options.contains(&&"call".to_string()) {
            eprintln!("fn identifier obfuscation was skipped because it is not compatible with call obfuscation");
        } else {
            obfuscator.obfuscate_functions()?;
        }
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

fn run_obfuscation(code: String, matches: ArgMatches) -> ExitCode {
    let obfuscator = match Obfuscator::new(code) {
        Ok(ob) => ob,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::SUCCESS;
        }
    };

    if let Err(err) = run_obfuscator(obfuscator, matches) {
        eprintln!("{err}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn main() -> ExitCode {
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
    let code = std::fs::read_to_string(file_name).expect("The Path given was wrong");
    run_obfuscation(code, matches)
}
