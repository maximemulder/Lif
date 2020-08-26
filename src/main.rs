#![allow(dead_code)]
#![feature(const_fn)]
#![feature(new_uninit)]
#![feature(maybe_uninit_ref)]
#![feature(unsize)]
#![feature(drain_filter)]

mod element;
mod elements;
mod lexer;
mod node;
mod parser;
mod parser2;
mod printer;
mod nodes;
mod runtime;

use runtime::engine::Engine;
use nodes::Node;
use std::env;
use std::fs;

fn cheat<T>(value: &T) -> &mut T {
	return unsafe { (value as *const T as *mut T).as_mut().unwrap() };
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
    printer::tokens(&tokens);

    println!("=====");

    if let Some(tree) = parser2::nodes::run(&tokens) {
        printer::tree(&tree);
		let program = nodes::build::program(&tree);
		program.execute(&mut Engine::new());
	}
}
