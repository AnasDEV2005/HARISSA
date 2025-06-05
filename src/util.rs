// src/util.rs

use std::fs;
use std::process::Command;

/// Read the entire file into a String
pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
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

