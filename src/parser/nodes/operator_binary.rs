use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

pub fn operator_binary_1<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.token(&elements::SYMBOL_ASTERISK) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_SLASH) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_PERCENT) {
		return Some(node);
	}

	return None;
}

pub fn operator_binary_2<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.token(&elements::SYMBOL_PLUS) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_MINUS) {
		return Some(node);
	}

	return None;
}

pub fn operator_binary_3<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.token(&elements::SYMBOL_GUILLEMET_L_D) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_GUILLEMET_R_D) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_GUILLEMET_L_T) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_GUILLEMET_R_T) {
		return Some(node);
	}

	return None;
}

pub fn operator_binary_4<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.token(&elements::SYMBOL_GUILLEMET_L) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_GUILLEMET_R) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_GUILLEMET_L_EQ) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_GUILLEMET_R_EQ) {
		return Some(node);
	}

	return None;
}

pub fn operator_binary_5<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.token(&elements::SYMBOL_EQUAL) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_EXCLAMATION_EQ) {
		return Some(node);
	}

	return None;
}
