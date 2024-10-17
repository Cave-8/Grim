use crate::interpreter::interpreter::boot_interpreter;
use crate::parsing::grammar::ProgramParser;
use crate::parsing::lexer::Lexer;
use colored::Colorize;

pub fn run_program(src: &String) {
    println!("Hi! \nGrim language interpreter started!\n");

    let lexer = Lexer::new(src.as_str());
    let parser = ProgramParser::new();
    let ast = parser.parse(lexer).unwrap();
    let _ = match boot_interpreter(&ast) {
        Ok(_) => (),
        Err(err) => {
            println!("{}", "ERROR!".red().bold());
            println!("{}", err);
        },
    };

    println!("\nGoodbye =)");
}
