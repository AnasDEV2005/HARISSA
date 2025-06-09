
use crate::syntaxtree::{Stmt, Expr};
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse_const(&mut self) -> Option<Stmt> {
        self.consume(Token::Const)?;

        let type_token = self.advance();
        let datatype = match type_token {
            Token::Type(t) => t,
            _ => return None, // you can add proper error handling here
        };

        let name_token = self.advance();
        let name = match name_token {
            Token::Identifier(n) => n,
            _ => return None,
        };

        self.consume(Token::Equals)?;
        let value = self.parse_expression()?; // Make sure you define this

        Some(Stmt::ConstDeclaration {
            name,
            datatype,
            value,
        })
    }

    fn consume(&mut self, expected: Token) -> Option<()> {
        if self.tokens.get(self.current)? == &expected {
            self.current += 1;
            Some(())
        } else {
            None
        }
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens.get(self.current).cloned().unwrap_or(Token::EOF);
        self.current += 1;
        token
    }

    // Dummy placeholder
    fn parse_expression(&mut self) -> Option<Expr> {
        // Implement this properly later
        Some(Expr::Number("0".to_string()))
    }
}

