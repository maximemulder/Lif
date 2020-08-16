use super::{ Node, SyntaxNode };
use crate::elements;
use super::identifier::Identifier;
use super::integer::Integer;
use super::string::String;

pub fn literal(node: &SyntaxNode) -> Box<dyn Node> {
	let child = &node.children()[0];
	return match child.element {
		&elements::variables::NUMBER     => Box::new(Integer::build(child)),
		&elements::variables::STRING     => Box::new(String::build(child)),
		&elements::variables::IDENTIFIER => Box::new(Identifier::build(child)),
		_ => panic!(),
	};
}
