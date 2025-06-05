// src/codegen.rs

use crate::ast::AstNode;
use std::fmt::Write;

/// Turn each line into a `println!` call (placeholder).
/// Eventually, each `AstNode` variant will generate real Rust code.
pub fn generate_rust(ast: &[AstNode]) -> String {
    let mut out = String::new();
    // 1. Header: import std I/O
    out.push_str("fn main() {\n");

    // 2. For each AST line, just emit a Rust `println!(...)` stub
    for node in ast {
        match node {
            AstNode::Line(tokens) => {
                // Convert tokens back to a single string
                let mut line_str = String::new();
                for tok in tokens {
                    let s = match tok {
                        crate::scanner::Token::Identifier(id) => id.clone(),
                        crate::scanner::Token::Number(n) => n.to_string(),
                        crate::scanner::Token::StringLiteral(slit) => {
                            format!("\"{}\"", slit)
                        }
                        crate::scanner::Token::Symbol(c) => c.to_string(),
                        crate::scanner::Token::Unknown(c) => c.to_string(),
                        _ => String::new(),
                    };
                    write!(line_str, "{} ", s).unwrap();
                }
                // Emit a placeholder: print the raw TELL line
                write!(out, "    println!(\"TELL> {}\\n\");\n", line_str.trim()).unwrap();
            }
        }
    }

    // 3. Close main
    out.push_str("}\n");
    out
}

