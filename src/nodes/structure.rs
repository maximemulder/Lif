use super::{ Node, SyntaxNode };
use crate::elements;
use super::r#if::If;
use super::r#loop::Loop;
use super::r#while::While;
use super::for_in::ForIn;

pub struct Expression {
	content: Box<dyn Node>,
}

impl Expression {
	pub fn build(node: &SyntaxNode) -> Expression {
		let child = node.children()[0].element;
		return Expression {
			content: match child {
				&elements::structures::IF     => Box::new(If::build(node)),
				&elements::structures::LOOP   => Box::new(Loop::build(node)),
				&elements::structures::WHILE  => Box::new(While::build(node)),
				&elements::structures::FOR_IN => Box::new(ForIn::build(node)),
				_ => panic!(),
			},
		};
	}
}

impl Node for Expression {
	fn execute(&self) {

	}
}
