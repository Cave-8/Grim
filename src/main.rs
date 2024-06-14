use std::fs::read_to_string;
use crate::parsing::grammar::ProgramParser;
use crate::parsing::lexer::Lexer;
use crate::interpreter::interpreter::{boot_interpreter};

mod parsing;
mod interpreter;

fn main()
{
    let source_code = read_to_string("src/examples/loop.grim").unwrap();
    let lexer = Lexer::new(source_code.as_str());
    let parser = ProgramParser::new();
    let ast = parser.parse(lexer).unwrap();
    let main_frame = boot_interpreter(&ast);

    // println!("{:#?}", ast);
    println!("Main frame: {:#?}", main_frame);
}