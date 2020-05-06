#![allow(dead_code)]
#![feature(const_fn)]

mod element;
mod elements;
mod lexer;
mod parser;
mod token;
mod tree;

use tree::{ Child, Tree };
use std::env;
use std::fs;

fn print_tree(mut shift: usize, tree: Tree) {
    shift += 1;
    println!("{}{}", " ".repeat(shift * 4), tree.element.name);
    for child in tree.children {
        match child {
            Child::Token(token) => println!("{}{}", " ".repeat((shift + 1) * 4), token.element.name),
            Child::Tree(tree) => print_tree(shift, tree),
        };
    }
}

fn main() {
    println!("Leaf compiler.");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Incorrect arguments length.");
        panic!();
    }

    let text = fs::read_to_string(&args[1]).expect("");

    let tokens = lexer::lex(&text);
    for token in tokens.iter() {
        println!("{} {:?}", token.element.name, token.string);
    }

    println!("=====");

    let mut tokens2 = Vec::new();
    for token in tokens.iter() {
        tokens2.push(token);
    }

    if let Some(tree) = parser::run(&mut tokens2) {
        print_tree(0, tree);
    }
}
