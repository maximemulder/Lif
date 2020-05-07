use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

pub fn operator_binary_1<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(mut children) = parser.commit(vec![&Next::Token(&elements::SYMBOL_ASTERISK)]) {
		return Some(children.remove(0));
	}

	if let Some(mut children) = parser.commit(vec![&Next::Token(&elements::SYMBOL_SLASH)]) {
		return Some(children.remove(0));
	}

	return None;
}
