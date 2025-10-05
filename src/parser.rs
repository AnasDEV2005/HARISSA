use crate::syntaxtree::{Statement, Expression};
use crate::tokenizer::Token;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {

// what im supposed to do here is, iterate over all the tokens, send them to parse_statement, while
// keeping track of the position
//
//
    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        let tokens: Vec<Token> = self.tokens.clone();
        for token in tokens.iter() {
            let t = token.clone();
            let stmt = self.parse_statements(t);
            statements.push(stmt);
            self.position += 1;
        }
        statements
    }

    pub fn parse_statements(&mut self, token: Token) -> Statement  {

        match token {
            Token::Number(n) => Statement::Expression(Expression::Number(n)),
            Token::String(s) => Statement::Expression(Expression::String(s)),
            Token::Symbol(chr) => self.parse_symbol(chr, self.position),
            Token::Keyword(k) => self.parse_keyword(k, self.position),
            Token::Identifier(i) => ,
            Token::Boolean(b) => ,
            Token::Operator(o) => ,
        }
        Statement::ConstDeclaration { name: ("x".to_string()), datatype: ("int".to_string()), value: Expression::Number("5".to_string()) } 
    }


    fn parse_symbol(&mut self, chr: char, pos: usize) -> Statement {
        Statement::ConstDeclaration { name: ("x".to_string()), datatype: ("int".to_string()), value: Expression::Number("5".to_string()) } 
    }






}





