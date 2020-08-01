use super::{ Node, SyntaxNode };
use super::literal::Literal;
use super::structure::Structure;
use crate::elements;

pub struct Expression {
	content: Box<dyn Node>,
}

impl Expression {
	pub fn build(node: &SyntaxNode) -> Expression {
		let child = node.children()[0].element;
		return Expression {
			content: match child {
				&elements::expressions::LITERAL   => Box::new(Literal::build(node)),
				&elements::structures::STRUCTURE => Box::new(Structure::build(node)),
				_ => panic!(),
			},
		};
	}
}

impl Node for Expression {
	fn execute(&self) {

	}
}
