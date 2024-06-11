use crate::parsing::grammar::ProgramParser;
use crate::parsing::lexer::Lexer;

mod parsing;

fn main()
{
    let source_code = "let a = 42;
                               let b = 23 + 7 - (3 * 2);
                               b = 0;
                               let c = true;
                               if 3 + 10 - 3 <= 20 { b = 1; } else { b = 0; }
                               return true;";

    let lexer = Lexer::new(source_code);
    let parser = ProgramParser::new();
    let ast = parser.parse(lexer).unwrap();

    println!("{:?}", ast);
}