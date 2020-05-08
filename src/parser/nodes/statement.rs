use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::expression::expression;

pub fn statement<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_STATEMENT, vec![
		&Next::Function(&expression),
		&Next::Element(&elements::SYMBOL_SEMICOLON),
	]) {
		return Some(node);
	}

	return None;
}
