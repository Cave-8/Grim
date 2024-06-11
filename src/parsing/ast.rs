use crate::parsing::lexer::{Token};

#[derive(Debug, PartialEq)]
pub enum StatementType {
    LetStatement,
}

pub struct Program {
    pub(crate) statements: Vec<Box<dyn Statement>>
}
pub trait Statement: std::fmt::Debug {
    fn show(&self);
}

// Possible statements
/// Let statement
#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub(crate) token: Token,
    pub(crate) name: Identifier,
    pub(crate) value: Expression,
}

impl Statement for LetStatement {
    fn show(&self) {
        println!("Token: {:?}, Identifier: {:?}, Expression: {:?}", self.token, self.name, self.value)
    }
}

/// Return statement
#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    pub(crate) token: Token,
    pub(crate) value: Expression,
}

// Auxiliary structs
#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub(crate) token: Token,
    pub(crate) value: String
}

#[derive(Debug, PartialEq)]
pub struct Expression {

}