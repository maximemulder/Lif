use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

use super::expression::expression;

pub fn statement<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return parser.safe(&|parser| Ok(Node::new_production(&elements::productions::STATEMENT, vec![
		expression(parser)?,
		parser.token(&elements::symbols::SEMICOLON)?,
	])));
}
