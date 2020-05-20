use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

use super::statement::statement;

pub fn statements<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Node<'a, 'b> {
	let mut children = Vec::new();
	while let Ok(child) = statement(parser) {
		children.push(child);
	}

	return Node::new_production(&elements::PRODUCTION_STATEMENTS, children);
}
