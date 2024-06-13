use std::fs::read_to_string;
use crate::parsing::grammar::ProgramParser;
use crate::parsing::lexer::Lexer;
use crate::interpreter::interpreter::{evaluate_ast, Frame};

mod parsing;
mod interpreter;

fn main()
{
    let source_code = read_to_string("src/examples/assignment.grim").unwrap();
    let lexer = Lexer::new(source_code.as_str());
    let parser = ProgramParser::new();
    let ast = parser.parse(lexer).unwrap();
    let mut starting_frame = Box::new(Frame::default());
    let frame = evaluate_ast(&ast, &mut starting_frame);

    // println!("{:#?}", ast);
    println!("Main frame: {:#?}", frame);
}