use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::expression::expression_1;
use super::expressions::expressions;

fn delimiter_l<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.token(&elements::SYMBOL_PARENTHESIS_L) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_CROTCHET_L) {
		return Some(node);
	}

	return None;
}

fn delimiter_r<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.token(&elements::SYMBOL_PARENTHESIS_R) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_CROTCHET_R) {
		return Some(node);
	}

	return None;
}

pub fn sequence<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.production(&elements::PRODUCTION_SEQUENCE, vec![
		&Next::Function(&expression_1),
		&Next::Function(&delimiter_l),
		&Next::Function(&expressions),
		&Next::Function(&delimiter_r),
	]);
}
