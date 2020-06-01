use crate::elements;
use crate::parser::Parser;
use crate::node::Node;

use super::super::productions::parameters::parameters;
use super::super::productions::r#type::r#type;
use super::super::structures::block::block;

pub fn function<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::expressions::FUNCTION, vec![
		parser.token(&elements::keywords::FUNCTION)?,
		parser.token(&elements::symbols::PARENTHESIS_L)?,
		parameters(parser),
		parser.token(&elements::symbols::PARENTHESIS_R)?,
		r#type(parser),
		block(parser)?,
	]));
}
