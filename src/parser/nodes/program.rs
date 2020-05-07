use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::statements::statements;

pub fn program<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(children) = parser.commit(vec![&Next::Function(&statements)]) {
		return Some(Node::new_production(&elements::PRODUCTION_PROGRAM, children));
	}

	return None;
}
