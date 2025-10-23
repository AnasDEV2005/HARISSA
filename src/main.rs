use lexer::tokenize;

mod util;
mod syntaxtree;
mod interpreter;
mod parser;
mod lexer;


fn main() {
    let contents = lexer::read_file("./example.tel");
    
    let tokens = tokenize(contents);
    // let parsed_contents = reader::parse_contents(contents);

   // println!("{:?}", tokens);

    // let tokens = tokenizer::tokenize(parsed_contents);

    // test
    for token in &tokens { 
        println!("{:?}", token); }
    
    /*
    let mut parser = parser::Parser {
        tokens: tokens,
        position: 0
    };
*/
   // let parsed = parser.parse();
    // println!("{:?}", parsed);
}
