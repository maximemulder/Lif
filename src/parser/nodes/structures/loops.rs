use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

use super::super::expression::expression;
use super::block::structure_block;

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

pub fn structure_loop<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::structures::LOOP, vec![
		parser.token(&elements::keywords::LOOP)?,
		expression(parser)?,
	]));
}

pub fn structure_while<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::structures::WHILE, vec![
		parser.token(&elements::keywords::WHILE)?,
		expression(parser)?,
		structure_do(parser)?,
	]));
}

pub fn structure_do_while<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::structures::DO_WHILE, vec![
		parser.token(&elements::keywords::DO)?,
		structure_do(parser)?,
		parser.token(&elements::keywords::WHILE)?,
		expression(parser)?,
	]));
}

pub fn structure_for_in<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::structures::FOR_IN, vec![
		parser.token(&elements::keywords::FOR)?,
		parser.token(&elements::variables::IDENTIFIER)?,
		parser.token(&elements::keywords::IN)?,
		expression(parser)?,
		structure_do(parser)?,
	]));
}
