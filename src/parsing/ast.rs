#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    ///////////////////////////
    // Assignment statements //
    ///////////////////////////
    VariableDeclarationStatement { name: String, value: Box<Expression> },
    AssignmentStatement { name: String, value: Box<Expression> },

    /////////////////////
    // Flow statements //
    /////////////////////
    IfStatement { cond: Box<Expression>, then_part: Vec<Statement> },
    IfElseStatement { cond: Box<Expression>, then_part: Vec<Statement>, else_part: Vec<Statement> },
    WhileStatement { cond: Box<Expression>, body: Vec<Statement> },
    FunctionDeclaration { name: String, parameters: Vec<String>, body: Vec<Statement> },
    ReturnStatement { value: Box<Expression> },

    ////////////////////////////
    // Interactive statements //
    ////////////////////////////
    PrintStatement { content: Box<Expression> },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Float(f64),
    Int(i64),
    Identifier(String),
    Str(String),
    Bool(bool),
    FunctionCall {
        name: String,
        arguments: Vec<Box<Expression>>,
    },
    BinaryOperation {
        lhs: Box<Expression>,
        operator: BinaryOperator,
        rhs: Box<Expression>,
    },
    UnaryOperation {
        operator: UnaryOperator,
        rhs: Box<Expression>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Less,
    Greater,
    LessEq,
    GreaterEq,
    CompareEq,
    CompareNeq,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
    Minus,
}
