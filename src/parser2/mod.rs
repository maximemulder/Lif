#![allow(unused_variables)]
#![allow(dead_code)]

pub mod matcher;
pub mod nodes;
pub mod arena;

use crate::node::Node;
use arena::Arena;
use matcher::Matcher;

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
	matchers: Arena<&'c dyn Matcher<'a>>,
	cursor: usize,
}

impl<'a, 'b, 'c> Parser<'a, 'b, 'c> {
	fn new(tokens: &'c Vec<Node<'a, 'b>>, matchers: Arena<&'c dyn Matcher<'a>>) -> Self {
		return Self {
			tokens,
			matchers,
			cursor: 0,
		};
	}

	fn done(&self) -> bool {
		return self.cursor == self.tokens.len();
	}

	fn next(&mut self) -> Option<Node<'a, 'b>> {
		let option = self.tokens.get(self.cursor);
		if let Some(token) = option {
			self.cursor += 1;
			return Some(token.clone());
		}

		return None;
	}

	fn go(&mut self, index: usize) -> Option<Vec<Node<'a, 'b>>> {
		let cursor = self.cursor;
		let nodes = self.matchers.get(index).go(self);
		if nodes.is_none() {
			self.cursor = cursor;
		}

		return nodes;
	}
}
