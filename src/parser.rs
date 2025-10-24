// other parser is a mess and a headache
// i wanna redo a somewhat clean version
//
//
//NOTE: so the idea is the following

/* 
 
1. the iterator becomes independant from the parser object
2. RAAAH NEVER MIND using a struct for this is utterly stupid
3. just more borrow checker headache for no reason what the heck bro
4. never let chatgpt cook again



 */

use crate::syntaxtree::{Statement, Expression, LoopRange};
use crate::lexer::Token;



#[derive(Debug)]
pub enum ASToutput {
    Error(Statement::ParsingError),
    StatementVec(Vec<Statement>),
}



/* 

parse()

 -> Takes Token stream and returns either an error statement or a syntax tree (vec of stmt)

*/


pub fn parse(token_stream: Vec<Token>) -> ASToutput {

    let mut statements = Vec::new();

    let mut statment_position = ( Statement::PlaceHolder() , 0);

    while tok_position < token_stream.len() {

        tok_position = statement_position[1];

        let token = token_stream[tok_position];
    
        let mut stmt = parse_statement(token, tok_position);

        match statement_position[0] { 
            Statement::SyntaxError(msg, line) => {
                return  Statement::SyntaxError(msg, line)
                },

            _ => {
                statements.push(statement_position[0]);
                tok_position += 1;
            }
        }


    }

    ASToutput(statements)
}


/* 

parse_statement() 

 -> Takes a Token enum with its positon and returns Statement enum and a new position

 */


pub fn parse_statement(token: Token, tok_position: i32) -> (Statement, i32) {
    match token {
        Token::Number((n, _)) => Statement::Expression(Expression::Number(n)),
        Token::String((s, _)) => Statement::Expression(Expression::String(s)),
        Token::Keyword((k, _)) => self.parse_keyword(k, self.position),
        Token::Symbol(chr, l) => self.parse_symbol(chr, self.position),
        Token::Identifier((ident, _)) => self.parse_identifier(ident, self.position),
        Token::Boolean((b, l)) => Statement::PlaceHolder {  },
        // Token::Comma() => ,
        // Token::Semicolon() => ,
        // Token::OpenParenth() => ,
        // Token::CloseParenth() => ,
        // Token::OpenCurlyBracket() => ,
        // Token::CloseCurlyBracket() => ,
        // Token::OpenSquareBracket() => ,
        // Token::CloseSquareBracket() => ,
        // Token::EndOfFile(i32) => ,
        // Token::AssignIncrement() => ,
        // Token::AssignDecrement() => ,
        // Token::PowerOperator() => ,
        // Token::PlusOperator() => ,
        // Token::MinusOperator() => ,
        // Token::MultiplicationOperator() => ,
        // Token::DivisionOperator() => ,
        // Token::ModuloOperator() => ,
        // Token::AssignBool() => ,
        // Token::AssignEqual() => ,
        // Token::OrBool() => ,
        // Token::DblOrBool() => ,
        // Token::AndBool() => ,
        // Token::DblAndBool() => ,
        // Token::Symbol(char) => ,

        Token::InvalidToken((msg, line)) => {
            println!("Line {line} | INVALID TOKEN: {}", msg);
            let stmt = Statement::ParsingError {
                message: msg,
                line, // equivalent to line: line,
            };
        },
        _ => {
            Statement::PlaceHolder {  }
        },
    }
}




fn parse_keyword(keyword: String, _pos: usize) -> Statement {
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


        // placeholders
        
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

        "variant" => Statement::Expression(Expression::Identifier("variant".into())),

        _ => Statement::Expression(Expression::Identifier("unknown".into())),
    }
}





fn parse_symbol(_chr: char, _pos: usize) -> Statement {
    Statement::Expression(Expression::Identifier("symbol".into()))
}

fn parse_identifier(_identifier: String, _pos: usize) -> Statement {
    Statement::Expression(Expression::Identifier("ident".into()))
}

fn parse_operator(_operator: String, _pos: usize) -> Statement {
    Statement::Expression(Expression::Identifier("operator".into()))
}


fn parse_expression(tokens: Vec<Token>, position: i32) -> Expression {
    position += 1;
    if sposition < tokens.len() {
        match tokens[position] {
            Token::Number((n, l)) => Expression::Number(n.clone().to_string()),
            _ => Expression::Identifier("expr".into()),
        }
    } else {
        Expression::Identifier("end".into())
    }
}

fn parse_condition(expr_vec: Vec<Token>) -> Expression {
    Expression::Identifier("end".into())
}

fn collect_condition() -> Vec<Token> {
    let mut expr_tokens = Vec::new();
    self.position += 1;
    while self.position < self.tokens.len() {

        expr_tokens.push(self.tokens[self.position].clone());
        self.position += 1;
    }
    // println!("{:?}", expr_tokens);
    expr_tokens
}



// FIXME: Still havent worked on this

//----------------------------------------------------------------------------
// BLOCK
fn collect_block() -> Vec<Token> {
    let mut expr_tokens = Vec::new();
    self.position += 1;
    while self.position < self.tokens.len() {

        //HACK: same as collect condition

        if let Token::Symbol(c, l) = &self.tokens[self.position] {
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

fn parse_block(block: Vec<Token>) -> Vec<Statement> {
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
// ------------------------------------------------------------------------------





//--------------------------------------------------------------------------------
// LOOP
// HACK: this function was borrow checker hell... i hope not to have to touch it again

fn parse_loop() -> Statement {
    self.position += 1;

    let mut iterator = None;
    let mut range = None;

    // this checks if the next token is an ident (like i for iterator)
    // if so it collects the range
    let current_token = self.tokens.get(self.position).cloned();


    if let Some(Token::Identifier((name, n))) = current_token {
        iterator = Some((name.clone(), n));

        self.position += 1;
        // check for ->
        let token = self.tokens[self.position].clone();
        match token {
            Token::Keyword((k, _l)) if k == "->".to_string() => {
                println!("OK ONCE");
                range = Some(self.collect_range());
                // self.collect_range collects range whether its (x, y) or just x
                println!("{0}", self.position);
            }
            Token::OpenCurlyBracket(_l) => {
                eprintln!("Syntax error for parse_loop, no range after ->");
            }
            _ => eprintln!("Error in function: parse_loop"),
        }
    }

    // now collect the body normally
    let block = self.collect_block();

    match iterator {
        Some((i, l)) => {
            match range {
                Some(LoopRange::InvalidRange(_)) => {
                    return Statement::SyntaxError {
                        message: "Invalid loop range".to_string(),
                        line: l,  // FIXME: Find a way to report the line number of this thing
                    }
                }
                _ => {
                    println!("{l}");
                    return Statement::Loop {
                        iterator: Some(i),
                        range,
                        body: Box::new(Statement::Block(self.parse_block(block))),
                    }
                }
            }
        }

        None => return Statement::SyntaxError {
            message: "Invalid loop range".to_string(),
            line: 0, // or use something else appropriate
        },
    }
}


// figure out how to look through the vector of tokens that are gonna be the range
// and decide if its a total range, a list of something, a tuple range (2, 8) or a string
//
fn collect_range(tokens: Vec<Token>, position: i32) -> LoopRange {
    let mut expr_tokens = Vec::new();
    position += 1;
    while position < tokens.len() {
        
        if let Token::OpenCurlyBracket(_) = tokens[position] {
            break; // stop before block start
        }
        expr_tokens.push(tokens[position].clone());
        position += 1;
    }

    // let l = extract_linecount(expr_tokens[0].clone());

    
    // NOTE: check for correct range syntax and then parse it
    match expr_tokens.as_slice() {
        [Token::OpenParenth(l1), Token::Number((a_str, _)), Token::Comma(l2), Token::Number((b_str, _)), Token::CloseParenth(l3)] => {
            if let (Ok(a), Ok(b)) = (a_str.parse::<i32>(), b_str.parse::<i32>()) {
                LoopRange::Range((a, b))
            } else {
                LoopRange::Number(1) // fallback
            }
        }
        [Token::Number((n_str, _))] => {
            let n = n_str.parse::<i32>(); 
            LoopRange::Number(Result::expect(Err(n), "error with loop range ?"))
        }
        _ => LoopRange::InvalidRange(l)
        
    }

}

fn get_iterator(&mut self) -> Option<String> {

    Some("placeholder".to_string())
}
//----------------------------------------------------------------------------------------------------

fn extract_linecount(token: Token) -> i32 {

    let line = match token {
        Token::Keyword((_, l))
        | Token::Identifier((_, l))
        | Token::Number((_, l))
        | Token::String((_, l))
        | Token::Boolean((_, l))
        | Token::InvalidToken((_, l)) 
        | Token::Symbol(_, l) => return l,

        Token::Comma(l)
        | Token::Semicolon(l)
        | Token::OpenParenth(l)
        | Token::CloseParenth(l)
        | Token::OpenCurlyBracket(l)
        | Token::CloseCurlyBracket(l)
        | Token::OpenSquareBracket(l)
        | Token::CloseSquareBracket(l)
        | Token::EndOfFile(l)
        | Token::AssignIncrement(l)
        | Token::AssignDecrement(l)
        | Token::PowerOperator(l)
        | Token::PlusOperator(l)
        | Token::MinusOperator(l)
        | Token::MultiplicationOperator(l)
        | Token::DivisionOperator(l)
        | Token::ModuloOperator(l)
        | Token::AssignBool(l)
        | Token::AssignEqual(l)
        | Token::OrBool(l)
        | Token::DblOrBool(l)
        | Token::AndBool(l)
        | Token::DblAndBool(l) => return l,

        _ => return 0, // has no line number field, default to 0 (or whatever fits)
    };
}














