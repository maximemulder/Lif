#![allow(dead_code)]
pub mod binary;
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
pub mod identifier;
pub mod number;
pub mod string;
pub mod structure;
pub mod operation;
pub mod sequence;
pub mod symbol;
pub mod then_body;
pub mod loop_body;
pub mod expressions;
pub mod declaration;
pub mod r#else;

pub use crate::node::Node as SyntaxNode;

pub trait Node {
	fn execute(&self);
}


