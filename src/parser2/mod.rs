#![allow(unused_variables)]
#![allow(dead_code)]

pub mod matcher;
pub mod nodes;

use crate::node::Node;
use nodes::Arena;

/* pub fn run<'a, 'b, 'c>(tokens: &'c Vec<Node<'a, 'b>>) -> Option<Node<'a, 'b>> {
	let mut parser = Parser::new(tokens);
	let tree = nodes::run(&mut parser);
	return if parser.done() {
		tree
	} else {
		None
	};
} */

pub struct Parser<'a, 'b, 'c> {
	tokens: &'c Vec<Node<'a, 'b>>,
	matchers: &'c Arena<'a, 'c>,
	cursor: usize,
}

impl<'a, 'b, 'c> Parser<'a, 'b, 'c> {
	fn new(tokens: &'c Vec<Node<'a, 'b>>, matchers: &'c Arena<'a, 'c>) -> Self {
		return Self {
			tokens,
			matchers,
			cursor: 0,
		};
	}

	fn done(&self) -> bool {
		return self.cursor == self.tokens.len();
	}

	fn save(&self) -> usize {
		return self.cursor;
	}

	fn restore(&mut self, cursor: usize) {
		self.cursor = cursor;
	}

	fn token(&mut self) -> Option<Node<'a, 'b>> {
		let option = self.tokens.get(self.cursor);
		if let Some(token) = option {
			return Some(token.clone());
		}

		return None;
	}

	fn advance(&mut self) {
		self.cursor += 1;
	}

	fn go(&mut self, index: usize) -> Option<Vec<Node<'a, 'b>>> {
		return self.matchers.get(index).go(self);
	}
}
