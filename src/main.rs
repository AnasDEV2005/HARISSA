mod reader;
mod util;
mod syntaxtree;
mod tokenizer;
mod interpreter;
mod parser;


fn main() {
    println!("\n\nTesting File Reading");
    let contents = reader::read_file("./example.tel");
    let normalized = util::normalize_input(contents.as_str());
    
    let parsed_contents = reader::parse_contents(contents);

   // println!("{:?}", parsed_contents);

    let tokens = tokenizer::tokenize(parsed_contents);
    // test
    for token in &tokens { println!("{:?}", token); }
    let mut parser = parser::Parser {
        tokens: tokens,
        position: 0
    };

    let parsed = parser.parse();
}

