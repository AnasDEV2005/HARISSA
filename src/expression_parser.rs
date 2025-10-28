// to avoid bloating parser.rs

use crate::lexer::Token;
use crate::syntaxtree::{Expression, Statement};

pub fn collect_condition(tokens: &Vec<Token>, tok_position: &mut usize) -> Vec<Token> {
    let mut expr_tokens = Vec::new();
    *tok_position += 1;
    while *tok_position < tokens.len() {
        if let Token::OpenCurlyBracket(_) = tokens[*tok_position] {
            break; // stop before block end ?
        }
        expr_tokens.push(tokens[*tok_position].clone());
        *tok_position += 1;
    }
    // println!("{:?}", expr_tokens);
    expr_tokens
}

pub fn collect_block(tokens: &Vec<Token>, tok_position: &mut usize) -> (Vec<Token>, usize) {
    let mut block_tokens = Vec::new();
    *tok_position += 1;
    while *tok_position < tokens.len() {
        if let Token::CloseCurlyBracket(_) = tokens[*tok_position] {
            break; // stop before block end ?
        }
        block_tokens.push(tokens[*tok_position].clone());
        *tok_position += 1;
    }
    // println!("{:?}", expr_tokens);
    (block_tokens, *tok_position)
}

fn parse_expression(expr_tokens: &mut Vec<Token>) -> Box<Expression> {
    let mut pos = 0;

    let mut cond = Expression::New();

    // TODO: 1. Parsing of binary and unary expressions, to enable arithmetic calculations
    // TODO: 2. finish the expr parsing before going to other keywords.

    while pos < expr_tokens.len() {
        // println!("{:?}", expr_tokens[pos]);
        match &expr_tokens[pos] {
            Token::Number((num, _line)) => {
                // if its the last element in list, then only Number
                // else, idk
                println!("Number arm");
                if pos + 1 == expr_tokens.len() {
                    cond = Expression::Number(num.to_string());
                    break;
                }
            }
            Token::Identifier((string, _line)) => {
                return Box::new(Expression::Identifier(string.to_string()))
            }
            Token::BoolEqual(_line) => {}
            Token::CloseParenth(_line) => {}

            Token::OpenParenth(_line) => {
                /*
                          // NOTE:this will parse a (x, y) pattern where x and y are expressions
                          //   it is meant to parse a loop range.
                          //   But it will probably parse any other (x, y) couple.
                          //   I think its fine tbh we'll see if that gets in the way later

                          // i could possibly also just make "couple" a data type of its own
                */
                if expr_tokens[pos + 2] == Token::Comma(*_line)
                    && expr_tokens[pos + 4] == Token::CloseParenth(*_line)
                {
                    let mut left = [expr_tokens[pos + 1].clone()].to_vec();
                    let mut right = [expr_tokens[pos + 3].clone()].to_vec();
                    println!("LEFT: {:?}, RIGHT: {:?}", left, right);
                    return Box::new(Expression::Couple(
                        parse_expression(&mut left),
                        parse_expression(&mut right),
                    ));
                }
            }
            _ => {
                println!("_ arm");
            }
        }
        pos += 1;
    }
    Box::new(cond)
}

pub fn collect_range(tokens: &Vec<Token>, tok_position: &mut usize) -> (Expression, usize) {
    let mut expr_tokens = Vec::new();
    *tok_position += 1;
    while *tok_position < tokens.len() {
        if let Token::OpenCurlyBracket(_) = tokens[*tok_position] {
            break; // stop before block start
        }
        expr_tokens.push(tokens[*tok_position].clone());
        *tok_position += 1;
    }

    // let l = extract_linecount(expr_tokens[0].clone());

    // NOTE: check for correct range syntax and then parse it

    let parsed_expression = parse_expression(&mut expr_tokens);
    return (*parsed_expression, *tok_position);
}


// CONDITION
pub fn parse_condition(cond_tokens: &Vec<Token>) -> (Expression, usize) {
    let mut pos = 0;

    let mut expr = (
        Expression::InvalidExpression {
            message: ("expected Expression".to_string()),
            line: (0),
        },
        pos,
    );
    while pos < cond_tokens.len() {
        match &cond_tokens[pos] {
            Token::Number((n, l)) => {
                expr = (Expression::Number(n.clone().to_string()), pos);
            }
            _ => {
                (Expression::Identifier("expr".into()), pos);
            }
        }
        pos += 1;
    }
    expr
}







