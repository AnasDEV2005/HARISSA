// gonna refactor reader and tokenizer into this file, soon enough



// Tasks:

// Read file into a string 
// Iterate through the string 
// slice by whitespace
// OR newline, but also add new line to token stream
// when inside string, if find a \n byte or \0 byte, return a invalid token
// check if there is an invalid token before `;` and operators
// check if there is an invalid token before keywords (aka another keyword)


use std::fs;

// NOTE: add something that checks the file extension etc..


pub fn read_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Error reading tel file");

    let raw_code = contents.to_string();
    raw_code
}


#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(String),
    String(String),
    Boolean(bool),
    Operator(char),
    Comma(),
    Semicolon(),
    OpenParenth(),
    CloseParenth(),
    OpenCurlyBracket(),
    CloseCurlyBracket(),
    OpenSquareBracket(),
    CloseSquareBracket(),
    EndOfLine(),
    Symbol(char),
    InvalidToken(String)
}


pub fn tokenize(contents: String) -> Vec<Token> {

    // HACK: a half chatgpt-written macro

    macro_rules! next_char {
        ($chars:expr) => {
            match $chars.next() {
                Some(c) => Some(c),
                None => None,
            }
        }
    }

    let mut debug = false;
    let mut linecount = 0;
    let mut tokens = Vec::new();
    let mut chars = contents.chars();
    let mut keyword_or_ident = String::new();

    loop {

        let c = next_char!(chars);

        if c == None { break; } // END OF FILE 

        let character = c.unwrap();
         

        match character {

            ' ' => {
                if !keyword_or_ident.is_empty() {
                    tokens.push(unwrap_word(&keyword_or_ident));
                    keyword_or_ident.clear();
                }
            }

            '\n' => {
                linecount += 1;
                if !keyword_or_ident.is_empty() {
                    tokens.push(Token::InvalidToken(keyword_or_ident));
                    break;
                }
                tokens.push(Token::EndOfLine());
            }

            ';' => {
                if !keyword_or_ident.is_empty() {
                    tokens.push(unwrap_word(&keyword_or_ident));
                    keyword_or_ident.clear();
                }
                tokens.push(Token::Semicolon());
            }

            ',' => {
                if !keyword_or_ident.is_empty() {
                    tokens.push(unwrap_word(&keyword_or_ident));
                    keyword_or_ident.clear();
                }
                tokens.push(Token::Comma());
            }

            '{' | '}' => {
                if !keyword_or_ident.is_empty() {
                    tokens.push(unwrap_word(&keyword_or_ident));
                    keyword_or_ident.clear();
                }
                if character == '{' {tokens.push(Token::OpenCurlyBracket());} else {tokens.push(Token::CloseCurlyBracket());}
            }

            '(' | ')' => {
                if !keyword_or_ident.is_empty() {
                    tokens.push(unwrap_word(&keyword_or_ident));
                    keyword_or_ident.clear();
                }
                if character == '(' {tokens.push(Token::OpenParenth());} else {tokens.push(Token::CloseParenth());}
            }

            '[' | ']' => {
                if !keyword_or_ident.is_empty() {
                    tokens.push(unwrap_word(&keyword_or_ident));
                    keyword_or_ident.clear();
                }
                if character == '[' {tokens.push(Token::OpenSquareBracket());} else {tokens.push(Token::CloseSquareBracket());}
            }


            '-' | '+' | '*' | '%' => {
                if !keyword_or_ident.is_empty() {
                    tokens.push(unwrap_word(&keyword_or_ident));
                    keyword_or_ident.clear();
                }

                match character {
                    '-' => {
                        
                        let next = next_char!(chars);
                        match next {
                            None | Some('\n')  | Some('-') | Some('+') | Some('*') | Some('%') | Some('=') => {
                                tokens.push(Token::InvalidToken(character.to_string()));
                                break;
                            },
                            Some('>') => {
                                tokens.push(Token::Keyword("->".to_string()));
                                // add the "->" token as a keyword possibly
                            },
                            _ => {
                                tokens.push(Token::Operator('-'));
                            },
                        }
                    }



                    // TODO: deal with next token being invalid
                    '+' => {},
                    '*' => {},
                    '%' => {},
                    _ => {}, // i dont get why this is non exaustive but oke
                }
                // tokens.push(Token::Operator(character));
            }

 

            '/' => {
                if !keyword_or_ident.is_empty() {
                    tokens.push(unwrap_word(&keyword_or_ident));
                    keyword_or_ident.clear();
                }
                let mut next = next_char!(chars);
                match next {
                    None | Some('\n') => {
                        tokens.push(Token::InvalidToken(character.to_string()));
                        break;
                    },
                    Some('/') => {
                        while next != Some('\n') {
                            next = next_char!(chars);
                        }
                        tokens.push(Token::EndOfLine());
                    }, // comment line ignore
                       
                    Some('*') => {
                        while next != Some('*') {
                            if next == Some('\n') {tokens.push(Token::EndOfLine()); }
                            next = next_char!(chars);
                        }
                        next = next_char!(chars);
                        if next != Some('/') {
                            tokens.push(Token::InvalidToken(character.to_string()));
                            break;
                        }
                    },           // comment block ignore

                    _ => tokens.push(Token::Operator(character)),

                }



            }

            '"' | '\'' => {
                if !keyword_or_ident.is_empty() {
                    tokens.push(unwrap_word(&keyword_or_ident));
                    keyword_or_ident.clear();
                }

                let mut string_literal = String::new();
                loop {
                    let ch = next_char!(chars);
                    if ch == None || ch == Some('\n') {
                        tokens.push(Token::InvalidToken(character.to_string()));
                        break;
                    }

                    if ch == Some(character) {
                        tokens.push(Token::String(string_literal));
                        break;
                    }
                    string_literal.push(ch.expect("Failed string UNWRAP at `\"` and `'` arm "));
                }
            }

// TODO: IF NUMBER WHILE keyword_or_ident EMPTY AND NEXT CHARACT IS NOT A NUMBER THEN BREAK AND
// SEND INVALID TOKEN

            ch => {

                if keyword_or_ident.is_empty() {
                    // check if ch is not a number, else return invalid token
                    if ch.is_digit(10) {
                        let mut number: String = ch.to_string() ;
                        loop {
                            let next = next_char!(chars);
                            if next == Some(' ') {
                                tokens.push(Token::Number(number.to_string()));
                                break;
                            }
                            if next == None || next == Some('\n') {
                                tokens.push(Token::InvalidToken(character.to_string()));
                                break;
                            } 
                            if next == Some(';') || next == Some(')') || next == Some(']') || next == Some(',') {
                                tokens.push(Token::Number(number.to_string()));
                                match next {
                                    Some(';') => tokens.push(Token::Semicolon()),
                                    Some(')') => tokens.push(Token::CloseParenth()),
                                    Some(']') => tokens.push(Token::CloseSquareBracket()),
                                    Some(',') => tokens.push(Token::Comma()),
                                    _ => println!("CASE ALREADY COVERED"),
                                }
                                break;
                            }
                            if !next.expect("Unwrapping ERROR lexer.rs at _ arm").is_digit(10) {
                                tokens.push(Token::InvalidToken(character.to_string()));
                                break;
                            }
                            number.push(next.expect("Unwrapping ERROR lexer.rs at _ arm"));
                        }
                    }
                } else {
                keyword_or_ident.push(ch);
                }
            }
        }
    }

    tokens

}


fn unwrap_word(word: &String) -> Token {
    match word.as_str() {
        "const" | "while" | "for" | "loop" | "if" | "else" | "fn" | "pub" | "string" | "int" | "uint" | "object" | "list" 
            | "variant" | "tuple" => Token::Keyword(word.to_string()), // i might make a token for every keyword idk
        _ => Token::Identifier(word.to_string()),
    }
}







