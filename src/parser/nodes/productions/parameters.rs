use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

use super::parameter::parameter;

pub fn parameters<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Node<'a, 'b> {
	let mut children = Vec::new();
	while let Ok(child) = parameter(parser) {
		children.push(child);
		if let Ok(separator) = parser.token(&elements::symbols::COMMA) {
			children.push(separator);
		} else {
			break;
		}
	}

	return Node::new_production(&elements::productions::PARAMETERS, children);
}
