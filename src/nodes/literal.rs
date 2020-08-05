use super::{ Node, SyntaxNode };
use crate::elements;

use super::string::String;
use super::identifier::Identifier;
use super::number::Number;

enum Content {
	String(String),
	Identifier(Identifier),
	Number(Number),
}

pub struct Literal {
	content: Content,
}

impl Literal {
	pub fn build(node: &SyntaxNode) -> Literal {
		let child = &node.children()[0];
		return Literal {
			content: match child.element {
				&elements::variables::STRING     => Content::String(String::build(child)),
				&elements::variables::IDENTIFIER => Content::Identifier(Identifier::build(child)),
				&elements::variables::NUMBER     => Content::Number(Number::build(child)),
				_ => panic!(),
			},
		};
	}
}

impl Node for Literal {
	fn execute(&self) {

	}
}
