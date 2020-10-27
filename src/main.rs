#![allow(incomplete_features)]
#![allow(dead_code)]
#![feature(const_fn)]
#![feature(new_uninit)]
#![feature(maybe_uninit_ref)]
#![feature(maybe_uninit_extra)]
#![feature(unsize)]
#![feature(drain_filter)]
#![feature(const_generics)]
#![feature(const_fn_fn_ptr_basics)]
#![warn(clippy::all)]

mod element;
mod elements;
mod lexer;
mod node;
mod nodes;
mod parser;
mod printer;
mod runtime;
mod code;

use code::Code;
use runtime::engine::Engine;
use std::cmp::min;
use std::env;

fn main() {
    // println!("Leaf compiler.");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Incorrect arguments length.");
        panic!();
    }

    let code = Code::new(&args[1]).unwrap();
    let tokens = lexer::lex(&code.text);
    // printer::tokens(&code, &tokens);

    // println!("=====");

    if let Some(tree) = parser::nodes::run(&code, &tokens) {
        // printer::tree(&tree);
        let program = nodes::build::program(&code.text, &tree);
        let mut engine = Engine::new();
        let result = engine.execute(&program);
        if let Err(error) = result {
            println!("{}", error.message);
            if let Some(node) = error.node {
                println!("\n{}\n{}{}",
                    code.node_line(&node),
                    " ".repeat(code.node_shift_left(&node)),
                    "^".repeat(min(code.node_str(&node).len(), code.node_shift_right(&node)))
                );
            }
        }
    }
}
