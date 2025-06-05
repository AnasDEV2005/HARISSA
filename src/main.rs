use clap::Parser;
mod scanner;
mod parser;
mod ast;
mod codegen;
mod util;

/// Simple TELL compiler (Phase 1 skeleton).
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Path to the .tell source file
    source: String,
}

fn main() {
    let args = Cli::parse();
    let src_text = util::read_file(&args.source)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", &args.source, e));

    // 1. Scan into tokens
    let tokens = scanner::tokenize(&src_text);

    // 2. Parse tokens into a very basic AST
    let ast = parser::parse_lines(tokens);

    // 3. Generate Rust code from AST
    let generated = codegen::generate_rust(&ast);

    // 4. Write Rust to disk
    let out_rs = args.source.replace(".tell", ".rs");
    util::write_file(&out_rs, &generated)
        .unwrap_or_else(|e| panic!("Failed to write {}: {}", &out_rs, e));

    // 5. Invoke rustc to compile the generated Rust
    util::compile_rust(&out_rs);
}

