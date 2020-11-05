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
use std::env::args;
use std::io::{ stderr, stdin, stdout };

fn main() {
    // println!("Leaf compiler.");

    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("Incorrect arguments length.");
        panic!();
    }

    let code = Code::new(&args[1]).unwrap();
    let tokens = lexer::lex(&code);
    // printer::tokens(&code, &tokens);

    // println!("=====");

    if let Some(tree) = parser::nodes::run(&code, &tokens) {
        // printer::tree(&tree);
        let program = nodes::build::program(&tree);
        let mut input  = stdin();
        let mut output = stdout();
        let mut error  = stderr();
        let mut engine = Engine::new(&mut input, &mut output, &mut error);
        let result = engine.execute(&program);
        if let Err(error) = result {
            eprintln!("{}", error.message);
            if let Some(node) = error.node {
                eprintln!("{}\n{}\n{}{}",
                    code.name,
                    code.node_line(&node),
                    " ".repeat(code.node_shift_left(&node)),
                    "^".repeat(min(code.node_str(&node).len(), code.node_shift_right(&node)))
                );
            }
        }
    }
}
