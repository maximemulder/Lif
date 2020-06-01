use crate::elements;
use crate::parser::Parser;
use crate::node::Node;

use super::super::productions::r#type::r#type;

pub fn parameter<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::productions::PARAMETER, vec![
		parser.token(&elements::variables::IDENTIFIER)?,
		r#type(parser),
	]));
}
