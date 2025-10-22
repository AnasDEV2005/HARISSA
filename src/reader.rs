// use std::env;
use std::fs;

// NOTE: add something that checks the file extension etc..

pub fn read_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Error reading tel file");

    let c = contents.to_string();
    c
}

pub fn parse_contents(contents: String) -> Vec<String> {
    let mut inside_single_string = false;
    let mut inside_double_string = false;

    let mut slash = false;
    let mut ignore_until_n = false;

    let mut parsed_contents = Vec::new();
    let mut word = String::new();

    for char in contents.chars() {



            match char {
                '/' => {
                    if slash {
                        ignore_until_n = true;
                    } else {
                        slash = true;
                    };
                },


                '\'' => if !ignore_until_n {
                    if !inside_double_string {
                        if inside_single_string {

                            parsed_contents.push(word.clone());
                            word.clear();
                            parsed_contents.push(char.to_string());
                            inside_single_string = false;
                        } else {

                            word.clear();
                            parsed_contents.push("'".to_string());
                            inside_single_string = true;
                        }
                    } else {word.push(char);}
                },

                '"' => if !ignore_until_n { 
                    if !inside_single_string {
                        if inside_double_string {

                            parsed_contents.push(word.clone());
                            word.clear();
                            parsed_contents.push(char.to_string());
                            inside_double_string = false;
                        } else {

                            word.clear();
                            parsed_contents.push("\"".to_string());
                            inside_double_string = true;
                        }
                    } else {word.push(char);}
                },

                ':' | '=' | '+' | '-' | '*' | '%' | '|' | '&' | '?' | '[' | ']' | '{' | '}' | '(' | ')' | '#' | '>' | '<' | ',' | '.' | '!' | ';' | '\\' => if !ignore_until_n {
                    if inside_single_string || inside_double_string { 

                        word.push(char);
                    } else {

                        if !word.is_empty() { 

                            parsed_contents.push(word.clone()) 
                        };

                        parsed_contents.push(char.to_string());
                        word.clear();
                    }
                },

                ' ' | '\n' => {
                    // handle newlines inside strings
                    if (inside_single_string || inside_double_string) && char == '\n' {
                        // push current partial string first, so it’s not lost
                        if !word.is_empty() {
                            parsed_contents.push(word.clone());
                            word.clear();
                        }

                        // now push newline as explicit token
                        parsed_contents.push("\n".to_string());
                        continue;
                    }

                    // handle end of single-line comment
                    if char == '\n' && ignore_until_n {
                        ignore_until_n = false;
                        continue;
                    }

                    // outside strings/comments
                    if !ignore_until_n {
                        if !word.is_empty() {
                            parsed_contents.push(word.clone());
                            word.clear();
                        }

                        if char == '\n' {
                            parsed_contents.push("\n".to_string());
                        }
                    }
                }

                _ => if !ignore_until_n {
                    word.push(char);
                }
        }
    }
    parsed_contents
}







