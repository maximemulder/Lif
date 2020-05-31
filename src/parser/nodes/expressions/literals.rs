use crate::element::Element;
use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

fn literal<'a, 'b>(parser: &mut Parser<'a, 'b, '_>, element: &'a Element) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::expressions::LITERAL, vec![parser.token(element)?]));
}

pub fn number<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return literal(parser, &elements::variables::NUMBER);
}

pub fn string<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return literal(parser, &elements::variables::STRING);
}

pub fn identifier<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return literal(parser, &elements::variables::IDENTIFIER);
}
