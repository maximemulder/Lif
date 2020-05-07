use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::expression::expression_6;

pub fn group<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.production(&elements::PRODUCTION_GROUP, vec![
		&Next::Element(&elements::SYMBOL_PARENTHESIS_L),
		&Next::Function(&expression_6),
		&Next::Element(&elements::SYMBOL_PARENTHESIS_R),
	]);
}
