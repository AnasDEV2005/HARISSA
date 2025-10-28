// src/syntaxtree.rs

#[allow(warnings)]
// expression is basically something that has a value
#[derive(Debug)]
pub enum Expression {
    New(),
    Number(String),
    String(String),
    Boolean(bool),
    Identifier(String),

    Couple(Box<Expression>, Box<Expression>),

    Binary {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },

    Unary {
        operator: String,
        operand: Box<Expression>,
    },

    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },

    InvalidExpression {
        message: String,
        line: i32,
    },
}

#[derive(Debug)]
pub enum Statement {
    New(),

    ErrorRules {},

    ConstDeclaration {
        name: String,
        datatype: String,
        value: Expression,
    },

    MutDeclaration {
        name: String,
        datatype: String,
        value: Expression,
    },

    Expression(Expression),

    Block(Vec<Statement>),

    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },

    Loop {
        iterator: Option<String>, // e.g., "i" (or None for plain `loop`)
        range: Expression,        // e.g., Expression::Identifier("count")
        body: Box<Statement>,     // usually a Block
    },

    While {
        condition: Expression,
        body: Box<Statement>,
    },

    Function {
        name: String,
        params: Vec<String>,
        body: Box<Statement>, // usually a Block
    },

    Return {
        value: Box<Expression>,
    },

    ParsingError {
        message: String,
        line: i32,
    },
}
