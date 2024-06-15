use std::env;
use std::fs::read_to_string;
use crate::interpreter::interpreter::boot_interpreter;
use crate::language_runner::run_language::run_program;
use crate::parsing::grammar::ProgramParser;
use crate::parsing::lexer::Lexer;

mod parsing;
mod interpreter;
mod language_runner;

fn main()
{
    let binding = read_to_string("src/examples/script.grim").unwrap();
    let src = binding.as_str();
    let lexer = Lexer::new(src);
    let parser = ProgramParser::new();
    let ast = parser.parse(lexer).unwrap();
    let main_frame = boot_interpreter(&ast);

    //
    //let args: Vec<String> = env::args().collect();
    //let source_code = read_to_string(&args[1]).unwrap();
    //run_program(&source_code);
}