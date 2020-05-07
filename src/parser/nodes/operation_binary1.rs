use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::expression_literal::expression_literal;
use super::operator_binary1::operator_binary1;

pub fn expression_operation_binary1<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(children) = parser.commit(vec![
		&Next::Production(&expression_literal),
		&Next::Production(&operator_binary1),
		&Next::Production(&expression_operation_binary1),
	]) {
		return Some(Node::new_production(&elements::PRODUCTION_EXPRESSION, children));
	}

	return expression_literal(parser);
}
