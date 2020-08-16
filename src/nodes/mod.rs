#![allow(dead_code)]

#[macro_use]
macro_rules! value {
	( $product:expr ) => {{
		let product = $product;
		if product.control.is_none() {
			product.reference
		} else {
			return product;
		}
	}}
}

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
pub mod chain;
pub mod control;
pub mod r#return;
pub mod r#break;
pub mod r#continue;

use crate::runtime::{ Engine, Reference };

pub use crate::node::Node as SyntaxNode;

pub trait Node {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product;
}

pub struct Product {
	pub reference: Reference,
	pub control: Option<Control>,
}

impl Product {
	pub fn new(reference: Reference) -> Self {
		return Self {
			reference,
			control: None
		};
	}

	pub fn new_control(reference: Reference, control: Control) -> Self {
		return Self {
			reference,
			control: Some(control),
		};
	}
}

pub enum Control {
	Return,
	Break,
	Continue,
}
