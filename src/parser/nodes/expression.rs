use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::group::group;
use super::literal::literal;
use super::operation_binary::*;
use super::sequence::sequence;

pub fn expression_1<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION, vec![
		&Next::Function(&group)
	]) {
		return Some(node);
	}

	if let Ok(node) = literal(parser) {
		return Some(Node::new_production(&elements::PRODUCTION_EXPRESSION, vec![node]));
	}

	return None;
}

pub fn expression_2<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION, vec![
		&Next::Function(&sequence)
	]) {
		return Some(node);
	}

	return expression_1(parser);
}

pub fn expression_3<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION, vec![
		&Next::Function(&operation_binary_1),
	]) {
		return Some(node);
	}

	return expression_2(parser);
}

pub fn expression_4<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION, vec![
		&Next::Function(&operation_binary_2),
	]) {
		return Some(node);
	}

	return expression_3(parser);
}

pub fn expression_5<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION, vec![
		&Next::Function(&operation_binary_3),
	]) {
		return Some(node);
	}

	return expression_4(parser);
}

pub fn expression_6<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION, vec![
		&Next::Function(&operation_binary_4),
	]) {
		return Some(node);
	}

	return expression_5(parser);
}

pub fn expression_7<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION, vec![
		&Next::Function(&operation_binary_5),
	]) {
		return Some(node);
	}

	return expression_6(parser);
}

pub fn expression_8<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION, vec![
		&Next::Function(&operation_binary_6),
	]) {
		return Some(node);
	}

	return expression_7(parser);
}

pub fn expression_9<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION, vec![
		&Next::Function(&operation_binary_7),
	]) {
		return Some(node);
	}

	return expression_8(parser);
}

pub fn expression_10<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION, vec![
		&Next::Function(&operation_binary_8),
	]) {
		return Some(node);
	}

	return expression_9(parser);
}

pub fn expression_11<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION, vec![
		&Next::Function(&operation_binary_9),
	]) {
		return Some(node);
	}

	return expression_10(parser);
}

pub fn expression_12<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION, vec![
		&Next::Function(&operation_binary_10),
	]) {
		return Some(node);
	}

	return expression_11(parser);
}

pub fn expression_13<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION, vec![
		&Next::Function(&operation_binary_11),
	]) {
		return Some(node);
	}

	return expression_12(parser);
}

pub fn expression_14<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION, vec![
		&Next::Function(&operation_binary_12),
	]) {
		return Some(node);
	}

	return expression_13(parser);
}

pub fn expression<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return expression_14(parser);
}

pub fn expression_literal<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::PRODUCTION_EXPRESSION, vec![literal(parser)?]));
}

pub fn operation<'a, 'b>(parser: &mut Parser<'a, 'b, '_>, left: Node<'a, 'b>) -> Result<Node<'a, 'b>, ()> {
	let operator = parser.shift()?;
	match operator.element {
		&elements::SYMBOL_PLUS | &elements::SYMBOL_MINUS => {
			if let Ok(right) = _expression(parser) {
				return Ok(Node::new_production(&elements::PRODUCTION_OPERATION, vec![left, operator, right]));
			}
		},
		_ => (),
	}

	parser.back();
	return Err(());
}

pub fn _expression<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	let root = expression_literal(parser)?;
	if let Ok(operation) = operation(parser, root.clone()) {
		return Ok(Node::new_production(&elements::PRODUCTION_EXPRESSION, vec![operation]));
	}

	return Ok(root);
}
