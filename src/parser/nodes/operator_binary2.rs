use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

pub fn operator_binary2<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(mut children) = parser.commit(vec![&Next::Token(&elements::SYMBOL_PLUS)]) {
		return Some(children.remove(0));
	}

	if let Some(mut children) = parser.commit(vec![&Next::Token(&elements::SYMBOL_MINUS)]) {
		return Some(children.remove(0));
	}

	return None;
}
