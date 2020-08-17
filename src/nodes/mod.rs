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
