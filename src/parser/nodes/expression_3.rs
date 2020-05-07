use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::expression_2::expression_2;
use super::operation_binary_2::operation_binary_2;

pub fn expression_3<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(children) = parser.commit(vec![&Next::Function(&operation_binary_2)]) {
		return Some(Node::new_production(&elements::PRODUCTION_EXPRESSION, children));
	}

	return expression_2(parser);
}
