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

mod element;
mod elements;
mod lexer;
mod memory;
mod node;
mod nodes;
mod parser;
mod printer;
mod runtime;
mod code;

#[cfg(test)]
mod tests;

use code::Code;
use parser::Parser;
use runtime::engine::Engine;
use std::env::args;
use std::io::{ stderr, stdin, stdout };

/* let parser = Parser::new();
if let Some(tree) = parser.parse(Ref::from_ref(&code)) {
    let program = nodes::build::program(Ref::from_ref(&tree));
    Engine::new(input, output, error).run(Ref::from_ref(&program));
} */

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Incorrect arguments length.");
        panic!();
    }

    let code = Code::from_file(&args[1]).unwrap();
    let parser = Parser::new();
    let mut input  = stdin();
    let mut output = stdout();
    let mut error  = stderr();
    Engine::new(&parser, &mut input, &mut output, &mut error).run(code);
}
