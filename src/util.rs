// src/util.rs

use std::fs;
use std::process::Command;

pub fn normalize_input(input: &str) -> String {
    // Define the set of single-character operators/symbols you care about
    let symbols: [char; 24] = [
        ':', '=', '+', '-', '/', '*', '%', '|', '&', '?', '[', ']', '{', '}', '(', ')', '#', '>',
        '<', ',', '.', '!', ';', '\\',
    ];

    let mut result = String::new();

    for c in input.chars() {
        if symbols.contains(&c) {
            // add spaces around the operator
            result.push(' ');
            result.push(c);
            result.push(' ');
        } else {
            result.push(c);
        }
    }

    result
}

/// Write a String to a file (overwriting if it exists)
pub fn write_file(path: &str, content: &str) -> Result<(), std::io::Error> {
    fs::write(path, content)
}

/// Invoke `rustc` on the given .rs file (errors printed to console)
pub fn compile_rust(source_rs: &str) {
    let output = Command::new("rustc")
        .arg(source_rs)
        .output()
        .expect("Failed to invoke rustc");
    if !output.status.success() {
        eprintln!("rustc failed:\n{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    } else {
        println!("Compiled {} successfully.", source_rs);
    }
}
