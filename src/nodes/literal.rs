use super::{ Engine, Node, SyntaxNode };
use crate::elements;

use super::identifier::Identifier;
use super::string::String;
use super::number::Number;
use crate::runtime::Value;

enum Content {
	Identifier(Identifier),
	String(String),
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
				&elements::variables::IDENTIFIER => Content::Identifier(Identifier::build(child)),
				&elements::variables::STRING     => Content::String(String::build(child)),
				&elements::variables::NUMBER     => Content::Number(Number::build(child)),
				_ => panic!(),
			},
		};
	}
}

impl Node for Literal {
	fn execute(&self, engine: &mut Engine) -> Option<usize> {
		return Some(match &self.content {
			Content::Identifier(identifier) => engine.get_variable(&identifier.text),
			Content::String(string)         => engine.new_value(Value::new_string(&string.text)),
			Content::Number(number)         => engine.new_value(Value::new_integer(number.text.parse::<usize>().unwrap())),
		});
	}
}
