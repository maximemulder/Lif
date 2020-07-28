#![allow(unused_variables)]
#![allow(dead_code)]

pub mod rule;
pub mod nodes;
pub mod arena;

use crate::node::Node;
use arena::Arena;

pub struct Parser<'a, 'b, 'c> {
	tokens: &'c Vec<Node<'a, 'b>>,
	rules: &'c Arena<'a, 'c>,
	cursor: usize,
}

impl<'a, 'b, 'c> Parser<'a, 'b, 'c> {
	fn new(tokens: &'c Vec<Node<'a, 'b>>, rules: &'c Arena<'a, 'c>) -> Self {
		return Self {
			tokens,
			rules,
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

	fn rule(&mut self, index: usize) -> Option<Vec<Node<'a, 'b>>> {
		let cursor = self.cursor;
		let nodes = self.rules.get(index).rule(self);
		if nodes.is_none() {
			self.cursor = cursor;
		}

		return nodes;
	}
}
