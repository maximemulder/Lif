use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

pub fn literal<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_LITERAL, vec![
		&Next::Element(&elements::STRING),
	]) {
		return Some(node);
	}

	if let Some(node) = parser.production(&elements::PRODUCTION_LITERAL, vec![
		&Next::Element(&elements::NUMBER),
	]) {
		return Some(node);
	}

	if let Some(node) = parser.production(&elements::PRODUCTION_LITERAL, vec![
		&Next::Element(&elements::IDENTIFIER),
	]) {
		return Some(node);
	}

	return None;
}
