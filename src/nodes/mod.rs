#![allow(dead_code)]
mod binary;
mod expression;
mod program;
mod r#if;
mod r#loop;
mod r#while;
mod for_in;
mod statement;
mod statements;
mod unary;
mod literal;
mod identifier;
mod number;
mod string;
mod structure;

pub use crate::node::Node as SyntaxNode;

pub trait Node {
	fn execute(&self);
}
