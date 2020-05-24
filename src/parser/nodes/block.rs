use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

use super::statements::statements;

pub fn block<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return parser.safe(&|parser| Ok(Node::new_expression(&elements::PRODUCTION_BLOCK, vec![
		parser.token(&elements::SYMBOL_BRACE_L)?,
		statements(parser),
		parser.token(&elements::SYMBOL_BRACE_R)?,
	])));
}
