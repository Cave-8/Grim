/// Range of possible statements
#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    ///////////////////////////
    // Assignment statements //
    ///////////////////////////
    VariableDeclarationStatement {
        name: String,
        value: Box<Expression>,
    },
    AssignmentStatement {
        name: String,
        value: Box<Expression>,
    },

    /////////////////////
    // Flow statements //
    /////////////////////
    IfStatement {
        cond: Box<Expression>,
        then_part: Vec<Statement>,
    },
    IfElseStatement {
        cond: Box<Expression>,
        then_part: Vec<Statement>,
        else_part: Vec<Statement>,
    },
    WhileStatement {
        cond: Box<Expression>,
        body: Vec<Statement>,
    },
    FunctionDeclaration {
        name: String,
        arguments: Vec<String>,
        body: Vec<Statement>,
    },
    FunctionCallStatement {
        name: String,
        arguments: Vec<Box<Expression>>,
    },
    ReturnStatement {
        value: Box<Expression>,
    },

    ////////////////////
    // I/O statements //
    ////////////////////
    PrintStatement {
        content: Box<Expression>,
    },
    PrintLineStatement {
        content: Box<Expression>,
    },
    InputStatement {
        name: String,
    },
}

/// Range of possible expressions.
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

/// Range of possible binary operators.
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

/// Range of possible unitary operator.
#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
    Minus,
}
