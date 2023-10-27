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
mod walker;
mod parser;
mod runtime;

#[cfg(test)]
mod tests;

use parser::Code;
use runtime::engine::Engine;
use walker::nodes::AProgram;

use std::env::args;
use std::io::{ stderr, stdin, stdout };

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Incorrect arguments length.");
        return;
    }

    let parser = parser::grammar();
    let mut input  = stdin();
    let mut output = stdout();
    let mut error  = stderr();
    let mut engine = Engine::new(&parser, &mut input, &mut output, &mut error);
    let code = Code::from_file::<AProgram>(engine.grammar, engine.grammar.program, &args[1]).unwrap();
    engine.run(code);

    println!("AAA");

    let mut input  = stdin();
    let mut output = stdout();
    let mut error  = stderr();
    let io = runtime::bis::engine::Io::new(&mut input, &mut output, &mut error);
    let mut engine = runtime::bis::engine::Engine::new(io, engine.grammar);
    let code = Code::from_file::<AProgram>(engine.grammar, engine.grammar.program, &args[1]).unwrap();
    engine.run(code);
}
