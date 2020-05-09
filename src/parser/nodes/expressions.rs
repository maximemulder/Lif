use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::expression::expression;

pub fn expressions<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.list(&elements::PRODUCTION_EXPRESSIONS,
		&Next::Function(&expression),
		&Next::Element(&elements::SYMBOL_COMMA)
	);
}
