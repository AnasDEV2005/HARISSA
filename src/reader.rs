use std::env;
use std::fs;

// NOTE add something that checks the file extension etc..

pub fn read_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Error reading tel file");

    let c = contents.to_string();
    c
}

pub fn parse_contents(contents: String) -> Vec<String> {
    let mut inside_string = false;
    let mut string_delimiter = '\0'; // track whether it's ' or "
    let mut parsed_contents = Vec::new();
    let mut word = String::new();

    for char in contents.chars() {

        if inside_string {

            word.push(char);
            if char == string_delimiter {
                parsed_contents.push(word.clone());
                word.clear();
                inside_string = false;
                string_delimiter = '\0';
            }
        } else {

            match char {

                '\'' | '"' => {
                    if (word != "") {
                        parsed_contents.push(word.clone());
                        word.clear();
                    }
                    inside_string = true;
                    string_delimiter = char;
                },

                ':' | '=' | '+' | '-' | '/' | '*' | '%' | '|' | '&' | '?' | '[' | ']' | '{' | '}' | '(' | ')' | '#' | '>' | '<' | ',' | '.' | '!' | ';' | '\\' | '\n'=> if (word == "") { 
                        parsed_contents.push(char.to_string())

                    } else {
                        parsed_contents.push(word.clone());
                        parsed_contents.push(char.to_string());
                        word.clear();
                },

                ' ' => {
                    parsed_contents.push(word.clone()); 
                    word.clear();
                },

                _ => {
                    word.push(char);
                }
                
            }
        }
// NOTE: operators...        
    }
    parsed_contents
}







