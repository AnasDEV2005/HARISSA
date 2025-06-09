#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let, // changing a mutable variable 
         // let x = 1 
    Const, // defining an immutable variable
           // const TYPE x = 1
    Mut, // defining a mutable variable
         // mut TYPE x = 1
    Type(String),
    Identifier(String),
    Number(String),
    Equals,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Newline,
    EOF,
    Unknown(char),
}

