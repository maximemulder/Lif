use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::expression_1::expression_1;
use super::expression_2::expression_2;
use super::operator_binary_1::operator_binary_1;

pub fn operation_binary_1<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(children) = parser.commit(vec![
		&Next::Production(&expression_1),
		&Next::Production(&operator_binary_1),
		&Next::Production(&expression_2),
	]) {
		return Some(Node::new_production(&elements::PRODUCTION_OPERATION, children));
	}

	return None;
}
