use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

use super::expressions::expressions;

pub fn sequence<'a, 'b>(parser: &mut Parser<'a, 'b, '_>, mut expression: Node<'a, 'b>) -> Result<Node<'a, 'b>, ()> {
	for delimiters in [
		(&elements::SYMBOL_PARENTHESIS_L, &elements::SYMBOL_PARENTHESIS_R),
		(&elements::SYMBOL_CROTCHET_L, &elements::SYMBOL_CROTCHET_R),
	].iter() {
		if let Ok(mut nodes) = parser.safes(&|parser| Ok(vec![
			parser.token(delimiters.0)?,
			expressions(parser),
			parser.token(delimiters.1)?,
		])) {
			nodes.insert(0, expression);
			expression = Node::new_expression(&elements::PRODUCTION_SEQUENCE, nodes);
			return sequence(parser, expression);
		}
	}

	return Ok(expression);
}
