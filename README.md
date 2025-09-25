
ai generated readme of my plans 

# WIP
(work in progress)

# TELL Programming Language

TELL stands for “Textually Explicit and Lightweight Language.” It’s a compiled, general-purpose programming language that prioritizes readability and explicitness over brevity. Built as a wrapper over Rust, TELL aims to make systems-level performance accessible through a simplified and expressive syntax.



## concept

TELL  intentionally avoids shorthand, magical behavior, and obscure syntax. Every line of code is meant to be readable, teachable, and obvious in function — even at the cost of verbosity.




## Error Handling

TELL replaces try/catch blocks with a clean `.except` syntax for handling fallible operations.
-> i also plan on adding the option of making alist of common errors that can be used for common cases in a file.
Errors can also be mapped at the file or function level for reusable recovery logic. Under the hood, TELL wraps Rust’s Option and Result types, but users never need to unwrap manually.
(i honestly didnt undersand this part but chatgpt seems to be cooking somehting)

For example, when calling a function that may fail, users can attach an `.except` clause to handle specific failure cases. Errors not explicitly handled will fall back to default mappings defined at the file level.



## Standard Library

TELL ships with a full-featured standard library by default. This includes:  
- Input/output operations  
- Math and string utilities  
- Lists, dictionaries, and tuples  
- File and system tools  
- Date and time support (i dont remember saying that one dawg)

Most of the standard library is written in TELL itself. (PAUSE - i didnt say that.. :sob:) Only essential external crates are used internally to reduce compile time and improve stability.



## Integration with Rust

TELL is built on top of Rust and transpiles directly into  Rust code. It retains Rust’s performance, safety, and ecosystem while abstracting away its complexity. (hopefully)

So I can call functions from Rust crates using simplified syntax. Macros and special behaviors are detected automatically when a crate is imported. For advanced cases, developers can manually wrap crates...
                                                                 (i still dont fully understand macros so this part is a rough idea)
                                                                 


## ideas
  
- type annotations on definitions  
- Explicit mutability: variables must be marked mutable to be reassigned  
- Readable function syntax with defined argument and return types  
- Familiar conditionals and loops with enforced block scoping  
- No references, pointers, or lifetimes exposed to users  
- Built-in list, map, tuple, string, and struct types  
- Rust-inspired performance with Python-inspired simplicity  

- Memory 

TELL hides the complexity of Rust’s ownership and borrowing. Users never interact with references or lifetimes. Variable assignments appear to copy or move values, while internally the compiler chooses the most efficient path.

The transpiler tracks how values are used:  
- If used once, they are moved directly.  
- If reused without mutation, an internal reference is used.  
- If reused with mutation, a mutable reference is used.  
- If usage would break Rust’s rules, a safe clone is inserted.  

(not very sure about this part, still fleshing out the idea)

This “copy on surface, borrow in secret” model keeps code readable while remaining performant and memory-safe.

## Compilation & Tooling

TELL compiles to a single binary. This tool handles:  
- Building programs into native executables  
- Running programs for development  
- Performing static analysis and linting before compilation  

There’s no interpreter in the initial release. All code is compiled, allowing fast execution and safe memory usage.
(Well yeah, cuz its written in Rust)


---


## Progress
- 5 june 2025 | wrote the token and started scanner/lexer
- 9 june 2025 | more to token & scanner and started parser
- 19 sep 2025 | redoing it, without the help of chatgpt this time... worked on the project layout, and changed it a bit 
- 20 sep 2025 | wrote the file reader
- 21 sep 2025 | more to file reader
- 22 sep 2025 | added punctuation and operators to reader
- 23-25 sep ..| spent alot of time fixing this: i wanted to read everything between "" or '' as full strings directly, so i had to edit the code accordingly
- 25 sep 2025 | started the tokenizer. Basically, after going through the code file, i seperated it by spaces, then went through that seperated list to make it into a a list of tokens.
Now im thinking i need to add comments
and maybe add the tokens for other stuff, other features i guess.
- 26 sep 2025 |
