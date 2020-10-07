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
pub mod operation;
pub mod chain;
pub mod method;
pub mod sequence;
pub mod declaration;
pub mod generic;
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
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub use crate::node::Node as SyntaxNode;

pub trait Executable<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b>;
}

pub struct Node<'a> {
	pub syn: &'a SyntaxNode<'a>,
	pub sem: Box<dyn Executable<'a> + 'a>,
}

impl<'a> Node<'a> {
	pub fn new(syn: &'a SyntaxNode<'a>, sem: impl Executable<'a> + 'a) -> Self {
		return Self {
			syn,
			sem: Box::new(sem),
		};
	}
}

impl<'a> Executable<'a> for Node<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		return self.sem.execute(engine);
	}
}
