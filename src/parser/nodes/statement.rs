use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::expression2::expression2;

pub fn statement<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(children) = parser.commit(vec![
		&Next::Production(&expression2),
		&Next::Token(&elements::SYMBOL_SEMICOLON),
	]) {
		return Some(Node::new_production(&elements::PRODUCTION_STATEMENT, children));
	}

	return None;
}
