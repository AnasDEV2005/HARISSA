#[allow(warnings)]
use lexer::tokenize;

mod expression_parser;
mod interpreter;
mod lexer;
mod parser;
mod syntaxtree;
mod util;

fn main() {
    let contents = lexer::read_file("./example.tel");

    let tokens = tokenize(contents);
    // let parsed_contents = reader::parse_contents(contents);

    // println!("{:?}", tokens);

    // let tokens = tokenizer::tokenize(parsed_contents);

    // test
    // for token in &tokens {
    //     println!("{:?}", token); }

    let parsed = parser::parse(tokens);
    println!("{:?}", parsed);
}
