mod reader;
mod util;
mod syntaxtree;
mod tokenizer;
mod interpreter;
mod parser;
mod errors;


fn main() {
    println!("\n\nTesting File Reading");
    let contents = reader::read_file("./example.tel");
    let normalized = util::normalize_input(contents.as_str());
    
    let parsed_contents = reader::parse_contents(contents);

   // println!("{:?}", parsed_contents);


    let tokens = tokenizer::tokenize(parsed_contents);
    if let Err(e) = tokenizer::validate_tokens(&tokens) {
        println!("Syntax error: {:?}", e);
        return;
    }

    for token in &tokens { println!("{:?}", token); }

    let mut parser = parser::Parser {
        tokens: tokens,
        position: 0,
        syn_errors: vec![],
    };

    let parsed = parser.parse();
    println!("{:?}", parsed);


    // NOTE: need to finish scanning for lexical errors first
    // then ill setup syntax error management
    // and then ill continue parsing
    
    
    
    
}

