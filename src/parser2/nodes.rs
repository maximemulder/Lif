use crate::elements;
use crate::node::Node;
use crate::parser2::matcher::*;
use crate::parser2::Parser;

const NUMBER_TOKEN:       usize = 0;
const STRING_TOKEN:       usize = 1;
const IDENTIFIER_TOKEN:   usize = 2;
const LITERAL_CHOICE:     usize = 3;
const LITERAL_ELEMENT:    usize = 4;
const IF_TOKEN:           usize = 5;
const IF_SEQUENCE:        usize = 6;
const IF_ELEMENT:         usize = 7;
const STRUCTURE_CHOICE:   usize = 8;
const STRUCTURE_ELEMENT:  usize = 9;
const EXPRESSION_CHOICE:  usize = 10;
const EXPRESSION_ELEMENT: usize = 11;
const SEMICOLON_TOKEN:    usize = 12;
const STATEMENT_SEQUENCE: usize = 13;
const STATEMENT_ELEMENT:  usize = 14;
const STATEMENTS_LIST:    usize = 15;
const STATEMENTS_ELEMENT: usize = 16;
const PROGRAM_ELEMENT:    usize = 17;

pub fn run<'a, 'b>(tokens: &Vec<Node<'a, 'b>>) -> Option<Node<'a, 'b>> {
	let matchers: [&dyn Matcher; 18] = [
		&token(&elements::variables::NUMBER),
		&token(&elements::variables::STRING),
		&token(&elements::variables::IDENTIFIER),
		&choice(&[NUMBER_TOKEN, STRING_TOKEN, IDENTIFIER_TOKEN]),
		&element(LITERAL_CHOICE, &elements::expressions::LITERAL),
		&token(&elements::keywords::IF),
		&sequence(&[IF_TOKEN, EXPRESSION_ELEMENT]),
		&element(IF_SEQUENCE, &elements::structures::IF),
		&choice(&[IF_ELEMENT]),
		&element(STRUCTURE_CHOICE, &elements::structures::STRUCTURE),
		&choice(&[STRUCTURE_ELEMENT, LITERAL_ELEMENT]),
		&element(EXPRESSION_CHOICE, &elements::productions::EXPRESSION),
		&token(&elements::symbols::SEMICOLON),
		&sequence(&[EXPRESSION_ELEMENT, SEMICOLON_TOKEN]),
		&element(STATEMENT_SEQUENCE, &elements::productions::STATEMENT),
		&list(STATEMENT_ELEMENT),
		&element(STATEMENTS_LIST, &elements::productions::STATEMENTS),
		&element(STATEMENTS_ELEMENT, &elements::productions::PROGRAM),
	 ];

	return if let Some(mut nodes) = matchers[PROGRAM_ELEMENT].go(&mut Parser::new(tokens, &matchers)) {
		nodes.pop()
	} else {
		None
	};
}
