#![allow(dead_code)]

macro_rules! execute {
    ( $engine:expr, $node:expr ) => {{
        let reference = $engine.execute($node)?;
        if $engine.control_none() {
            reference
        } else {
            return Ok(reference);
        }
    }
}}

pub mod program;
pub mod statements;
pub mod statement;
pub mod r#if;
pub mod r#loop;
pub mod r#while;
pub mod do_while;
pub mod for_in;
pub mod assignment;
pub mod preop;
pub mod binop;
pub mod chain;
pub mod sequence;
pub mod declaration;
pub mod generic;
pub mod structure;
pub mod class;
pub mod function;
pub mod block;
pub mod array;
pub mod group;
pub mod integer;
pub mod string;
pub mod identifier;
pub mod r#true;
pub mod r#false;
pub mod r#return;
pub mod r#break;
pub mod r#continue;

pub mod build;

use crate::memory::Ref;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub use crate::node::Node as SyntaxNode;

pub trait Executable {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a>;
}

pub struct Node {
    pub syn: Ref<SyntaxNode>,
    pub sem: Box<dyn Executable>,
}

impl Node {
    pub fn new(syn: Ref<SyntaxNode>, sem: impl Executable + 'static) -> Self {
        Self {
            syn,
            sem: Box::new(sem),
        }
    }
}

impl Executable for Node {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        self.sem.execute(engine)
    }
}
