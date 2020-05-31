use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

use super::super::expressions::expression;

pub fn program<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Node<'a, 'b> {
	let mut children = Vec::new();
	while let Ok(child) = parser.safe(&|parser| Ok(Node::new_production(&elements::productions::STATEMENT, vec![
		expression(parser)?,
		parser.token(&elements::symbols::SEMICOLON)?,
	]))) {
		children.push(child);
	}

	return Node::new_production(&elements::productions::PROGRAM, vec![
		Node::new_production(&elements::productions::STATEMENTS, children)
	]);
}
