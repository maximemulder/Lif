use crate::element::Element;
use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::expressions::expressions;

fn new_expression<'a, 'b>(node: Node<'a, 'b>) -> Node<'a, 'b> {
	return Node::new_production(&elements::PRODUCTION_EXPRESSION, vec![node]);
}

fn operation_binary<'a, 'b, 'c>(
	parser: &mut Parser<'a, 'b, 'c>,
	operators: &[&'a Element],
	expression_left:  &Next<'a, 'b, 'c>,
	expression_right: &Next<'a, 'b, 'c>,
) -> Result<Node<'a, 'b>, ()> {
	let left = expression_left(parser)?;
	let node = parser.safe(&|parser| Ok(Node::new_production(&elements::PRODUCTION_OPERATION, vec![
		left.clone(),
		parser.tokens(&operators)?,
		expression_right(parser)?,
	])));

	if node.is_ok() {
		return node;
	}

	return Ok(left);
}

fn sequence<'a, 'b>(
	parser: &mut Parser<'a, 'b, '_>,
	delimiter_l: &'a Element,
	delimiter_r: &'a Element,
	expression: Node<'a, 'b>,
) -> Result<Node<'a, 'b>, ()> {
	let node = parser.safe(&|parser| Ok(Node::new_production(&elements::PRODUCTION_SEQUENCE, vec![
		expression.clone(),
		parser.token(delimiter_l)?,
		expressions(parser),
		parser.token(delimiter_r)?,
	])));

	if node.is_ok() {
		return node;
	}

	return Ok(expression);
}

const LITERALS: [&Element; 3] = [&elements::STRING, &elements::NUMBER, &elements::IDENTIFIER];

fn expression_1<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	if let Ok(group) = parser.safe(&|parser| Ok(Node::new_production(&elements::PRODUCTION_GROUP, vec![
		parser.token(&elements::SYMBOL_PARENTHESIS_L)?,
		expression(parser)?,
		parser.token(&elements::SYMBOL_PARENTHESIS_R)?,
	]))) {
		return Ok(new_expression(group));
	}

	return Ok(new_expression(
		Node::new_production(&elements::PRODUCTION_LITERAL, vec![parser.tokens(&LITERALS)?])
	));
}

fn expression_0<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	let mut expression;
	expression = expression_1(parser)?;
	expression = sequence(parser, &elements::SYMBOL_PARENTHESIS_L, &elements::SYMBOL_PARENTHESIS_R, expression)?;
	expression = sequence(parser, &elements::SYMBOL_CROTCHET_L,    &elements::SYMBOL_CROTCHET_R,    expression)?;
	return Ok(expression);
}

const OPERATORS_BINARY_1: [&Element; 4] = [
	&elements::SYMBOL_ASTERISK,
	&elements::SYMBOL_SLASH,
	&elements::SYMBOL_PERCENT,
	&elements::SYMBOL_ASTERISK_D,
];

fn expression_2<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_1, &expression_0, &expression_2);
}

const OPERATORS_BINARY_2: [&Element; 2] = [&elements::SYMBOL_PLUS, &elements::SYMBOL_MINUS];

fn expression_3<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_2, &expression_2, &expression_3);
}

const OPERATORS_BINARY_3: [&Element; 4] = [
	&elements::SYMBOL_GUILLEMET_L_D,
	&elements::SYMBOL_GUILLEMET_R_D,
	&elements::SYMBOL_GUILLEMET_L_T,
	&elements::SYMBOL_GUILLEMET_R_T,
];

fn expression_4<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_3, &expression_3, &expression_4);
}

const OPERATORS_BINARY_4: [&Element; 4] = [
	&elements::SYMBOL_GUILLEMET_L,
	&elements::SYMBOL_GUILLEMET_R,
	&elements::SYMBOL_GUILLEMET_L_EQ,
	&elements::SYMBOL_GUILLEMET_R_EQ,
];

fn expression_5<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_4, &expression_4, &expression_5);
}

const OPERATORS_BINARY_5: [&Element; 2] = [
	&elements::SYMBOL_EQUAL_D,
	&elements::SYMBOL_EXCLAMATION_EQ,
];

fn expression_6<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_5, &expression_5, &expression_6);
}

const OPERATORS_BINARY_6: [&Element; 1] = [&elements::SYMBOL_AMPERSAND];

fn expression_7<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_3, &expression_6, &expression_7);
}

const OPERATORS_BINARY_7: [&Element; 1] = [&elements::SYMBOL_CARET];

fn expression_8<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_3, &expression_7, &expression_8);
}

const OPERATORS_BINARY_8: [&Element; 1] = [&elements::SYMBOL_PIPE];

fn expression_9<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_3, &expression_8, &expression_9);
}

const OPERATORS_BINARY_9: [&Element; 1] = [&elements::SYMBOL_AMPERSAND_D];

fn expression_10<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_3, &expression_9, &expression_10);
}

const OPERATORS_BINARY_10: [&Element; 1] = [&elements::SYMBOL_PIPE_D];

fn expression_11<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_3, &expression_10, &expression_11);
}

const OPERATORS_BINARY_11: [&Element; 2] = [&elements::SYMBOL_DOT_D, &elements::SYMBOL_DOT_D_EQ];

fn expression_12<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_3, &expression_11, &expression_12);
}

const OPERATORS_BINARY_12: [&Element; 16] = [
	&elements::SYMBOL_EQUAL,
	&elements::SYMBOL_PLUS_EQ,
	&elements::SYMBOL_MINUS_EQ,
	&elements::SYMBOL_ASTERISK_EQ,
	&elements::SYMBOL_SLASH_EQ,
	&elements::SYMBOL_PERCENT_EQ,
	&elements::SYMBOL_ASTERISK_D_EQ,
	&elements::SYMBOL_GUILLEMET_L_D_EQ,
	&elements::SYMBOL_GUILLEMET_R_D_EQ,
	&elements::SYMBOL_GUILLEMET_L_T_EQ,
	&elements::SYMBOL_GUILLEMET_R_T_EQ,
	&elements::SYMBOL_AMPERSAND_EQ,
	&elements::SYMBOL_CARET_EQ,
	&elements::SYMBOL_PIPE_EQ,
	&elements::SYMBOL_AMPERSAND_D_EQ,
	&elements::SYMBOL_PIPE_D_EQ,
];

fn expression_13<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return operation_binary(parser, &OPERATORS_BINARY_3, &expression_12, &expression_13);
}

pub fn expression<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return expression_13(parser);
}
