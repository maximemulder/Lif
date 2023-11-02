#![allow(incomplete_features)]
#![allow(dead_code)]
#![feature(box_into_inner)]
#![feature(const_trait_impl)]
#![feature(drain_filter)]
#![feature(new_uninit)]
#![feature(ptr_metadata)]
#![feature(unsize)]
#![warn(clippy::all)]

mod ast;
mod memory;
mod parser;
mod runtime;

#[cfg(test)]
mod tests;

use parser::Code;
use runtime::engine::{Engine, Io};

use std::env::args;
use std::fs::read_to_string;
use std::io::{ stderr, stdin, stdout };

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Incorrect arguments length.");
        return;
    }

    let grammar = parser::grammar();
    let mut input  = stdin();
    let mut output = stdout();
    let mut error  = stderr();
    let io = Io::new(&mut input, &mut output, &mut error);
    let mut engine = Engine::new(io, &grammar);
    let text = read_to_string(&args[1]).unwrap();
    let code = Code::new(engine.grammar, engine.grammar.program, Some(&args[1]), text.into_boxed_str());
    engine.run(code);
}
