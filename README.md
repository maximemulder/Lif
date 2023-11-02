# Lif

## Introduction

This is an interpreter for Lif, a small dynamically typed imperative programming language.

## Disclaimer

A lot of this project was written a few years ago when I was a beginner in both Rust and interpreters. Although I have done some significant refactoring, many designs are still far from optimal. In the long term, I would like to evolve this project and explore gradual typing with optional compilation using static evaluation. However, I think I will be prioritizing [Clam](https://github.com/MaximeMulder/clam) in the short and medium terms.

## How to build

This interpreter is written in Rust, you can download the compiler and tools for the language [here](https://www.rust-lang.org/).
Once installed, you need to switch to Rust nightly using the command `rustup default nightly`.
Finally, you can use the command `cargo build` in the project root directory to build the program.

## How to run

In order to run a Lif program, you must first write the code in a text file.
Then, you can use this interpreter by providing the file path as a command line argument.

## Features

Lif features classic dynamic and object-oriented features that are listed here. More examples can be found in the `samples` directory, which is the directory used for tests.

### Classes

Lif is a class-based object-oriented programming language. Everything is an object and methods can access their receiver using the `self` keyword. Lif also supports single inheritance.

```
class Counter {
    function increment() {
        self.i += 1;
    }
}

class ResetCounter : Counter {
    function reset() {
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

Most language constructs are also expressions, including `if` and other control flow statements.

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
for element in List[Any](true, "Boat", 0) {
    print(element); // Prints "true", "Boat", "0"
}
```

## Maintainance

This project will probably receive updates when I feel like working on it, which may be a little random.

Desirable refactors:
- Parser
- Real generics (they are currently more templates than anything else)
- More pretty errors coverage
- Primitive declarations and small standard library
- Language improvements (constants...)
- Garbage collector improvements

## License

This project is distributed under the MIT license.
