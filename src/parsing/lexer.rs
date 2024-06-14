use std::fmt;
use std::num::ParseIntError;
use logos::{Logos, SpannedIter};

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger(ParseIntError),
    #[default]
    InvalidToken,
}

impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        LexicalError::InvalidInteger(err)
    }
}


#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+", skip r"#.*\n?", error = LexicalError)]
pub enum Token {
    #[regex("[0-9][.][0-9]+", | lex | lex.slice().parse::< f64 > ().unwrap())]
    TokFloat(f64),      // Float Number
    #[regex("[0-9]*", | lex | lex.slice().parse::< i64 > ().unwrap())]
    TokInt(i64),        // Integer number
    #[regex("[a-z][a-zA-Z0-9]*", | lex | lex.slice().to_owned())]
    TokIdentifier(String), // Identifiers
    #[regex("[\"][a-zA-Z0-9]*[\"]", | lex | lex.slice().to_owned())]
    TokString(String), // String
    #[regex("true|false", | lex | lex.slice().parse::< bool > ().unwrap())]
    TokBool(bool),    // Boolean
    #[token("(")]
    TokLpar,       // (
    #[token(")")]
    TokRpar,       // )
    #[token("{")]
    TokLbrace,     // {
    #[token("}")]
    TokRbrace,     // }
    #[token("[")]
    TokLsquare,    // [
    #[token("]")]
    TokRsquare,    // ]
    #[token("=")]
    TokEquals,     // =
    #[token("+")]
    TokPlus,       // +
    #[token("-")]
    TokMinus,      // -
    #[token("*")]
    TokTimes,      // *
    #[token("/")]
    TokDivide,    // /
    #[token(",")]
    TokComma,      //- ,
    #[token(";")]
    TokSemi,       // ;
    #[token(":")]
    TokColon,       // ;
    #[token("<")]
    TokLess,       // <
    #[token(">")]
    TokGreater,    // >
    #[token("<=")]
    TokLessEq,     // <=
    #[token(">=")]
    TokGreaterEq,  // >=
    #[token("==")]
    TokCompareEq,  // ==
    #[token("!=")]
    TokCompareNeq, // !=
    #[token("!")]
    TokNot,        // logic NOT
    #[token("&&")]
    TokAnd,        // logic AND
    #[token("||")]
    TokOr,         // logic OR
    #[token("->")]
    TokArrow,
    #[token("let")]
    TokLet,        // "let"
    #[token("if")]
    TokIf,         // "if"
    #[token("else")]
    TokElse,       // "else"
    #[token("fn")]
    TokFn,         // "fn"
    #[token("while")]
    TokWhile,         // "fn"
    #[token("return")]
    TokReturn,     // "return"
    #[token("print")]
    TokPrint,   // "print"
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Logos to LALRPOP

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'input> {
    token_stream: SpannedIter<'input, Token>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self { token_stream: Token::lexer(input).spanned() }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream
            .next()
            .map(|(token, span)| Ok((span.start, token?, span.end)))
    }
}

#[cfg(test)]
mod tests {
    use crate::parsing::lexer::Token;
    use super::*;

    #[test]
    fn tokenizer_test_1() {
        let src: &str = "let test = 120; let test1 = 0;";
        let mut lex = Token::lexer(&src);

        assert_eq!(lex.next(), Some(Ok(Token::TokLet)));
        assert_eq!(lex.next(), Some(Ok(Token::TokIdentifier("test".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::TokEquals)));
        assert_eq!(lex.next(), Some(Ok(Token::TokInt(120))));
        assert_eq!(lex.next(), Some(Ok(Token::TokSemi)));
        assert_eq!(lex.next(), Some(Ok(Token::TokLet)));
        assert_eq!(lex.next(), Some(Ok(Token::TokIdentifier("test1".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::TokEquals)));
        assert_eq!(lex.next(), Some(Ok(Token::TokInt(0))));
        assert_eq!(lex.next(), Some(Ok(Token::TokSemi)))
    }

    #[test]
    #[should_panic]
    fn tokenizer_test_2() {
        let src: &str = "&|;";
        let lex = Token::lexer(&src);

        for res in lex {
            match res {
                Ok(_) => {}
                Err(_) => { panic!() }
            }
        }
    }

    #[test]
    fn tokenizer_test_3() {
        let src: &str = "<= >= == != &&";
        let mut lex = Token::lexer(&src);

        assert_eq!(lex.next(), Some(Ok(Token::TokLessEq)));
        assert_eq!(lex.next(), Some(Ok(Token::TokGreaterEq)));
        assert_eq!(lex.next(), Some(Ok(Token::TokCompareEq)));
        assert_eq!(lex.next(), Some(Ok(Token::TokCompareNeq)));
        assert_eq!(lex.next(), Some(Ok(Token::TokAnd)))
    }

    #[test]
    fn tokenizer_test_4() {
        let src: &str = "let test = 0.123; let test1 = 0.0;";
        let mut lex = Token::lexer(&src);

        assert_eq!(lex.next(), Some(Ok(Token::TokLet)));
        assert_eq!(lex.next(), Some(Ok(Token::TokIdentifier("test".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::TokEquals)));
        assert_eq!(lex.next(), Some(Ok(Token::TokFloat(0.123))));
        assert_eq!(lex.next(), Some(Ok(Token::TokSemi)));
        assert_eq!(lex.next(), Some(Ok(Token::TokLet)));
        assert_eq!(lex.next(), Some(Ok(Token::TokIdentifier("test1".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::TokEquals)));
        assert_eq!(lex.next(), Some(Ok(Token::TokFloat(0.0))));
        assert_eq!(lex.next(), Some(Ok(Token::TokSemi)))
    }

    #[test]
    fn tokenizer_test_5() {
        let src: &str = "let test = true; let test1 = false;";
        let mut lex = Token::lexer(&src);

        assert_eq!(lex.next(), Some(Ok(Token::TokLet)));
        assert_eq!(lex.next(), Some(Ok(Token::TokIdentifier("test".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::TokEquals)));
        assert_eq!(lex.next(), Some(Ok(Token::TokBool(true))));
        assert_eq!(lex.next(), Some(Ok(Token::TokSemi)));
        assert_eq!(lex.next(), Some(Ok(Token::TokLet)));
        assert_eq!(lex.next(), Some(Ok(Token::TokIdentifier("test1".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::TokEquals)));
        assert_eq!(lex.next(), Some(Ok(Token::TokBool(false))));
        assert_eq!(lex.next(), Some(Ok(Token::TokSemi)))
    }
}