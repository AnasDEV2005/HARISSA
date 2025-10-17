
# WIP
(work in progress)

# TELL Programming Language

TELL stands for “Textually Explicit and Lightweight Language.” It’s a compiled, general-purpose programming language that prioritizes readability and explicitness over brevity. Built as a wrapper over Rust, TELL aims to make systems-level performance accessible through a simplified and expressive syntax.



# Language Specification: TELL

## 1. Overview
- **Language name:** TELLC (subject to change) 
- **Paradigm:** Expression-oriented, imperative–functional hybrid  
- **Goal / Purpose:** im still doing some self-discovery :D  
- **Execution model:** transpiled into rust  
- **Typing:** Static 

---

## 2. Lexical Structure

### 2.1 Tokens
- **Identifiers:**  
  Pattern: `[a-zA-Z_][a-zA-Z0-9_]*`
- **Numbers:**  
  Pattern: `[0-9]+(\.[0-9]+)?`
- **Strings:**  
  Pattern: `"[^"]*"`
- **Booleans:**  
  Keywords: `true`, `false`
- **Symbols:**  
  List: `() { } , ;`
- **Operators:**  
  List: `+ - * / = == != > < >= <= && || !`
- **Keywords:**  
  List: `if`, `else`, `while`, `fn`, `const`, `return`, `for ... in`

### 2.2 Comments
- **Single-line:** `// comment`
- **Multi-line:** `/* comment */`

---

## 3. Syntax (Grammar)
Use BNF or pseudocode to describe syntax rules.
```rs
run {
    const string name = "your_name";
    print(hello_world(name, 0));
}

// comment

/*
comment block
*/

fn hello_world(name string, count int): list {
    list l = [];
    
    loop i -> count {
        l.add(name)
    };
    
    return l
}

object Person {
    Age: int,
    Alive: bool,
}

```

## Error Handling

```rs
ERROR CASES { 
    file_read: "Error reading file",
    data_not_found(string): "Error data not found at {0}",
}

run {
    print(get_data("header")); // implicitly unwraps the error so will PRINT an error if file_text = error, and all std funcs that return something, will return an error.

    print(file_text("file.txt").except(handle_missing_file)); // this is to deal manually with the error
}

fn file_text(filename string): string {
    return std.file.read(filename);
}

fn get_data(header string): int {
    return db.find(header)#data_not_found(header); // to apply error case to a function
}


```

## Standard Library

- Input/output operations  
- Math and string utilities  
- Lists, dictionaries, and tuples  
- File and system tools  

### random thoughts 
  
- i wanna take some stuff from ocaml's type system

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

- 16 october | (lol i hate uni lol) working on the parser still. I thought it'd be more complicated dealing with the nested stuff, but when i split stuff up in simple one-task functions everything is clear and easy to understand. Makes the work so much easier.
To sum it up, today i worked on parsing if statements.
To clarify: the parsing will generate the ast which will be interpreted later.
But it feels like the parsing stage is gonna take more time that interpreting. At least for the basic layer of the language. I will add all my ideas later when i have a proper base working. Though i need to keep in mind my future plans while programming such as to not be put in a situation where i have to refactor a big block of code. Splitting stuff up in functions helped with that
TODO: 
- /* */ comment blocks

- 17 october | wrote a cleaner readme, with the language specification and cleared up what i had in mind for the syntax (obviously still subject to change but yeah)
also more on the parser, after finishing the if statement parsing i moved onto the loop parsing.
it might end up needing more work to be honest because for the range of the loop i would need to check in the syntax tree for the variable specified as range. could be an int, a list or a string, so, would need more work outside just parsing.

