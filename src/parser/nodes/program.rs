use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

use super::statements::statements;

pub fn program<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Node<'a, 'b> {
	return Node::new_production(&elements::productions::PROGRAM, vec![statements(parser)]);
}
