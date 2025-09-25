// use std::env;
use std::fs;

// NOTE add something that checks the file extension etc..

pub fn read_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Error reading tel file");

    let c = contents.to_string();
    c
}


pub fn parse_contents(contents: String) -> Vec<String> {
    let mut inside_single_string = false;
    let mut inside_double_string = false;

    let mut parsed_contents = Vec::new();
    let mut word = String::new();

    for char in contents.chars() {



            match char {

                '\'' => if !inside_double_string {
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
                } else {word.push(char);},
                '"' => if !inside_single_string {
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
                } else {word.push(char);},

                ':' | '=' | '+' | '-' | '/' | '*' | '%' | '|' | '&' | '?' | '[' | ']' | '{' | '}' | '(' | ')' | '#' | '>' | '<' | ',' | '.' | '!' | ';' | '\\' | '\n'=> if inside_single_string || inside_double_string { 

                        word.push(char);
                    } else {

                        if !word.is_empty() { 

                            parsed_contents.push(word.clone()) 
                        };

                        parsed_contents.push(char.to_string());
                        word.clear();
                },

                ' ' => {
                    if inside_single_string || inside_double_string {

                        word.push(char); 
                    } else {
                        if !word.is_empty() {
                            parsed_contents.push(word.clone());
                            word.clear();
                        }
                    }
                },

                _ => {
                    word.push(char);
                }
        }
    }
    parsed_contents
}







