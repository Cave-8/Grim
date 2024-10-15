use crate::interpreter::interpreter::boot_interpreter;
use crate::parsing::grammar::ProgramParser;
use crate::parsing::lexer::Lexer;

pub fn run_program(src: &String) {
    println!("Hi! \nGrim language interpreter started!\n");

    let lexer = Lexer::new(src.as_str());
    let parser = ProgramParser::new();
    let ast = parser.parse(lexer).unwrap();
    let _ = match boot_interpreter(&ast) {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    };

    println!("\nGoodbye =)");
}
