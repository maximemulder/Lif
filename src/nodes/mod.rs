#![allow(dead_code)]

macro_rules! execute {
	( $engine:expr, $node:expr ) => {{
		let reference = $engine.execute($node)?;
		if $engine.control.is_none() {
			reference
		} else {
			return Ok(reference);
		}
	}}
}

pub mod program;
pub mod statements;
pub mod statement;
pub mod expression;
pub mod structure;
pub mod r#if;
pub mod r#loop;
pub mod r#while;
pub mod do_while;
pub mod for_in;
pub mod operation;
pub mod sequence;
pub mod declaration;
pub mod function;
pub mod block;
pub mod group;
pub mod integer;
pub mod string;
pub mod identifier;
pub mod chain;
pub mod r#return;
pub mod r#break;
pub mod r#continue;

pub mod build;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub use crate::node::Node as SyntaxNode;

pub trait Node {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a>;
}
