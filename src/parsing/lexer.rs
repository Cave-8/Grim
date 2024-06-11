use crate::parsing::lexer::TokenType::*;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    TokNumber,     // Number
    TokIdentifier, // Identifiers
    TokLpar,       // (
    TokRpar,       // )
    TokLbrace,     // {
    TokRbrace,     // }
    TokLsquare,    // [
    TokRsquare,    // ]
    TokEquals,     // =
    TokPlus,       // +
    TokMinus,      // -
    TokTimes,      // *
    TokDivides,    // /
    TokComma,      //- ,
    TokSemi,       // ;
    TokLess,       // <
    TokGreater,    // >
    TokLessEq,     // <=
    TokGreaterEq,  // >=
    TokEq,         // ==
    TokNeq,        // !=
    TokNot,        // logic NOT
    TokAnd,        // logic AND
    TokOr,         // logic OR
    TokLet,        // "let"
    TokIf,         // "if"
    TokElse,       // "else"
    TokFn,         // "fn"
    TokReturn,     // "return"
    TokEOF,
    TokIllegal,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) value: String,
}

/// Given the code, it returns a list of tokens.
///
/// src: source code.
pub fn tokenize(src: &String) -> Vec<Token> {
    let binary_operations = vec!["+", "-", "/", "*"];
    let parenthesis = vec!["(", ")", "[", "]", "{", "}"];
    let logical = vec!["!", "||", "&&"];
    let comparisons = vec!["=", "<", ">", "<=", ">=", "==", "!="];
    let keywords = vec!["let", "fn", "if", "else", "return"];

    let mut tokens: Vec<Token> = Vec::new();
    let mut chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut already_read = 0;

    for mut i in 0..len {
        if already_read > 0 {
            already_read -= 1;
            continue;
        }

        let curr_c = chars.get(i).unwrap();
        let next_c = chars.get(i + 1);

        // Parse number and identifiers
        if curr_c.is_digit(10) {
            let mut num = String::from("");
            for j in i..len {
                if chars.get(j).unwrap().is_digit(10) {
                    num.push(*chars.get(j).unwrap());
                    already_read += 1;
                } else {
                    already_read -= 1;
                    break;
                }
            }
            tokens.push(Token {
                token_type: TokNumber,
                value: num,
            })
        } else if curr_c.is_alphabetic() {
            let mut ident = String::from("");
            for j in i..len {
                if chars.get(j).unwrap().is_alphanumeric() {
                    ident.push(*chars.get(j).unwrap());
                    already_read += 1;
                } else {
                    already_read -= 1;
                    break;
                }
            }
            if keywords.contains(&&*ident) {
                match ident.as_str() {
                    "let" => tokens.push(Token {
                        token_type: TokLet,
                        value: ident,
                    }),
                    "if" => tokens.push(Token {
                        token_type: TokIf,
                        value: ident,
                    }),
                    "else" => tokens.push(Token {
                        token_type: TokElse,
                        value: ident,
                    }),
                    "fn" => tokens.push(Token {
                        token_type: TokFn,
                        value: ident,
                    }),
                    "return" => tokens.push(Token {
                        token_type: TokReturn,
                        value: ident,
                    }),
                    _ => (),
                }
            } else {
                tokens.push(Token {
                    token_type: TokIdentifier,
                    value: ident,
                })
            }
        } else {
            match curr_c {
                ' ' | '\n' | '\t' => (),
                '(' => tokens.push(Token {
                    token_type: TokLpar,
                    value: String::from(*curr_c),
                }),
                ')' => tokens.push(Token {
                    token_type: TokRpar,
                    value: String::from(*curr_c),
                }),
                '[' => tokens.push(Token {
                    token_type: TokLsquare,
                    value: String::from(*curr_c),
                }),
                ']' => tokens.push(Token {
                    token_type: TokRsquare,
                    value: String::from(*curr_c),
                }),
                '{' => tokens.push(Token {
                    token_type: TokLbrace,
                    value: String::from(*curr_c),
                }),
                '}' => tokens.push(Token {
                    token_type: TokRbrace,
                    value: String::from(*curr_c),
                }),
                '+' => tokens.push(Token {
                    token_type: TokPlus,
                    value: String::from(*curr_c),
                }),
                '-' => tokens.push(Token {
                    token_type: TokMinus,
                    value: String::from(*curr_c),
                }),
                '*' => tokens.push(Token {
                    token_type: TokTimes,
                    value: String::from(*curr_c),
                }),
                '/' => tokens.push(Token {
                    token_type: TokDivides,
                    value: String::from(*curr_c),
                }),
                '=' => match next_c {
                    Some(c) => {
                        if *c == '=' {
                            tokens.push(Token {
                                token_type: TokEq,
                                value: String::from("=="),
                            });
                            already_read += 1;
                        } else {
                            tokens.push(Token {
                                token_type: TokEquals,
                                value: String::from(*curr_c),
                            });
                        }
                    }
                    None => {
                        tokens.push(Token {
                            token_type: TokEquals,
                            value: String::from(*curr_c),
                        });
                    }
                },
                '<' => match next_c {
                    Some(c) => {
                        if *c == '=' {
                            tokens.push(Token {
                                token_type: TokLessEq,
                                value: String::from("<="),
                            });
                            already_read += 1;
                        } else {
                            tokens.push(Token {
                                token_type: TokLess,
                                value: String::from(*curr_c),
                            });
                        }
                    }
                    None => {
                        tokens.push(Token {
                            token_type: TokLess,
                            value: String::from(*curr_c),
                        });
                    }
                },
                '>' => match next_c {
                    Some(c) => {
                        if *c == '=' {
                            tokens.push(Token {
                                token_type: TokGreaterEq,
                                value: String::from(">="),
                            });
                            already_read += 1;
                        } else {
                            tokens.push(Token {
                                token_type: TokGreater,
                                value: String::from(*curr_c),
                            });
                        }
                    }
                    None => {
                        tokens.push(Token {
                            token_type: TokGreater,
                            value: String::from(*curr_c),
                        });
                    }
                },
                '!' => match next_c {
                    Some(c) => {
                        if *c == '=' {
                            tokens.push(Token {
                                token_type: TokNeq,
                                value: String::from(*curr_c),
                            });
                            already_read += 1;
                        } else {
                            tokens.push(Token {
                                token_type: TokNot,
                                value: String::from(*curr_c),
                            });
                        }
                    }
                    None => {
                        tokens.push(Token {
                            token_type: TokNot,
                            value: String::from(*curr_c),
                        });
                    }
                },
                '|' => match next_c {
                    Some(c) => {
                        if *c == '|' {
                            tokens.push(Token {
                                token_type: TokOr,
                                value: String::from("||"),
                            });
                            already_read += 1;
                        } else {
                            panic!("Expected |")
                        }
                    }
                    None => {
                        panic!("Expected |")
                    }
                },
                '&' => match next_c {
                    Some(c) => {
                        if *c == '&' {
                            tokens.push(Token {
                                token_type: TokAnd,
                                value: String::from("&&"),
                            });
                            already_read += 1;
                        } else {
                            panic!("Expected &")
                        }
                    }
                    None => {
                        panic!("Expected &")
                    }
                },
                ';' => tokens.push(Token {
                    token_type: TokSemi,
                    value: String::from(*curr_c),
                }),
                ',' => tokens.push(Token {
                    token_type: TokComma,
                    value: String::from(*curr_c),
                }),
                _ => panic!("Error, unexpected token -> {}", curr_c),
            }
        }
    }
    tokens.push(Token {
        token_type: TokEOF,
        value: String::from("EOF"),
    });

    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer_test_1() {
        let src = String::from("let test = 120;");

        let mut tokens = tokenize(&src);
        assert_eq!(
            Token {
                token_type: TokEOF,
                value: "EOF".to_string()
            },
            tokens.pop().unwrap()
        );
        assert_eq!(
            Token {
                token_type: TokSemi,
                value: ";".to_string()
            },
            tokens.pop().unwrap()
        );
        assert_eq!(
            Token {
                token_type: TokNumber,
                value: "120".to_string()
            },
            tokens.pop().unwrap()
        );
        assert_eq!(
            Token {
                token_type: TokEquals,
                value: "=".to_string()
            },
            tokens.pop().unwrap()
        );
        assert_eq!(
            Token {
                token_type: TokIdentifier,
                value: "test".to_string()
            },
            tokens.pop().unwrap()
        );
        assert_eq!(
            Token {
                token_type: TokLet,
                value: "let".to_string()
            },
            tokens.pop().unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn tokenizer_test_2() {
        let src = String::from("let test = 120;?");
        let tokens = tokenize(&src);
    }

    #[test]
    #[should_panic]
    fn tokenizer_test_3() {
        let src = String::from("&|");
        let tokens = tokenize(&src);
    }
    // Wrong syntax, just for testing
    #[test]
    fn tokenizer_test_4() {
        let src = String::from("<= >= == ! || &&");
        let mut tokens = tokenize(&src);

        assert_eq!(
            Token {
                token_type: TokEOF,
                value: "EOF".to_string()
            },
            tokens.pop().unwrap()
        );
        assert_eq!(
            Token {
                token_type: TokAnd,
                value: "&&".to_string()
            },
            tokens.pop().unwrap()
        );
        assert_eq!(
            Token {
                token_type: TokOr,
                value: "||".to_string()
            },
            tokens.pop().unwrap()
        );
        assert_eq!(
            Token {
                token_type: TokNot,
                value: "!".to_string()
            },
            tokens.pop().unwrap()
        );
        assert_eq!(
            Token {
                token_type: TokEq,
                value: "==".to_string()
            },
            tokens.pop().unwrap()
        );
        assert_eq!(
            Token {
                token_type: TokGreaterEq,
                value: ">=".to_string()
            },
            tokens.pop().unwrap()
        );
        assert_eq!(
            Token {
                token_type: TokLessEq,
                value: "<=".to_string()
            },
            tokens.pop().unwrap()
        );
    }
}
