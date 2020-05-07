use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::expression1::expression1;

pub fn expression2<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(children) = parser.commit(vec![
		&Next::Production(&expression1),
		&Next::Token(&elements::SYMBOL_PLUS),
		&Next::Production(&expression2),
	]) {
		return Some(Node::new_production(&elements::PRODUCTION_EXPRESSION, children));
	}

	return expression1(parser);
}
