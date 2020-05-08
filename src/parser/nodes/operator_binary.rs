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

	if let Some(node) = parser.token(&elements::SYMBOL_ASTERISK_D) {
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

pub fn operator_binary_6<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.token(&elements::SYMBOL_AMPERSAND);
}

pub fn operator_binary_7<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.token(&elements::SYMBOL_CARET);
}

pub fn operator_binary_8<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.token(&elements::SYMBOL_PIPE);
}

pub fn operator_binary_9<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.token(&elements::SYMBOL_AMPERSAND_D);
}

pub fn operator_binary_10<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.token(&elements::SYMBOL_PIPE_D);
}

pub fn operator_binary_11<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.token(&elements::SYMBOL_DOT_D) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_DOT_D_EQ) {
		return Some(node);
	}

	return None;
}

pub fn operator_binary_12<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	if let Some(node) = parser.token(&elements::SYMBOL_EQUAL) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_PLUS_EQ) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_MINUS_EQ) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_ASTERISK_EQ) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_SLASH_EQ) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_PERCENT_EQ) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_ASTERISK_D_EQ) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_GUILLEMET_L_D_EQ) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_GUILLEMET_R_D_EQ) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_GUILLEMET_L_T_EQ) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_GUILLEMET_R_T_EQ) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_AMPERSAND_EQ) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_CARET_EQ) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_PIPE_EQ) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_AMPERSAND_D_EQ) {
		return Some(node);
	}

	if let Some(node) = parser.token(&elements::SYMBOL_PIPE_D_EQ) {
		return Some(node);
	}

	return None;
}
