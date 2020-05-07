use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::statements::statements;

pub fn program<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_PROGRAM, vec![
		&Next::Function(&statements),
	]) {
		return Some(node);
	}

	return None;
}
