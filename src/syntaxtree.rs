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


// statement is a key word that does something
#[derive(Debug)]
pub enum Statement {

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

    While {
        condition: Expression,
        body: Box<Statement>,
    },

    Function {
        name: String,
        params: Vec<String>,
        body: Box<Statement>, // usually a Block
    },
}



