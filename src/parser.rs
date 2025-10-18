
use crate::syntaxtree::{Statement, Expression, LoopRange};
use crate::tokenizer::Token;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub position: usize,
}



impl Parser {
    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            let stmt = self.parse_statement(token);
            statements.push(stmt);
            self.position += 1;
        }
        statements
    }

    pub fn parse_statement(&mut self, token: Token) -> Statement {
        match token {
            Token::Number(n) => Statement::Expression(Expression::Number(n)),
            Token::String(s) => Statement::Expression(Expression::String(s)),
            Token::Keyword(k) => self.parse_keyword(k, self.position),
            Token::Symbol(chr) => self.parse_symbol(chr, self.position),
            Token::Identifier(i) => self.parse_identifier(i, self.position),
            Token::Boolean(b) => Statement::ConstDeclaration {
                name: "x".to_string(),
                datatype: "int".to_string(),
                value: Expression::Boolean(b),
            },
            Token::Operator(o) => self.parse_operator(o, self.position),
        }
    }




    fn parse_keyword(&mut self, keyword: String, _pos: usize) -> Statement {
        match keyword.as_str() {
            "if" => {
                let condition = self.collect_condition();
                let block = self.collect_block();
                Statement::If {
                    condition: self.parse_condition(condition),
                    then_branch: Box::new(Statement::Block(self.parse_block(block))),
                    else_branch: None,
                }
            }

            // iterator: checks if there is an identifier after impl Trait for Type 
            // range: either, length of the variable after the iterator, or the range specified
            
            "loop" => {println!("{:?}", self.parse_loop()); return self.parse_loop()},

            "while" => Statement::While {
                condition: Expression::Identifier("cond".into()),
                body: Box::new(Statement::Expression(Expression::Identifier("body".into()))),
            },

            "fn" => Statement::Function {
                name: "f".into(),
                params: vec![],
                body: Box::new(Statement::Expression(Expression::Identifier("body".into()))),
            },

            "const" => Statement::ConstDeclaration {
                name: "x".into(),
                datatype: "int".into(),
                value: Expression::Number("0".into()),
            },

            "elif" => Statement::Expression(Expression::Identifier("elif".into())),

            "string" => Statement::Expression(Expression::Identifier("string".into())),

            "int" => Statement::Expression(Expression::Identifier("int".into())),

            "list" => Statement::Expression(Expression::Identifier("list".into())),

            "return" => Statement::Expression(Expression::Identifier("return".into())),

            "object" => Statement::Expression(Expression::Identifier("object".into())),

            "enum" => Statement::Expression(Expression::Identifier("object".into())),

            _ => Statement::Expression(Expression::Identifier("unknown".into())),
        }
    }





    fn parse_symbol(&mut self, _chr: char, _pos: usize) -> Statement {
        Statement::Expression(Expression::Identifier("symbol".into()))
    }

    fn parse_identifier(&mut self, _identifier: String, _pos: usize) -> Statement {
        Statement::Expression(Expression::Identifier("ident".into()))
    }

    fn parse_operator(&mut self, _operator: String, _pos: usize) -> Statement {
        Statement::Expression(Expression::Identifier("operator".into()))
    }

    fn parse_expression(&mut self) -> Expression {
        self.position += 1;
        if self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                Token::Number(n) => Expression::Number(n.clone()),
                _ => Expression::Identifier("expr".into()),
            }
        } else {
            Expression::Identifier("end".into())
        }
    }

    fn parse_condition(&mut self, expr_vec: Vec<Token>) -> Expression {
        Expression::Identifier("end".into())
    }

    fn collect_condition(&mut self) -> Vec<Token> {
        let mut expr_tokens = Vec::new();
        self.position += 1;
        while self.position < self.tokens.len() {

            //HACK: i dont quite understand this uh syntax but i know the whole point is to check
            //if the token is a symbol and run a condition on the aforementioned symbol

            if let Token::Symbol(c) = &self.tokens[self.position] {
                if *c == '{' {
                    break; // stop before block start
                }
            }
            expr_tokens.push(self.tokens[self.position].clone());
            self.position += 1;
        }
        // println!("{:?}", expr_tokens);
        expr_tokens
    }



//----------------------------------------------------------------------------
    // BLOCK
    fn collect_block(&mut self) -> Vec<Token> {
        let mut expr_tokens = Vec::new();
        self.position += 1;
        while self.position < self.tokens.len() {

            //HACK: same as collect condition

            if let Token::Symbol(c) = &self.tokens[self.position] {
                if *c == '}' {
                    break; // stop before block start
                }
            }
            expr_tokens.push(self.tokens[self.position].clone());
            self.position += 1;
        }
        // println!("{:?}", expr_tokens);
        expr_tokens
    }

    fn parse_block(&mut self, block: Vec<Token>) -> Vec<Statement> {
        let mut pos = 0;
        let mut statements = Vec::new();
        while pos < block.len() {
            let token = block[pos].clone();
            let stmt = self.parse_statement(token);
            statements.push(stmt);
            pos += 1;
        }
        statements
    }
//------------------------------------------------------------------------------






//--------------------------------------------------------------------------------
    // LOOP
    fn parse_loop(&mut self) -> Statement {
        self.position += 1;

        println!("parsing loop");
        let mut iterator = None;
        let mut range = None;

        // check if next token is an identifier → "loop i -> count"
        if let Some(Token::Identifier(name)) = self.tokens.get(self.position) {
            iterator = Some(name.clone());
            self.position += 1;

            let dash = "-".to_string();
            let greath = ">".to_string();

            // check for ->
            if let Some(Token::Operator(dash)) = self.tokens.get(self.position) {
                self.position += 1; // skip '-'
                if let Some(Token::Operator(greath)) = self.tokens.get(self.position) {
                    range = Some(self.collect_range());
                    println!("{:?}", range);
                }
            }
        }

        // now collect the body normally
        let block = self.collect_block();
        Statement::Loop {
            iterator: iterator,

            range: range,
            body: Box::new(Statement::Block(self.parse_block(block))),
        }
    }


    // figure out how to look through the vector of tokens that are gonna be the range
    // and decide if its a total range, a list of something, a tuple range (2, 8) or a string
    //
    fn collect_range(&mut self) -> LoopRange {
        let mut expr_tokens = Vec::new();
        self.position += 1;
        while self.position < self.tokens.len() {
            
            if let Token::Symbol(c) = &self.tokens[self.position] {
                if *c == '{' {
                    break; // stop before block start
                }
            }
            expr_tokens.push(self.tokens[self.position].clone());
            self.position += 1;
        }
        
        match expr_tokens.as_slice() {
            [Token::Symbol('('), Token::Number(a_str), Token::Symbol(','), Token::Number(b_str), Token::Symbol(')')] => {
                if let (Ok(a), Ok(b)) = (a_str.parse::<i64>(), b_str.parse::<i64>()) {
                    LoopRange::Range((a, b))
                } else {
                    LoopRange::Number(1) // fallback
                }
            }
            [Token::Number(n_str)] => {
                let n = n_str.parse::<i64>(); 
                LoopRange::Number(Result::expect(n, "error with loop range ?"))
            }
            _ => LoopRange::Number(1),
        }

    }

    fn get_iterator(&mut self) -> Option<String> {

        Some("placeholder".to_string())
    }
    //----------------------------------------------------------------------------------------------------

}

