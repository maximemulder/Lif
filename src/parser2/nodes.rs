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
		while self.matchers.len() < index {
			self.matchers.push(self.none);
		}

		self.matchers[index] = matcher;
	}

	pub fn get(&self, index: usize) -> &'b dyn Matcher<'a> {
		return self.matchers[index];
	}
}

pub fn run<'a, 'b>(tokens: &Vec<Node<'a, 'b>>) -> Option<Node<'a, 'b>> {
	let mut matchers = Arena::new(&token(&elements::ignores::WHITESPACE));
	let program = element(STATEMENTS_ELEMENT, &elements::productions::PROGRAM);
	matchers.insert(PROGRAM_ELEMENT, &program);
	matchers.insert(STATEMENTS_ELEMENT, &element(STATEMENTS_LIST, &elements::productions::STATEMENTS));
	matchers.insert(STATEMENTS_LIST, &list(STATEMENT_ELEMENT));
	matchers.insert(STATEMENT_ELEMENT, &element(STATEMENT_SEQUENCE, &elements::productions::STATEMENT));
	matchers.insert(STATEMENT_SEQUENCE, &sequence(&[EXPRESSION_ELEMENT, SEMICOLON_TOKEN]));
	matchers.insert(SEMICOLON_TOKEN, &token(&elements::symbols::SEMICOLON));
	matchers.insert(EXPRESSION_ELEMENT, &element(EXPRESSION_CHOICE, &elements::productions::EXPRESSION));
	matchers.insert(EXPRESSION_CHOICE, &choice(&[STRUCTURE_ELEMENT, LITERAL_ELEMENT]));
	matchers.insert(STRUCTURE_ELEMENT, &element(STRUCTURE_CHOICE, &elements::structures::STRUCTURE));
	matchers.insert(STRUCTURE_CHOICE, &choice(&[IF_ELEMENT]));
	matchers.insert(IF_ELEMENT, &element(IF_SEQUENCE, &elements::structures::IF));
	matchers.insert(IF_SEQUENCE, &sequence(&[IF_TOKEN, EXPRESSION_ELEMENT]));
	matchers.insert(IF_TOKEN, &token(&elements::keywords::IF));
	matchers.insert(LITERAL_ELEMENT, &element(LITERAL_CHOICE, &elements::expressions::LITERAL));
	matchers.insert(LITERAL_CHOICE, &choice(&[NUMBER_TOKEN, STRING_TOKEN, IDENTIFIER_TOKEN]));
	matchers.insert(IDENTIFIER_TOKEN, &token(&elements::variables::IDENTIFIER));
	matchers.insert(STRING_TOKEN, &token(&elements::variables::STRING));
	matchers.insert(NUMBER_TOKEN, &token(&elements::variables::NUMBER));

	return if let Some(mut nodes) = program.go(&mut Parser::new(tokens, &matchers)) {
		nodes.pop()
	} else {
		None
	};
}
