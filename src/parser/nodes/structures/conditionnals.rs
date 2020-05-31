use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

use super::super::expressions::expression;
use super::block::block;

fn r#else<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Node<'a, 'b> {
	return Node::new_production(&elements::productions::ELSE, if let Ok(nodes) = parser.safes(&|parser| Ok(vec![
		parser.token(&elements::keywords::ELSE)?,
		expression(parser)?,
	])) {
		nodes
	} else {
		Vec::new()
	});
}

fn then<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::productions::THEN, if let Ok(nodes) = parser.safes(&|parser| Ok(vec![
		parser.token(&elements::keywords::THEN)?,
		expression(parser)?,
	])) {
		nodes
	} else {
		vec![block(parser)?]
	}));
}

pub fn r#if<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::structures::IF, vec![
		parser.token(&elements::keywords::IF)?,
		expression(parser)?,
		then(parser)?,
		r#else(parser),
	]));
}
