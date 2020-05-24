use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

use super::expression::expression;

pub fn group<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return parser.safe(&|parser| Ok(Node::new_production(&elements::expressions::GROUP, vec![
		parser.token(&elements::symbols::PARENTHESIS_L)?,
		expression(parser)?,
		parser.token(&elements::symbols::PARENTHESIS_R)?,
	])));
}
