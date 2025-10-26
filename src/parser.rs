//NOTE: so the idea is the following

/*

This file parses the Token stream, converting it into a syntax tree.

General idea:
* every function that modifies the position takes current position and outputs update position
* if it doesnt need to modify it, an immutable reference is passed
* if i need the position value mutably in a single scope, either return an updated position first,
   or returns an increment value meant to update the original position, while taking it as an immutable reference.

 */

use crate::expression_parser::{collect_block, collect_condition, collect_range};
use crate::lexer::Token;
use crate::syntaxtree::{Expression, Statement};

#[derive(Debug)]
pub enum ASToutput {
    Error(Statement),
    StatementVec(Vec<Statement>),
}

/*

parse()

 -> Takes Token stream and returns either an error statement or a syntax tree (vec of stmt)

*/

pub fn parse(token_stream: Vec<Token>) -> ASToutput {
    let mut statements = Vec::new();

    let mut tok_position = 0;

    let mut parsing_result: Statement = Statement::New();

    while tok_position < token_stream.len() {
        let token = &token_stream[tok_position.clone()];

        (parsing_result, tok_position) = parse_statement(&token_stream, token, &mut tok_position);

        match parsing_result {
            Statement::ParsingError { message: msg, line } => {
                return ASToutput::Error(Statement::ParsingError { message: msg, line })
            }

            _ => {
                statements.push(parsing_result);
                tok_position += 1;
            }
        }
    }

    ASToutput::StatementVec(statements)
}

/*

parse_statement()

 -> Takes a Token enum with its positon and returns Statement enum and a new position

*/

pub fn parse_statement(
    tokens: &Vec<Token>,
    token: &Token,
    tok_position: &mut usize,
) -> (Statement, usize) {
    match token {
        Token::Number((n, _)) => (Statement::New(), *tok_position),
        Token::String((s, _)) => (Statement::New(), *tok_position),
        Token::Keyword((k, _)) => parse_keyword(tokens, k, tok_position),
        Token::Identifier((ident, _)) => parse_identifier(ident, *tok_position),
        Token::Boolean((b, l)) => (Statement::New(), *tok_position),
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
            // println!("Line {line} | INVALID TOKEN: {}", msg);
            let stmt = Statement::ParsingError {
                message: msg.to_string().clone(),
                line: *line, // equivalent to line: line,
            };
            (stmt, *tok_position)
        }
        _ => (Statement::New(), *tok_position),
    }
}

/*

parse_keyword()

 -> Takes a Token enum with its positon and returns Statement enum and a new position

*/

fn parse_keyword(
    tokens: &Vec<Token>,
    keyword: &String,
    tok_position: &mut usize,
) -> (Statement, usize) {
    match keyword.as_str() {
        "if" => {
            let mut pos = tok_position.clone();
            let (condition, _) = parse_condition(&tokens);

            let (block, _) = collect_block(&tokens, &mut pos);

            let (block, _) = parse_block(block);

            let stmt = Statement::If {
                condition,
                then_branch: Box::new(Statement::Block(block)),
                else_branch: None,
            };
            (stmt, *tok_position)
        }

        // iterator: checks if there is an identifier after impl Trait for Type
        // range: either, length of the variable after the iterator, or the range specified
        "loop" => {
            let mut pos = tok_position.clone();
            parse_loop(tokens.to_vec(), &mut pos)
        }

        // placeholders
        "while" => {
            let mut pos = tok_position.clone();
            let condition_vec = collect_condition(&tokens, tok_position);
            let (condition, _) = parse_condition(&condition_vec);

            let (block, _) = collect_block(&tokens, &mut pos);

            let (block, _) = parse_block(block);

            let stmt = Statement::While {
                condition,
                body: Box::new(Statement::Block(block)),
            };
            (stmt, *tok_position)
        }

        "fn" => (Statement::New(), *tok_position),

        "const" => (Statement::New(), *tok_position),

        "elif" => (Statement::New(), *tok_position),

        "string" => (Statement::New(), *tok_position),

        "int" => (Statement::New(), *tok_position),

        "list" => (Statement::New(), *tok_position),

        "return" => (Statement::New(), *tok_position),

        "object" => (Statement::New(), *tok_position),

        "variant" => (Statement::New(), *tok_position),

        _ => (Statement::New(), *tok_position),
    }
}

fn parse_symbol(_chr: char, _pos: usize) -> Statement {
    Statement::Expression(Expression::Identifier("symbol".into()))
}

fn parse_identifier(identifier: &String, pos: usize) -> (Statement, usize) {
    (Statement::New(), pos)
}

fn parse_operator(_operator: String, _pos: usize) -> Statement {
    Statement::Expression(Expression::Identifier("operator".into()))
}



// BLOCK

fn parse_block(block: Vec<Token>) -> (Vec<Statement>, usize) {
    let mut pos = 0;
    let mut statements = Vec::new();
    while pos < block.len() {
        let token = block[pos].clone();
        let (stmt, _) = parse_statement(&block, &token, &mut pos);
        statements.push(stmt);
        pos += 1;
    }
    (statements, pos)
}
// ------------------------------------------------------------------------------

//--------------------------------------------------------------------------------
// LOOP
// HACK: this function was borrow checker hell... i hope not to have to touch it again

fn parse_loop(tokens: Vec<Token>, tok_position: &mut usize) -> (Statement, usize) {
    *tok_position += 1;

    let mut iterator = None;
    let mut range = Expression::InvalidExpression {
        message: ("Initialized expression".to_string()),
        line: (0),
    };

    // this checks if the next token is an ident (like i for iterator)
    // if so it collects the range
    let current_token = tokens.get(*tok_position);

    if let Some(Token::Identifier((name, n))) = current_token {
        iterator = Some((name.clone(), n));

        *tok_position += 1;
        // check for ->
        let token = tokens[*tok_position].clone();
        match token {
            Token::Keyword((k, _l)) if k == "->".to_string() => {
                (range, *tok_position) = collect_range(&tokens, &mut tok_position.clone());
            }
            Token::OpenCurlyBracket(_l) => {
                eprintln!("Syntax error for parse_loop, no range after ->");
            }
            _ => eprintln!("Error in function: parse_loop"),
        }
    }

    // now collect the body normally
    let (block, _) = collect_block(&tokens, tok_position);

    match iterator {
        Some((i, _l)) => {
            match range {
                Expression::InvalidExpression { message, line } => {
                    let stmt = Statement::ParsingError { message, line };
                    return (stmt, 0);
                }
                _ => {
                    // println!("loop line: {l}");

                    let (code_block, _) = parse_block(block);
                    let stmt = Statement::Loop {
                        iterator: Some(i),
                        range,
                        body: Box::new(Statement::Block(code_block)),
                    };
                    return (stmt, *tok_position);
                }
            }
        }

        None => {
            let stmt = Statement::ParsingError {
                message: "".to_string(),
                line: 0, // or use something else appropriate
            };
            return (stmt, 0);
        }
    }
}

// RANGE IS AN EXPRESSION

/*
fn get_iterator(&mut self) -> Option<String> {

    Some("placeholder".to_string())
}
*/

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
        | Token::BoolEqual(l)
        | Token::AssignEqual(l)
        | Token::OrBool(l)
        | Token::DblOrBool(l)
        | Token::AndBool(l)
        | Token::DblAndBool(l) => return l,

        _ => return 0, // has no line number field, default to 0 (or whatever fits)
    };
}
