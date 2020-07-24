#![allow(unused_variables)]
#![allow(dead_code)]

pub mod matcher;
mod nodes;

use crate::node::Node;

pub fn run<'a, 'b, 'c>(tokens: &'c Vec<Node<'a, 'b>>) -> Option<Node<'a, 'b>> {
	return nodes::run(&mut Parser::new(tokens));
}

pub struct Parser<'a, 'b, 'c> {
	pub tokens: &'c Vec<Node<'a, 'b>>,
	pub cursor: usize,
}

impl<'a, 'b, 'c> Parser<'a, 'b, 'c> {
	fn new(tokens: &'c Vec<Node<'a, 'b>>) -> Self {
		return Self {
			tokens,
			cursor: 0,
		};
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
}
