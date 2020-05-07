use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::literal::literal;

pub fn expression_1<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(children) = parser.commit(vec![&Next::Production(&literal)]) {
		return Some(Node::new_production(&elements::PRODUCTION_EXPRESSION, children));
	}

	return None;
}
