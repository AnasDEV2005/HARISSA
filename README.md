
# WIP
(work in progress)

# TELL Programming Language

TELL stands for “Textually Explicit and Lightweight Language.” It’s a compiled, general-purpose programming language that prioritizes readability and explicitness over brevity. Built as a wrapper over Rust, TELL aims to make systems-level performance accessible through a simplified and expressive syntax.



## concept

TELL  intentionally avoids shorthand, magical behavior, and obscure syntax. Every line of code is meant to be readable, teachable, and obvious in function — even at the cost of verbosity.




## Error Handling

the plan is to add the .except option to handle a function failing (if u dont know what happens inside the function)
and for errors values, ill figure something out that ressembles that result or option features of rust


## Standard Library

i dont want crate importing hell so, the standard library:

- Input/output operations  
- Math and string utilities  
- Lists, dictionaries, and tuples  
- File and system tools  
- Date and time support (i dont remember saying that one dawg)

(no idea how to implement the math stuff but oh well)

## ideas
  
- type annotations on definitions (not obligated)

- my code is supposed to deal with the borrowing system, so the programmer can assign stuff with less worry

- function with defined argument and return types  

- i wanna take some stuff from ocaml's type system


The transpiler tracks how values are used:  
- If used once, they are moved directly.  
- If reused without mutation, an internal reference is used.  
- If reused with mutation, a mutable reference is used.  
- If usage would break Rust’s rules, a safe clone is inserted.  
sounds useful so imma keep it but this was ai
(not very sure about this part, still fleshing out the idea)

This “copy on surface, borrow in secret” model keeps code readable while remaining performant and memory-safe.

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
- 26 sep 2025 | added the possibility to ignore comments and dealt with the newline issue (i was splitting elements by whitespace only)
- 27 sep - 5 oct 2025 | made the tokenizer, fixed some stuff, wrote syntax tree, started with converting token stream into syntax tree
this is looking complicated
alot of ifs and matches
so it will take time i think

