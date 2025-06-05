ai generated readme of my plans 

# TELL Programming Language

TELL stands for “Textual Explicit Lightweight Language.” It’s a compiled, general-purpose programming language that prioritizes readability and explicitness over brevity. Built as a wrapper over Rust, TELL aims to make systems-level performance accessible through a simplified and expressive syntax.

---

## Philosophy

TELL is for developers who value clarity. It intentionally avoids shorthand, magical behavior, and obscure syntax. Every line of code is meant to be readable, teachable, and obvious in function — even at the cost of verbosity.

TELL reduces mental overhead by removing hidden memory concepts, enforcing explicit typing, and providing predictable structure. It emphasizes learning-friendly design while keeping powerful Rust capabilities under the hood.

---

## Key Features

- Curly-brace syntax, similar to C-like languages  
- No semicolon requirement at end of lines  
- Mandatory type annotations on definitions (with some inference)  
- Explicit mutability: variables must be marked mutable to be reassigned  
- Readable function syntax with defined argument and return types  
- Familiar conditionals and loops with enforced block scoping  
- No references, pointers, or lifetimes exposed to users  
- Built-in list, map, tuple, string, and struct types  
- Rust-inspired performance with Python-inspired simplicity  

---

## Memory Model

TELL hides the complexity of Rust’s ownership and borrowing. Users never interact with references or lifetimes. Variable assignments appear to copy or move values, while internally the compiler chooses the most efficient path.

The transpiler tracks how values are used:  
- If used once, they are moved directly.  
- If reused without mutation, an internal reference is used.  
- If reused with mutation, a mutable reference is used.  
- If usage would break Rust’s rules, a safe clone is inserted.  

This “copy on surface, borrow in secret” model keeps code readable while remaining performant and memory-safe.

---

## Error Handling

TELL replaces try/catch blocks with a clean `.except` syntax for handling fallible operations. Errors can also be mapped at the file or function level for reusable recovery logic. Under the hood, TELL wraps Rust’s Option and Result types, but users never need to unwrap manually.

For example, when calling a function that may fail, users can attach an `.except` clause to handle specific failure cases. Errors not explicitly handled will fall back to default mappings defined at the file level.

---

## Standard Library

TELL ships with a full-featured standard library by default. This includes:  
- Input/output operations  
- Math and string utilities  
- Lists, dictionaries, and tuples  
- File and system tools  
- Date and time support  

Most of the standard library is written in TELL itself. Only essential external crates are used internally to reduce compile time and improve stability.

---

## Integration with Rust

TELL is built on top of Rust and transpiles directly into safe, idiomatic Rust code. It retains Rust’s performance, safety, and ecosystem while abstracting away its complexity.

Users can call functions from Rust crates using simplified syntax. Macros and special behaviors are detected automatically when a crate is imported. For advanced cases, developers can manually wrap crates using TELL’s Rust interop tools.

---

## Compilation & Tooling

TELL compiles to a single binary. This tool handles:  
- Building programs into native executables  
- Running programs for development  
- Performing static analysis and linting before compilation  

There’s no interpreter in the initial release. All code is compiled, allowing fast execution and safe memory usage.

---

## Project Roadmap

1. Support for variable declarations, printing, and input  
2. Core control flow (conditionals and loops)  
3. Functions and scopes  
4. Static checking and error reporting (precompiler)  
5. Type system and type inference  
6. Integration with Rust crates and macro support  
7. Polished standard library and CLI tools  

---


## Progress
- 5 june 2025
