use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

use super::super::productions::expressions::expressions;

pub fn sequence<'a, 'b>(parser: &mut Parser<'a, 'b, '_>, mut expression: Node<'a, 'b>) -> Result<Node<'a, 'b>, ()> {
	for delimiters in [
		(&elements::symbols::PARENTHESIS_L, &elements::symbols::PARENTHESIS_R),
		(&elements::symbols::CROTCHET_L, &elements::symbols::CROTCHET_R),
	].iter() {
		if let Ok(mut nodes) = parser.safes(&|parser| Ok(vec![
			parser.token(delimiters.0)?,
			expressions(parser),
			parser.token(delimiters.1)?,
		])) {
			nodes.insert(0, expression);
			expression = Node::new_production(&elements::expressions::SEQUENCE, nodes);
			return sequence(parser, expression);
		}
	}

	return Ok(expression);
}
