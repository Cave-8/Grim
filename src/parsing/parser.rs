use crate::parsing::ast::{Expression, Identifier, LetStatement, Program, ReturnStatement, Statement};
use crate::parsing::lexer::{Token, TokenType};
use crate::parsing::lexer::TokenType::{TokEOF, TokEquals, TokIdentifier, TokLet, TokReturn, TokSemi};


/// Parse the entire program.
fn parse(tokens: &Vec<Token>) -> Program {
    // Program, it will contain all statements
    let mut program: Program = Program { statements: vec![] };
    let mut read_tokens = 0;

    // Iterate over tokens vector
    for i in 0..tokens.len() {
        if read_tokens > 0 {
            read_tokens -= 1;
            continue;
        }

        let curr_token = tokens.get(i).unwrap();

        // Terminate
        if curr_token.token_type == TokEOF {
            break;
        }

        let (curr_stmt, consumed_tokens) = parse_statement(&tokens, &i);
        read_tokens = consumed_tokens;
        program.statements.push(curr_stmt);
    }

    return program;
}

/// Parse current statement.
///
/// tokens - the vector containing tokens.
///
/// index - current position in tokens vector.
fn parse_statement(tokens: &Vec<Token>, index: &usize) -> (Box<dyn Statement>, usize) {
    let curr_token: &Token = tokens.get(*index).unwrap();

    return match curr_token.token_type {
        TokLet => {
            let (stmt, consumed_tokens) = parse_let_statement(&tokens, &index);
            (stmt, consumed_tokens)
        }
        TokReturn => {
            let (stmt, consumed_tokens) = parse_return_statement(&tokens, &index);
            (stmt, consumed_tokens)
        }
        _ => {
            let (stmt, consumed_tokens) = parse_let_statement(&tokens, &index);
            (stmt, 0)
        }
    };
}

/// Parse let statement, return (LetStatement, consumed_tokens), panic if a syntax error is found.
///
/// tokens - the vector containing tokens.
///
/// index - current position in tokens vector.
fn parse_let_statement(tokens: &Vec<Token>, index: &usize) -> (Box<dyn Statement>, usize) {
    // Locally consumed tokens
    let mut local_index: usize = 0;
    // Placeholder values
    let mut stmt_to_return = LetStatement {
        token: Token { token_type: TokLet, value: "let".to_string() },
        name: Identifier { token: Token { token_type: TokIdentifier, value: "id".to_string() }, value: "".to_string() },
        value: Expression {},
    };

    // Parse TokLet -> "let"
    let mut curr_token: &Token = tokens.get(*index).unwrap();
    if curr_token.token_type != TokLet {
        panic!("Syntax error, expected let keyword - got {:?}", curr_token.value)
    }
    stmt_to_return.token = Token {
        token_type: TokLet,
        value: "let".to_string(),
    };
    local_index += 1;

    // Parse TokIdentifier
    curr_token = tokens.get(index + local_index).unwrap();
    if curr_token.token_type != TokIdentifier {
        panic!("Syntax error, expected an identifier - got {:?}", curr_token.value)
    }
    stmt_to_return.name = Identifier {
        token: Token { token_type: TokIdentifier, value: curr_token.value.clone() },
        value: curr_token.value.clone(),
    };
    local_index += 1;

    // Parse TokEquals -> =
    curr_token = tokens.get(index + local_index).unwrap();
    if curr_token.token_type != TokEquals {
        panic!("Syntax error, expected = - got {:?}", curr_token.value)
    }
    local_index += 1;

    // todo!(Expression parsing)

    // JUST FOR TEMPORARY TESTING
    loop {
        curr_token = tokens.get(index + local_index).unwrap();
        if curr_token.token_type == TokSemi {
            break;
        }
        local_index += 1;
    }

    // Parse TokSemi -> ;
    if curr_token.token_type != TokSemi {
        panic!("Syntax error, expected ; - got {:?}", curr_token.value)
    }
    local_index += 1;

    return (Box::new(stmt_to_return), local_index);
}

/// Parse return statement, return (ReturnStatement, consumed_tokens), panic if a syntax error is found.
///
/// tokens - the vector containing tokens.
///
/// index - current position in tokens vector.
fn parse_return_statement(tokens: &Vec<Token>, index: &usize) -> (Box<dyn Statement>, usize) {
    // Locally consumed tokens
    let mut local_index: usize = 0;
    // Placeholder values
    let mut stmt_to_return = ReturnStatement {
        token: Token { token_type: TokReturn, value: "return".to_string() },
        value: Expression {},
    };

    // Parse TokReturn -> "return"
    let mut curr_token: &Token = tokens.get(*index).unwrap();
    if curr_token.token_type != TokReturn {
        panic!("Syntax error, expected return keyword - got {:?}", curr_token.value)
    }
    stmt_to_return.token = Token {
        token_type: TokReturn,
        value: "return".to_string(),
    };
    local_index += 1;

    // todo!(Expression parsing)

    // JUST FOR TEMPORARY TESTING
    loop {
        curr_token = tokens.get(index + local_index).unwrap();
        if curr_token.token_type == TokSemi {
            break;
        }
        local_index += 1;
    }

    // Parse TokSemi -> ;
    if curr_token.token_type != TokSemi {
        panic!("Syntax error, expected ; - got {:?}", curr_token.value)
    }
    local_index += 1;

    return (Box::new(stmt_to_return), local_index);
}

mod tests {
    use crate::parsing::ast::*;
    use crate::parsing::parser::*;
    use crate::parsing::lexer::*;
    use super::*;

    #[test]
    fn parser_let_test() {
        let tokens = tokenize(&"let x = 10;".to_string());
        let parsed = parse(&tokens);
    }
    #[test]
    #[should_panic]
    fn parser_let_test_error_1() {
        let tokens = tokenize(&"let * = 10;".to_string());
        let parsed = parse(&tokens);
    }
}