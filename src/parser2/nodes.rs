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

pub struct Arena<'a, 'b> {
	none: &'b dyn Matcher<'a>,
	matchers: Vec<&'b dyn Matcher<'a>>,
}

impl<'a, 'b> Arena<'a, 'b> {
	fn new(none: &'b dyn Matcher<'a>) -> Self {
		return Self {
			none,
			matchers: Vec::new(),
		};
	}

	fn insert(&mut self, index: usize, matcher: &'b dyn Matcher<'a>) {
		while self.matchers.len() <= index {
			self.matchers.push(self.none);
		}

		self.matchers[index] = matcher;
	}

	pub fn get(&self, index: usize) -> &'b dyn Matcher<'a> {
		return self.matchers[index];
	}
}


pub fn run<'a, 'b>(tokens: &Vec<Node<'a, 'b>>) -> Option<Node<'a, 'b>> {
	let none = token(&elements::ignores::WHITESPACE);
	let mut matchers = Arena::new(&none);
	macro_rules! rule {
		( $index:expr, $rule:expr ) => {
			let variable = $rule;
			matchers.insert($index, &variable);
		};
	}

	rule!(PROGRAM_ELEMENT, element(STATEMENTS_ELEMENT, &elements::productions::PROGRAM));
	rule!(STATEMENTS_ELEMENT, element(STATEMENTS_LIST, &elements::productions::STATEMENTS));
	rule!(STATEMENTS_LIST, list(STATEMENT_ELEMENT));
	rule!(STATEMENT_ELEMENT, element(STATEMENT_SEQUENCE, &elements::productions::STATEMENT));
	rule!(STATEMENT_SEQUENCE, sequence(&[EXPRESSION_ELEMENT, SEMICOLON_TOKEN]));
	rule!(SEMICOLON_TOKEN, token(&elements::symbols::SEMICOLON));
	rule!(EXPRESSION_ELEMENT, element(EXPRESSION_CHOICE, &elements::productions::EXPRESSION));
	rule!(EXPRESSION_CHOICE, choice(&[STRUCTURE_ELEMENT, LITERAL_ELEMENT]));
	rule!(STRUCTURE_ELEMENT, element(STRUCTURE_CHOICE, &elements::structures::STRUCTURE));
	rule!(STRUCTURE_CHOICE, choice(&[IF_ELEMENT]));
	rule!(IF_ELEMENT, element(IF_SEQUENCE, &elements::structures::IF));
	rule!(IF_SEQUENCE, sequence(&[IF_TOKEN, EXPRESSION_ELEMENT]));
	rule!(IF_TOKEN, token(&elements::keywords::IF));
	rule!(LITERAL_ELEMENT, element(LITERAL_CHOICE, &elements::expressions::LITERAL));
	rule!(LITERAL_CHOICE, choice(&[NUMBER_TOKEN, STRING_TOKEN, IDENTIFIER_TOKEN]));
	rule!(IDENTIFIER_TOKEN, token(&elements::variables::IDENTIFIER));
	rule!(STRING_TOKEN, token(&elements::variables::STRING));
	rule!(NUMBER_TOKEN, token(&elements::variables::NUMBER));

	return if let Some(mut nodes) = matchers.get(PROGRAM_ELEMENT).go(&mut Parser::new(tokens, &matchers)) {
		nodes.pop()
	} else {
		None
	};
}
