#![allow(dead_code)]

#![macro_use]

macro_rules! get {
    ( $flow:expr ) => {{
        use crate::runtime::r#return::Jump;

        let flow = $flow;
        if flow.jump == Jump::None {
            flow.reference
        } else {
            return Ok(flow);
        }
    }
}}

macro_rules! get_loop {
    ( $flow:expr ) => {{
        use crate::runtime::r#return::Jump;

        if $flow.jump != Jump::Return {
            $flow.reference
        } else {
            return Ok($flow);
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
pub mod r#true;
pub mod r#false;
pub mod integer;
pub mod float;
pub mod string;
pub mod identifier;
pub mod r#return;
pub mod r#break;
pub mod r#continue;

pub mod build;

use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnFlow;

pub use crate::node::Node as SyntaxNode;

pub trait Executable {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a>;
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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut flow = self.sem.execute(engine);
        if let Err(mut error) = flow.as_mut() {
            if error.node.is_none(){
                error.node = Some(self.syn)
            }
        }

        flow
    }
}
