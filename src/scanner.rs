// src/scanner.rs

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Number(i64),
    StringLiteral(String),
    Symbol(char),
    Newline,
    Unknown(char),
}

/// Splits on whitespace, quotes, punctuation, and newlines.
/// This is just a placeholder; expand it later.
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for line in input.lines() {
        // Emit one Newline token per source line
        let mut chars = line.chars().peekable();
        while let Some(&ch) = chars.peek() {
            match ch {
                '"' => {
                    chars.next();
                    let mut literal = String::new();
                    while let Some(&c2) = chars.peek() {
                        if c2 == '"' {
                            chars.next();
                            break;
                        } else {
                            literal.push(c2);
                            chars.next();
                        }
                    }
                    tokens.push(Token::StringLiteral(literal));
                }
                c if c.is_ascii_digit() => {
                    let mut num = String::new();
                    while let Some(&d) = chars.peek() {
                        if d.is_ascii_digit() {
                            num.push(d);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    let val = num.parse().unwrap_or(0);
                    tokens.push(Token::Number(val));
                }
                c if c.is_ascii_alphabetic() || c == '_' => {
                    let mut ident = String::new();
                    while let Some(&c2) = chars.peek() {
                        if c2.is_ascii_alphanumeric() || c2 == '_' {
                            ident.push(c2);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Identifier(ident));
                }
                c if c.is_whitespace() => {
                    chars.next(); // skip
                }
                ';' | '{' | '}' | '(' | ')' | ',' | ':' | '+' | '-' | '*' | '/' | '=' => {
                    tokens.push(Token::Symbol(ch));
                    chars.next();
                }
                other => {
                    tokens.push(Token::Unknown(other));
                    chars.next();
                }
            }
        }
        tokens.push(Token::Newline);
    }
    tokens
}

