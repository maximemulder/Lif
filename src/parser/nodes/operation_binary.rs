use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::expression::*;
use super::operator_binary::*;

pub fn operation_binary_1<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.production(&elements::PRODUCTION_OPERATION, vec![
		&Next::Function(&expression_1),
		&Next::Function(&operator_binary_1),
		&Next::Function(&expression_2),
	]);
}

pub fn operation_binary_2<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.production(&elements::PRODUCTION_OPERATION, vec![
		&Next::Function(&expression_2),
		&Next::Function(&operator_binary_2),
		&Next::Function(&expression_3),
	]);
}

pub fn operation_binary_3<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.production(&elements::PRODUCTION_OPERATION, vec![
		&Next::Function(&expression_3),
		&Next::Function(&operator_binary_3),
		&Next::Function(&expression_4),
	]);
}

pub fn operation_binary_4<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.production(&elements::PRODUCTION_OPERATION, vec![
		&Next::Function(&expression_4),
		&Next::Function(&operator_binary_4),
		&Next::Function(&expression_5),
	]);
}

pub fn operation_binary_5<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.production(&elements::PRODUCTION_OPERATION, vec![
		&Next::Function(&expression_5),
		&Next::Function(&operator_binary_5),
		&Next::Function(&expression_6),
	]);
}
