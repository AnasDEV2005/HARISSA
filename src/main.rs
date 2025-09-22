mod reader;
mod util;
mod syntaxtree;
mod tokenizer;



fn main() {
    println!("\n\nTesting File Reading");
    let contents = reader::read_file("./example.tel");

    
    let parsed_contents = reader::parse_contents(contents);

    // test
    println!("{:?}", parsed_contents);
}

