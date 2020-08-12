#![allow(dead_code)]
pub mod expression;
pub mod program;
pub mod r#if;
pub mod r#loop;
pub mod r#while;
pub mod for_in;
pub mod statement;
pub mod statements;
pub mod unary;
pub mod literal;
pub mod token;
pub mod structure;
pub mod operation;
pub mod sequence;
pub mod then;
pub mod r#do;
pub mod expressions;
pub mod declaration;
pub mod r#else;
pub mod parameters;
pub mod function;
pub mod block;
pub mod group;
pub mod integer;
pub mod identifier;
pub mod string;

use crate::runtime::{ Engine, Reference };

pub use crate::node::Node as SyntaxNode;

pub trait Node {
	fn execute<'a>(&'a self, engine: &Engine<'a>) -> Reference;
}


