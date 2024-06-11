pub mod lexer;
pub mod ast;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar, "/parsing/grammar.rs");