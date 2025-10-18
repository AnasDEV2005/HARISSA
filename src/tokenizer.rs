#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(String),
    String(String),
    Boolean(bool),
    Operator(String),
    Symbol(char),
}


pub fn tokenize(raw: Vec<String>) -> Vec<Token> {

    let mut tokens = Vec::new();
    
    let mut previous = String::new();
    // to remind me to check previous token to determine if its an Identifier

    for element in raw.iter() {
        
        if previous == "'" || previous == "\"" {
            tokens.push(Token::String(element.to_string()));
            previous = element.to_string();
        } else {
            match element.as_str() {

                number if number.chars().all(|number| number.is_ascii_digit()) => { 
                    tokens.push(Token::Number(element.to_string()));
                    previous = element.to_string();
            },

                "const" | "while" | "for" | "where" | "if" | "else" | "elif" | "fn" | "pub" | "loop" => {
                    tokens.push(Token::Keyword(element.to_string()));
                    previous = element.to_string();
                },

                "=" | "+" | "-" | "/" | "*" | "%" | "|" | "&" | ">" | "<" => {
                    let c: char = element.chars().next().unwrap();
                    tokens.push(Token::Operator(c.to_string()));
                    previous = element.to_string();
                },

                "\"" | "'" | ":" | "?" | "[" | "]" | "{" | "}" | "(" | ")" | "#" | "," | "." | "!" | ";" | "\\" | "\n" => {
                    let c: char = element.chars().next().unwrap();
                    tokens.push(Token::Symbol(c));
                    previous = element.to_string();
                }

                "true" | "false" => {
                    let value = element == "true";
                    tokens.push(Token::Boolean(value));
                },

                string if (string.starts_with('"') && string.ends_with('"')) || (string.starts_with('\'') && string.ends_with('\'')) => {
                    let content = &string[1..string.len() - 1];
                    tokens.push(Token::String(content.to_string()));
                    previous = element.to_string();
                },



                _ => {
                    tokens.push(Token::Identifier(element.to_string()));
                    previous = element.to_string();
                }
            }
        }
    }
    tokens
} 



