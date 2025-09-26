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

    println!("{:?}", parsed_contents);
    // still need to add comments
    // and think more about everything so i get done with keywords and stuff
    let tokenized = tokenizer::tokenize(parsed_contents);
    // test
    for token in &tokenized {
        println!("{:?}", token);
    }
}

