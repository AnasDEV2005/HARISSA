use crate::reader::TokenPos;
use crate::errors::SyntaxErrors;


#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(String),
    String(String),
    Boolean(bool),
    Operator(String),
    Symbol(char),
    WithPos(Box<Token>, usize, usize), // (actual token, line, column)
}


pub fn tokenize(parsed: Vec<TokenPos>) -> Vec<Token> {
    let mut tokens = Vec::new();

    for item in parsed {
        let token_value = item.value.as_str();

        let token = match token_value {
            "if" | "else" | "while" | "loop" | "const" | "fn" | "return" | "true" | "false" => {
                Token::Keyword(token_value.to_string())
            }

            "=" | "+" | "-" | "*" | "/" | "%" | ">" | "<" | "==" | "!=" | ">=" | "<=" => {
                Token::Operator(token_value.to_string())
            }

            "{" | "}" | "(" | ")" | "[" | "]" | ";" | "," | ":" | "." => {
                Token::Symbol(token_value.chars().next().unwrap())
            }

            _ if token_value.chars().all(|c| c.is_ascii_digit()) => {
                Token::Number(token_value.to_string())
            }

            _ if token_value.starts_with('"') && token_value.ends_with('"') => {
                Token::String(token_value.to_string())
            }

            _ => Token::Identifier(token_value.to_string()),
        };

        tokens.push(Token::WithPos(Box::new(token), item.line, item.column));
    }

    tokens
}
 


pub fn validate_tokens(tokens: &[Token]) -> Result<(), SyntaxErrors> {
    for i in 0..tokens.len() {
        if let Token::Symbol('\n') = tokens[i] {
            // look at the previous token
            if let Some(prev) = tokens.get(i.wrapping_sub(1)) {
                // if previous isn't ';' or '}', that’s an error
                match prev {
                    Token::Symbol(';') | Token::Symbol('}') => (),
                    _ => return Err(SyntaxErrors::MissingSEMICOLON("Expected ';' before newline".into())),
                }
            }
        }
    }

    // also check for unclosed blocks
    let open_braces = tokens.iter().filter(|t| matches!(t, Token::Symbol('{'))).count();
    let close_braces = tokens.iter().filter(|t| matches!(t, Token::Symbol('}'))).count();

    if open_braces != close_braces {
        return Err(SyntaxErrors::MissingCURLYBRACKET("Mismatched curly braces".into()));
    }

    Ok(())
}




