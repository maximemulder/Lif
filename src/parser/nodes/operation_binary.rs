use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::expression::*;
use super::operator_binary::*;

pub fn operation_binary_1<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.production(&elements::PRODUCTION_OPERATION, vec![
		&Next::Function(&expression_2),
		&Next::Function(&operator_binary_1),
		&Next::Function(&expression_3),
	]);
}

pub fn operation_binary_2<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.production(&elements::PRODUCTION_OPERATION, vec![
		&Next::Function(&expression_3),
		&Next::Function(&operator_binary_2),
		&Next::Function(&expression_4),
	]);
}

pub fn operation_binary_3<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.production(&elements::PRODUCTION_OPERATION, vec![
		&Next::Function(&expression_4),
		&Next::Function(&operator_binary_3),
		&Next::Function(&expression_5),
	]);
}

pub fn operation_binary_4<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.production(&elements::PRODUCTION_OPERATION, vec![
		&Next::Function(&expression_5),
		&Next::Function(&operator_binary_4),
		&Next::Function(&expression_6),
	]);
}

pub fn operation_binary_5<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.production(&elements::PRODUCTION_OPERATION, vec![
		&Next::Function(&expression_6),
		&Next::Function(&operator_binary_5),
		&Next::Function(&expression_7),
	]);
}

pub fn operation_binary_6<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.production(&elements::PRODUCTION_OPERATION, vec![
		&Next::Function(&expression_7),
		&Next::Function(&operator_binary_6),
		&Next::Function(&expression_8),
	]);
}

pub fn operation_binary_7<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.production(&elements::PRODUCTION_OPERATION, vec![
		&Next::Function(&expression_8),
		&Next::Function(&operator_binary_7),
		&Next::Function(&expression_9),
	]);
}

pub fn operation_binary_8<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.production(&elements::PRODUCTION_OPERATION, vec![
		&Next::Function(&expression_9),
		&Next::Function(&operator_binary_8),
		&Next::Function(&expression_10),
	]);
}

pub fn operation_binary_9<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.production(&elements::PRODUCTION_OPERATION, vec![
		&Next::Function(&expression_10),
		&Next::Function(&operator_binary_9),
		&Next::Function(&expression_11),
	]);
}

pub fn operation_binary_10<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.production(&elements::PRODUCTION_OPERATION, vec![
		&Next::Function(&expression_11),
		&Next::Function(&operator_binary_10),
		&Next::Function(&expression_12),
	]);
}

pub fn operation_binary_11<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.production(&elements::PRODUCTION_OPERATION, vec![
		&Next::Function(&expression_12),
		&Next::Function(&operator_binary_11),
		&Next::Function(&expression_13),
	]);
}

pub fn operation_binary_12<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.production(&elements::PRODUCTION_OPERATION, vec![
		&Next::Function(&expression_13),
		&Next::Function(&operator_binary_12),
		&Next::Function(&expression_14),
	]);
}
