use crate::language_runner::run_language::run_program;
use colored::Colorize;
use std::env;
use std::fs::read_to_string;
use std::process::exit;

mod interpreter;
mod language_runner;
mod parsing;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!(
            "{}",
            "ERROR!\nPlease, insert the path of only one valid .grim file".bright_red()
        );
        exit(1);
    }
    let source_code = read_to_string(&args[1]).unwrap();
    run_program(&source_code);
}
