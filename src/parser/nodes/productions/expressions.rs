use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

use super::super::expressions::expression;

pub fn expressions<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Node<'a, 'b> {
	let mut children = Vec::new();
	while let Ok(child) = expression(parser) {
		children.push(child);
		if let Ok(separator) = parser.token(&elements::symbols::COMMA) {
			children.push(separator);
		} else {
			break;
		}
	}

	return Node::new_production(&elements::productions::EXPRESSIONS, children);
}
