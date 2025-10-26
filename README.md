
# WIP
(work in progress)

# TELL Programming Language

Name is subject to change.



# Language Specification: TELL

## Overview
- **Language name:** TELL (subject to change) 
- **Paradigm:** Pretty much like rust.
- **Goal / Purpose:** im still doing some self-discovery :D  
- **Execution model:** transpiled into rust  
- **Typing:** Static 

---


## Syntax (Grammar)
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
ERROR_RULES { 
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
- File and system tools  


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
- 18 october | RAAAAAHH im almost done with parsing the loop. finished parsing the range, which is the hardest part
look at this beauty 
```rust
Loop { iterator: Some("i"), range: Some(Range((0, 1))), body: Block(x) }
```

- 22 october | alot of random duct-taping and now i got it to pick up on newline and eof bytes inside strings and and report an invalid token at it.
Honestly the tokenizer and parser are cooked and i'll definetly refactor them later but for now i want to get this somewhat up and running so i can implement all my ideas into the interpreter (before i forget them :sob: )

idk how to deal with github...
- 23 october | a new day, a new uni day... is NOT a good day.. well except i made big progress this time <br>
Started and finished the lexer. Going about it some other way, based on seeing someone else's lexer lol.
So i iterate over the string once  
basically:  
advance in string -> find something i want -> advance in string but inside another loop -> return result of previous loop -> continue from when i stopped in the loop  
so i dont go over anything more than once  
since i use match  
when im moving to the next character to check for syntax errors i already know what the previous character was based on what i matched with

- 26 october | past few days i worked on the parser. spent a whole day working on it following chatgpt's template of using a struct with methods... i wasnt familiar enough with the borrow checker so i actually got stuck.  
I then decided to rewrite the whole parser, but without a parser struct this time. just functions and passing around indexes and vectors.  
I then realized i had only set the loop to accept a (int, int) range. Which should be able to read an expression inside the (). So yeah i redid that, only solution here was recursion.  
