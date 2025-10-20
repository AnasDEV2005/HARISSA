
use std::fs;

pub fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Error reading tel file")
}

#[derive(Debug, Clone)]
pub struct TokenPos {
    pub value: String,
    pub line: usize,
    pub column: usize,
}

pub fn parse_contents(contents: String) -> Vec<TokenPos> {
    let mut inside_single_string = false;
    let mut inside_double_string = false;

    let mut slash = false;
    let mut ignore_until_n = false;
    let mut inside_block_comment = false;

    let mut parsed_contents = Vec::new();
    let mut word = String::new();

    let mut line: usize = 1;
    let mut column: usize = 0;

    let chars: Vec<char> = contents.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let char = chars[i];
        column += 1;

        // track newlines globally
        if char == '\n' {
            line += 1;
            column = 0;
        }

        // handle block comments
        if inside_block_comment {
            if char == '*' && i + 1 < chars.len() && chars[i + 1] == '/' {
                inside_block_comment = false;
                i += 1; // skip '/'
            }
            i += 1;
            continue;
        }

        match char {
            '/' => {
                if slash {
                    ignore_until_n = true; // start line comment //
                    slash = false;
                } else if i + 1 < chars.len() && chars[i + 1] == '*' {
                    inside_block_comment = true; // start block comment /*
                    i += 1; // skip '*'
                } else {
                    slash = true;
                }
            }

            '\'' => if !ignore_until_n {
                if !inside_double_string {
                    if inside_single_string {
                        push_token(&mut parsed_contents, &word, line, column);
                        word.clear();
                        push_token(&mut parsed_contents, &"'".to_string(), line, column);
                        inside_single_string = false;
                    } else {
                        word.clear();
                        push_token(&mut parsed_contents, &"'".to_string(), line, column);
                        inside_single_string = true;
                    }
                } else {
                    word.push(char);
                }
            }

            '"' => if !ignore_until_n {
                if !inside_single_string {
                    if inside_double_string {
                        push_token(&mut parsed_contents, &word, line, column);
                        word.clear();
                        push_token(&mut parsed_contents, &"\"".to_string(), line, column);
                        inside_double_string = false;
                    } else {
                        word.clear();
                        push_token(&mut parsed_contents, &"\"".to_string(), line, column);
                        inside_double_string = true;
                    }
                } else {
                    word.push(char);
                }
            }

            ':' | '=' | '+' | '-' | '*' | '%' | '|' | '&' | '?' | '[' | ']' | '{' | '}' | '(' | ')' | '#' | '>' | '<' | ',' | '.' | '!' | ';' | '\\' => if !ignore_until_n {
                if inside_single_string || inside_double_string {
                    word.push(char);
                } else {
                    if !word.is_empty() {
                        push_token(&mut parsed_contents, &word, line, column);
                    }
                    push_token(&mut parsed_contents, &char.to_string(), line, column);
                    word.clear();
                }
            }

            ' ' | '\n' => {
                if char == '\n' && ignore_until_n {
                    ignore_until_n = false;
                }
                if !ignore_until_n {
                    if inside_single_string || inside_double_string {
                        word.push(char);
                    } else if !word.is_empty() {
                        push_token(&mut parsed_contents, &word, line, column);
                        word.clear();
                    }
                }
            }

            _ => if !ignore_until_n {
                word.push(char);
            }
        }

        i += 1;
    }

    parsed_contents
}

fn push_token(parsed: &mut Vec<TokenPos>, word: &str, line: usize, column: usize) {
    if !word.trim().is_empty() {
        parsed.push(TokenPos {
            value: word.trim().to_string(),
            line,
            column,
        });
    }
}

