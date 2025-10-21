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
        let ch = chars[i];
        column += 1;

        // track newlines
        if ch == '\n' {
            line += 1;
            column = 0;
        }

        // handle block comments
        if inside_block_comment {
            if ch == '*' && i + 1 < chars.len() && chars[i + 1] == '/' {
                inside_block_comment = false;
                i += 1;
            }
            i += 1;
            continue;
        }

        //=============================
        // COMMENT HANDLING
        //=============================
        if !inside_single_string && !inside_double_string {
            if ch == '/' {
                if slash {
                    ignore_until_n = true;
                    slash = false;
                } else if i + 1 < chars.len() && chars[i + 1] == '*' {
                    inside_block_comment = true;
                    i += 1;
                } else {
                    slash = true;
                }
                i += 1;
                continue;
            } else {
                slash = false;
            }
        }

        //=============================
        // STRING HANDLING (quotes INCLUDED)
        //=============================
        match ch {
            '"' => {
                if !ignore_until_n {
                    word.push(ch); // always include the quote itself
                    if inside_double_string {
                        // closing double quote
                        push_token(&mut parsed_contents, &word, line, column);
                        word.clear();
                        inside_double_string = false;
                    } else if !inside_single_string {
                        // opening double quote
                        inside_double_string = true;
                    }
                }
            }

            '\'' => {
                if !ignore_until_n {
                    word.push(ch); // include the quote itself
                    if inside_single_string {
                        // closing single quote
                        push_token(&mut parsed_contents, &word, line, column);
                        word.clear();
                        inside_single_string = false;
                    } else if !inside_double_string {
                        // opening single quote
                        inside_single_string = true;
                    }
                }
            }

            ':' | '=' | '+' | '-' | '*' | '%' | '|' | '&' | '?' | '[' | ']' | '{' | '}' | '(' | ')' | '#' | '>' | '<' | ',' | '.' | '!' | ';' | '\\'
                if !ignore_until_n =>
            {
                if inside_single_string || inside_double_string {
                    word.push(ch);
                } else {
                    if !word.is_empty() {
                        push_token(&mut parsed_contents, &word, line, column);
                        word.clear();
                    }
                    push_token(&mut parsed_contents, &ch.to_string(), line, column);
                }
            }

            ' ' | '\n' => {
                if ch == '\n' && ignore_until_n {
                    ignore_until_n = false;
                }

                if ch == '\n' && !inside_single_string && !inside_double_string {
                    if !word.is_empty() {
                        push_token(&mut parsed_contents, &word.clone().as_str(), line, column);
                        word.clear();
                    }
                    push_token(&mut parsed_contents, "\\n", line, column);
                }

                if !ignore_until_n {
                    if inside_single_string || inside_double_string {
                        word.push(ch);
                    } else if !word.is_empty() {
                        push_token(&mut parsed_contents, &word, line, column);
                        word.clear();
                    }
                }
            }

            _ => {
                if inside_single_string || inside_double_string {
                    word.push(ch);
                } else if !ignore_until_n {
                    word.push(ch);
                }
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

