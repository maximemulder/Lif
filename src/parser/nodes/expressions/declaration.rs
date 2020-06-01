use crate::elements;
use crate::parser::Parser;
use crate::node::Node;

use super::super::productions::parameter::parameter;

pub fn declaration<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::expressions::DECLARATION, vec![
		parser.token(&elements::keywords::LET)?,
		parameter(parser)?,
	]));
}
