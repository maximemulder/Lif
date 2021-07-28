#![allow(incomplete_features)]
#![allow(dead_code)]
#![feature(bool_to_option)]
#![feature(drain_filter)]
#![feature(const_generics)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_fn_trait_bound)]
#![feature(maybe_uninit_extra)]
#![feature(new_uninit)]
#![feature(raw)]
#![feature(unsize)]
#![warn(clippy::all)]

mod memory;
mod walker;
mod parser;
mod runtime;

#[cfg(test)]
mod tests;

use parser::Code;
use runtime::engine::Engine;
use walker::build;

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
    let code = Code::from_file(engine.grammar, engine.grammar.program, &build::program, &args[1]).unwrap();
    engine.run(code);
}
