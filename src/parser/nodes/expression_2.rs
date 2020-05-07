use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::expression_1::expression_1;
use super::operation_binary_1::operation_binary_1;

pub fn expression_2<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(children) = parser.commit(vec![&Next::Function(&operation_binary_1)]) {
		return Some(Node::new_production(&elements::PRODUCTION_EXPRESSION, children));
	}

	return expression_1(parser);
}
