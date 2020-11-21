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
#![feature(raw)]
#![warn(clippy::all)]

mod memory;
mod element;
mod elements;
mod lexer;
mod node;
mod nodes;
mod parser;
mod printer;
mod runtime;
mod code;

#[cfg(test)]
mod tests;

use code::Code;
use runtime::engine::Engine;
use std::env::args;
use std::io::{ Read, Write, stderr, stdin, stdout };

pub fn run(code: &Code, input: &mut dyn Read, output: &mut dyn Write, error: &mut dyn Write) {
    let tokens = lexer::lex(&code);
    if let Some(tree) = parser::nodes::run(&code, &tokens) {
        let program = nodes::build::program(&tree);
        Engine::new(input, output, error).run(&program);
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Incorrect arguments length.");
        panic!();
    }

    let code = Code::from_file(&args[1]).unwrap();
    let mut input  = stdin();
    let mut output = stdout();
    let mut error  = stderr();
    run(&code, &mut input, &mut output, &mut error);
}
