use crate::element::Element;
use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

use super::super::expression::expression;

fn structure_control<'a, 'b>(
	parser: &mut Parser<'a, 'b, '_>,
	element: &'a Element,
	token: &'a Element
) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(element, {
		let mut children = Vec::new();
		children.push(parser.token(token)?);
		if let Ok(expression) = parser.safe(&|parser| expression(parser)) {
			children.push(expression);
		}

		children
	}));
}

pub fn structure_break<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return structure_control(parser, &elements::structures::BREAK, &elements::keywords::BREAK);
}

pub fn structure_continue<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return structure_control(parser, &elements::structures::CONTINUE, &elements::keywords::CONTINUE);
}

pub fn structure_return<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return structure_control(parser, &elements::structures::RETURN, &elements::keywords::RETURN);
}
