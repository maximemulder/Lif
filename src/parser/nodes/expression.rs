use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::group::group;
use super::literal::literal;
use super::operation_binary::*;

pub fn expression_1<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION, vec![
		&Next::Function(&group)
	]) {
		return Some(node);
	}

	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION, vec![
		&Next::Function(&literal),
	]) {
		return Some(node);
	}

	return None;
}

pub fn expression_2<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION,vec![
		&Next::Function(&operation_binary_1),
	]) {
		return Some(node);
	}

	return expression_1(parser);
}

pub fn expression_3<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION, vec![
		&Next::Function(&operation_binary_2),
	]) {
		return Some(node);
	}

	return expression_2(parser);
}

pub fn expression_4<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION, vec![
		&Next::Function(&operation_binary_3),
	]) {
		return Some(node);
	}

	return expression_3(parser);
}

pub fn expression_5<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION, vec![
		&Next::Function(&operation_binary_4),
	]) {
		return Some(node);
	}

	return expression_4(parser);
}

pub fn expression_6<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.production(&elements::PRODUCTION_EXPRESSION, vec![
		&Next::Function(&operation_binary_5),
	]) {
		return Some(node);
	}

	return expression_5(parser);
}
