#![allow(dead_code)]
#![feature(const_fn)]

mod element;
mod elements;
mod lexer;
mod node;
mod parser;
mod printer;

use std::env;
use std::fs;

fn main() {
    println!("Leaf compiler.");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Incorrect arguments length.");
        panic!();
    }

    let text = fs::read_to_string(&args[1]).expect("");
    let tokens = lexer::lex(&text);
    printer::tokens(&tokens);

    println!("=====");

    if let Ok(tree) = parser::run(&tokens) {
        printer::tree(&tree);
    }
}
