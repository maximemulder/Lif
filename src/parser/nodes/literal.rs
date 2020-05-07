use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

pub fn expression_literal<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(children) = parser.commit(vec![&Next::Token(&elements::STRING)]) {
		return Some(Node::new_production(&elements::PRODUCTION_EXPRESSION, children));
	}

	if let Some(children) = parser.commit(vec![&Next::Token(&elements::NUMBER)]) {
		return Some(Node::new_production(&elements::PRODUCTION_EXPRESSION, children));
	}

	if let Some(children) = parser.commit(vec![&Next::Token(&elements::IDENTIFIER)]) {
		return Some(Node::new_production(&elements::PRODUCTION_EXPRESSION, children));
	}

	return None;
}
