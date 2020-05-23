use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

use super::expression::expression;

pub fn group<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return parser.safe(&|parser| Ok(Node::new_expression(&elements::PRODUCTION_GROUP, vec![
		parser.token(&elements::SYMBOL_PARENTHESIS_L)?,
		expression(parser)?,
		parser.token(&elements::SYMBOL_PARENTHESIS_R)?,
	])));
}
