use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

use super::block::block;
use super::expression::expression;

fn structure_if<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_expression(&elements::PRODUCTION_STRUCTURE_IF, parser.safes(&|parser| Ok(vec![
		parser.token(&elements::KEYWORD_IF)?,
		expression(parser)?,
		structure_then(parser)?,
		structure_else(parser),
	]))?));
}

fn structure_then<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::PRODUCTION_STRUCTURE_IF_THEN, if let Ok(nodes) = parser.safes(&|parser| Ok(vec![
		parser.token(&elements::KEYWORD_THEN)?,
		expression(parser)?,
	])) {
		nodes
	} else {
		parser.safes(&|parser| Ok(vec![
			block(parser)?,
		]))?
	}));
}

fn structure_else<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Node<'a, 'b> {
	return Node::new_production(&elements::PRODUCTION_STRUCTURE_IF_ELSE, if let Ok(nodes) = parser.safes(&|parser| Ok(vec![
		parser.token(&elements::KEYWORD_ELSE)?,
		expression(parser)?,
	])) {
		nodes
	} else {
		Vec::new()
	});
}

pub fn structure<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	if let Ok(node) = structure_if(parser) {
		return Ok(node);
	};

	return Err(());
}
