use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::expression_2::expression_2;
use super::expression_3::expression_3;
use super::operator_binary_2::operator_binary_2;

pub fn operation_binary_2<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(children) = parser.commit(vec![
		&Next::Production(&expression_2),
		&Next::Production(&operator_binary_2),
		&Next::Production(&expression_3),
	]) {
		return Some(Node::new_production(&elements::PRODUCTION_OPERATION, children));
	}

	return None;
}
