use crate::parsing::lexer::tokenize;

mod parsing;

fn main()
{
    let res = tokenize(&"1 +(3 * 2)".to_string());
    println!("{:?}", res)
}
