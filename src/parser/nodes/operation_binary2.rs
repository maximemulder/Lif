use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::expression_operation_binary1::expression_operation_binary1;
use super::operator_binary2::operator_binary2;

pub fn expression_operation_binary2<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(children) = parser.commit(vec![
		&Next::Production(&expression_operation_binary1),
		&Next::Production(&operator_binary2),
		&Next::Production(&expression_operation_binary2),
	]) {
		return Some(Node::new_production(&elements::PRODUCTION_EXPRESSION, children));
	}

	return expression_operation_binary1(parser);
}
