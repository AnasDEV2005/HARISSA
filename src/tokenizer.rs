#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(String),
    String(String),
    Boolean(bool),
    Operator(String),
    Symbol(char),
    InvalidToken(String)
}



pub fn tokenize(raw: Vec<String>) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut inside_string = false;
    let mut current_string = String::new();
    let mut quote_char = '\0';
    let mut i = 0;

    for element in raw.iter() {
        let s = element.as_str();

        print!("{}", s);
        // if we’re inside a string, accumulate until closing quote
        if inside_string {
            println!("{}", current_string);
            if element == "\n" || element == "\0" {
                println!("DEBBUG");
                tokens.push(Token::InvalidToken(current_string.clone()));
                break;
            }
            if s == quote_char.to_string() {
                // closing quote
                tokens.push(Token::String(current_string.clone()));
                current_string.clear();
                inside_string = false;
                continue;
            } else {
                // still inside string
                println!("{}", current_string);
                current_string.push_str(s);
                continue;
            }
        }

        // opening a string
        if s == "\"" || s == "'" {
            inside_string = true;
            quote_char = s.chars().next().unwrap();
            current_string.clear();
            continue;
        }

        // normal tokenization
        match s {
            "const" | "while" | "for" | "loop" | "if" | "else" | "fn" | "pub" |
            "string" | "int" | "object" | "list" | "enum" => tokens.push(Token::Keyword(s.to_string())),

            "=" | "+" | "-" | "/" | "*" | "%" | "|" | "&" | ">" | "<" => tokens.push(Token::Operator(s.to_string())),

            ":" | "?" | "[" | "]" | "{" | "}" | "(" | ")" | "#" | "," | "." | "!" | ";" | "\\" | "\n" => {
                let c = s.chars().next().unwrap();
                tokens.push(Token::Symbol(c));
            }

            n if n.chars().all(|c| c.is_ascii_digit()) => tokens.push(Token::Number(n.to_string())),

            "true" | "false" => tokens.push(Token::Boolean(s == "true")),

            _ => tokens.push(Token::Identifier(s.to_string())),
        }
        i+=1;
    }
    remove_quotes(&mut tokens);
    tokens
} 

fn remove_quotes(tokens: &mut Vec<Token>) {

    let mut i = 0;
    while i < tokens.len() {
        match tokens[i] {
            Token::Symbol('"') | Token::Symbol('\'') => {
                tokens.remove(i);
            }
            _ => {}
        }
        i += 1;
    }
}

fn collect_invalid_token() {

}

