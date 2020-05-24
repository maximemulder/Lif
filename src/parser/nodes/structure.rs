use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

use super::expression::expression;
use super::statements::statements;

fn structure_else<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Node<'a, 'b> {
	return Node::new_production(&elements::productions::ELSE, if let Ok(nodes) = parser.safes(&|parser| Ok(vec![
		parser.token(&elements::keywords::ELSE)?,
		expression(parser)?,
	])) {
		nodes
	} else {
		Vec::new()
	});
}

fn structure_then<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::productions::THEN, if let Ok(nodes) = parser.safes(&|parser| Ok(vec![
		parser.token(&elements::keywords::THEN)?,
		expression(parser)?,
	])) {
		nodes
	} else {
		parser.safes(&|parser| Ok(vec![
			structure_block(parser)?,
		]))?
	}));
}

fn structure_do<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::productions::DO, if let Ok(nodes) = parser.safes(&|parser| Ok(vec![
		parser.token(&elements::keywords::DO)?,
		expression(parser)?,
	])) {
		nodes
	} else {
		parser.safes(&|parser| Ok(vec![
			structure_block(parser)?,
		]))?
	}));
}

fn structure_block<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return parser.safe(&|parser| Ok(Node::new_production(&elements::structures::BLOCK, vec![
		parser.token(&elements::symbols::BRACE_L)?,
		statements(parser),
		parser.token(&elements::symbols::BRACE_R)?,
	])));
}

fn structure_if<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::structures::IF, parser.safes(&|parser| Ok(vec![
		parser.token(&elements::keywords::IF)?,
		expression(parser)?,
		structure_then(parser)?,
		structure_else(parser),
	]))?));
}

fn structure_loop<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::structures::LOOP, parser.safes(&|parser| Ok(vec![
		parser.token(&elements::keywords::LOOP)?,
		expression(parser)?,
	]))?));
}

fn structure_while<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::structures::WHILE, parser.safes(&|parser| Ok(vec![
		parser.token(&elements::keywords::WHILE)?,
		expression(parser)?,
		structure_do(parser)?,
	]))?));
}

fn structure_do_while<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::structures::DO_WHILE, parser.safes(&|parser| Ok(vec![
		parser.token(&elements::keywords::DO)?,
		structure_do(parser)?,
		parser.token(&elements::keywords::WHILE)?,
		expression(parser)?,
	]))?));
}

fn structure_for_in<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::structures::FOR_IN, parser.safes(&|parser| Ok(vec![
		parser.token(&elements::keywords::FOR)?,
		parser.token(&elements::variables::IDENTIFIER)?,
		parser.token(&elements::keywords::IN)?,
		expression(parser)?,
		structure_do(parser)?,
	]))?));
}

pub fn structure<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	if let Ok(node) = structure_block(parser) {
		return Ok(node);
	};

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
