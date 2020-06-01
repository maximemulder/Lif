use crate::elements;
use crate::parser::Parser;
use crate::node::Node;

use super::super::expressions::expression_13;

pub fn r#type<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Node<'a, 'b> {
	return Node::new_production(&elements::productions::TYPE, {
		if let Ok(children) = parser.safes(&|parser| Ok(vec![
			parser.token(&elements::symbols::SEMICOLON)?,
			expression_13(parser)?,
		])) {
			children
		} else {
			Vec::new()
		}
	});
}
