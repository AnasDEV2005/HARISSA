// src/syntaxtree.rs





// expression is basically something that has a value
#[derive(Debug)]
pub enum Expression {
    Number(String),
    String(String),
    Boolean(bool),
    Identifier(String),
    
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
}


#[derive(Debug)]
pub enum LoopRange {
    Number(i32),
    Range((i32, i32)),
    Identifier(String),
    List(Vec<Expression>),
    InvalidRange(i32),
}


#[derive(Debug)]
pub enum Statement {

    PlaceHolder {},

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
        iterator: Option<String>,       // e.g., "i" (or None for plain `loop`)
        range: Option<LoopRange>,      // e.g., Expression::Identifier("count")
        body: Box<Statement>,           // usually a Block
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

    ParsingError {
        message: String,
        line: i32,
    }
}



