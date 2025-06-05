use clap::Parser;
mod scanner;
mod parser;
mod ast;
mod codegen;
mod util;
mod token;


use scanner::Scanner;
use token::Token;

/// Simple TELL compiler (Phase 1 skeleton).
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Path to the .tell source file
    source: String,
}

fn main() {
    let source = "let x = 5 + 3";
    let mut scanner = Scanner::new(source);

    loop {
        let token = scanner.next_token();
        println!("{:?}", token);
        if token == Token::EOF {
            break;
        }
    }
}

