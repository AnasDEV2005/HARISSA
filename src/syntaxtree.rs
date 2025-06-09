// src/syntaxtree.rs

#[derive(Debug)]
pub enum Expr {
    Number(String),
    String(String),
    Boolean(bool),
    Identifier(String),
    
    Binary {
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>,
    },

    Unary {
        operator: String,
        operand: Box<Expr>,
    },

    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
}

#[derive(Debug)]
pub enum Stmt {
    LetDeclaration {
        name: String,
        value: Expr,
    },

    ConstDeclaration {
        name: String,
        datatype: String,
        value: Expr,
    },

    MutDeclaration {
        name: String,
        datatype: String,
        value: Expr,
    },

    Expression(Expr),

    Block(Vec<Stmt>),

    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },

    While {
        condition: Expr,
        body: Box<Stmt>,
    },

    Function {
        name: String,
        params: Vec<String>,
        body: Box<Stmt>, // usually a Block
    },
}


