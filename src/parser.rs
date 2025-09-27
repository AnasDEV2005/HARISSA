use crate::syntaxtree::{Statement, Expression};
use crate::tokenizer::Token;


pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while !self.tokens.is_at_end() {
            statements.push(self.parse_statement());
        }
        statements
    }

    pub fn parse_statements(&mut self, statement: Token) -> Statement  {

    }







}





