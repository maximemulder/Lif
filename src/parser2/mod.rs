#![allow(unused_variables)]
#![allow(dead_code)]

pub mod rules;
pub mod filters;
pub mod nodes;
pub mod arena;

use crate::node::Node;
use arena::Arena;
use rules::Rule;
use filters::Filter;

pub struct Parser<'a, 'b, 'c> {
	tokens: &'c Vec<Node<'a, 'b>>,
	rules: &'c Arena<dyn Rule<'a> + 'c>,
	filters: &'c Arena<dyn Filter<'a> + 'c>,
	cursor: usize,
}

impl<'a, 'b, 'c> Parser<'a, 'b, 'c> {
	fn new(tokens: &'c Vec<Node<'a, 'b>>, rules: &'c Arena<dyn Rule<'a> + 'c>, filters: &'c Arena<dyn Filter<'a> + 'c>) -> Self {
		return Self {
			tokens,
			rules,
			filters,
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

	fn filter(&mut self, index: usize, nodes: Vec<Node<'a, 'b>>) -> Vec<Node<'a, 'b>> {
		return self.filters.get(index).filter(self, nodes);
	}
}
