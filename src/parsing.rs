pub mod ast;
pub mod lexer;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar, "/parsing/grammar.rs");
