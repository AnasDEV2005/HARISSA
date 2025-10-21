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
 


pub fn validate_tokens(tokens: Vec<Token>) -> Result<(), SyntaxErrors> {
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::String(_) => {

                //NOTE: this says "if this STRING is the last token, then something is wrong"
                if i + 1 >= tokens.len() {
                    return Err(SyntaxErrors::MissingSEMICOLON(
                        "expected ) or , or ; or }... found something".to_string(),
                    ));
                }

// if it isnt, i check if the next token is valid
                match &tokens[i + 1] {
                    Token::Keyword(_) | Token::Identifier(_) => {
                        return Err(SyntaxErrors::MissingSEMICOLON(
                            "expected ) or , or ; or }... found something".to_string(),
                        ));
                    }
                    _ => {}
                }
            }

            Token::Identifier(identifier) => {

                //NOTE: this says "if this IDENTIFIER is the last token, then something is wrong"
                if i + 1 >= tokens.len() {
                    return Err(SyntaxErrors::MissingSEMICOLON(
                        "expected ) or , or ; or }... found something".to_string(),
                    ));
                }

// if it isnt, i check if the next token is valid
                if identifier.contains('\n') {
                    return Err(SyntaxErrors::MissingSEMICOLON(
                        "identifier contains newline".to_string(),
                    ));
                }
            }

            _ => {}
        }

        i += 1; // ✅ only once per loop
    }

    Ok(())
}




