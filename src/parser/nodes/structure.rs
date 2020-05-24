use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

use super::block::block;
use super::expression::expression;

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

fn structure_do<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::PRODUCTION_STRUCTURE_DO, if let Ok(nodes) = parser.safes(&|parser| Ok(vec![
		parser.token(&elements::KEYWORD_DO)?,
		expression(parser)?,
	])) {
		nodes
	} else {
		parser.safes(&|parser| Ok(vec![
			block(parser)?,
		]))?
	}));
}

fn structure_if<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_expression(&elements::PRODUCTION_STRUCTURE_IF, parser.safes(&|parser| Ok(vec![
		parser.token(&elements::KEYWORD_IF)?,
		expression(parser)?,
		structure_then(parser)?,
		structure_else(parser),
	]))?));
}

fn structure_loop<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_expression(&elements::PRODUCTION_STRUCTURE_LOOP, parser.safes(&|parser| Ok(vec![
		parser.token(&elements::KEYWORD_LOOP)?,
		expression(parser)?,
	]))?));
}

fn structure_while<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_expression(&elements::PRODUCTION_STRUCTURE_WHILE, parser.safes(&|parser| Ok(vec![
		parser.token(&elements::KEYWORD_WHILE)?,
		expression(parser)?,
		structure_do(parser)?,
	]))?));
}

fn structure_do_while<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_expression(&elements::PRODUCTION_STRUCTURE_DO_WHILE, parser.safes(&|parser| Ok(vec![
		parser.token(&elements::KEYWORD_DO)?,
		structure_do(parser)?,
		parser.token(&elements::KEYWORD_WHILE)?,
		expression(parser)?,
	]))?));
}

fn structure_for_in<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_expression(&elements::PRODUCTION_STRUCTURE_FOR_IN, parser.safes(&|parser| Ok(vec![
		parser.token(&elements::KEYWORD_FOR)?,
		parser.token(&elements::IDENTIFIER)?,
		parser.token(&elements::KEYWORD_IN)?,
		expression(parser)?,
		structure_do(parser)?,
	]))?));
}

pub fn structure<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	if let Ok(node) = structure_if(parser) {
		return Ok(node);
	};

	if let Ok(node) = structure_loop(parser) {
		return Ok(node);
	};

	if let Ok(node) = structure_while(parser) {
		return Ok(node);
	};

	if let Ok(node) = structure_do_while(parser) {
		return Ok(node);
	};

	if let Ok(node) = structure_for_in(parser) {
		return Ok(node);
	};

	return Err(());
}
