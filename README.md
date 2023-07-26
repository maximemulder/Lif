# Lif

## Introduction

This is an interpreter for Lif, a small dynamically typed imperative programming language.

## Features

Lif features classic dynamic and object-oriented features that are listed here. More examples can be found in the `examples` directory.

### Classes

Lif is a class-based object-oriented programming language where everything is an object. Classes are declared using the keyword `class` and methods can access the receiver object using their first parameter. Lif also implements single inheritance.

```
class Counter {
    function increment(self) {
        self.i += 1;
    }
}

class ResetCounter : Counter {
    function reset(self) {
        self.i = 0;
    }
}

let counter = new(ResetCounter);
counter.i = 0;
counter.increment();
print(counter.i); // Prints "1"
```

### Functions

Functions are declared using the keyword `function` and called using parentheses.

```
function hello(name) {
    return "Hello " + name + " !";
}

print(hello("Alice")); // Prints "Hello Alice !"
```

### Expressions

In Lif, most language constructs are expressions, including `if` and other control flow statements.

```
let status = if age >= 18 {
    "Adult"
} else {
    "Minor"
};
```

### Control flow

Lif has several control flow expressions: `if`, `loop`, `while` and `for`. Lif also has `break` and `continue` expressions;

```
for element in Array[Any](true, "Boat", 0) {
    print(element); // Prints "true", "Boat", "0"
}
```

### Architecture

I wrote this project a few years ago at a time where I was a novice in both Rust and interpreters. As such, this project codebase is far from perfect and could be improved.

This interpreter uses a handwritten lexer and parser. Whenever it is given a file, it proceeds in the following order:
1. The file is lexed into a sequence of tokens.
2. The tokens are parsed into a concrete syntax tree.
3. The concrete syntax tree is used to create an abstract syntax tree.
4. The abstract syntax tree is executed using a tree-walking interpreter.

The parser is built from a set of grammar rules that look are similar to a parsing expression grammar combined with either top-down or bottom-up annotations.

A program is executed in a single pass, there is no prior static analysis to a program execution. Everything is a value, including classes and functions, and is garbage-collected.

The garbage collector is implemented using a stop-the-world tracing strategy. In the code, this is done through the `GcTrace` trait and the `GcRef<T>` smart pointer. The garbage collector is triggered naively once every x number of allocations.

## How to build

This interpreter is written in Rust, you can download the compiler and tools for the language [here](https://www.rust-lang.org/).
Once installed, you need to switch to Rust nightly using the command `rustup default nightly`.
Finally, you can use the command `cargo build` in the project root directory to build the program.

## How to run

In order to run a Lif program, you must first write the code in a text file.
Then, you can use this interpreter by providing the file path as a command line argument.

## Maintainance

This project will probably receive updates when I feel like working on it, which may be a little random.

If I have enough time, I would like to improve this project, although I also have other priorities currently.

## License

This project is distributed under the MIT license.
