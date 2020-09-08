#![allow(unused_variables)]
#![allow(dead_code)]

pub mod descent;
pub mod ascent;
pub mod nodes;
pub mod arena;

use crate::node::Node;
use arena::Arena;
use descent::Descent;
use ascent::Ascent;

pub struct Parser<'a, 'b> {
	text: &'b str,
	tokens: &'b Vec<Node<'a>>,
	descents: &'b Arena<dyn Descent<'a> + 'b>,
	ascents: &'b Arena<dyn Ascent<'a> + 'b>,
	cursor: usize,
	reach: usize,
}

impl<'a, 'b> Parser<'a, 'b> {
	fn new(text: &'b str, tokens: &'b Vec<Node<'a>>, descents: &'b Arena<dyn Descent<'a> + 'b>, ascents: &'b Arena<dyn Ascent<'a> + 'b>) -> Self {
		return Self {
			text,
			tokens,
			descents,
			ascents,
			cursor: 0,
			reach: 0,
		};
	}

	fn done(&self) -> bool {
		return self.cursor == self.tokens.len();
	}

	fn next(&mut self) -> Option<Node<'a>> {
		let option = self.tokens.get(self.cursor);
		if let Some(token) = option {
			if self.reach < self.cursor {
				self.reach = self.cursor;
			}

			self.cursor += 1;
			return Some(token.clone());
		}

		return None;
	}

	fn descent(&mut self, index: usize) -> Option<Vec<Node<'a>>> {
		let cursor = self.cursor;
		let nodes = self.descents.get(index).descent(self);
		if nodes.is_none() {
			self.cursor = cursor;
		}

		return nodes;
	}

	fn descent_predicate(&mut self, index: usize) -> bool {
		let cursor = self.cursor;
		let nodes = self.descents.get(index).descent(self);
		self.cursor = cursor;
		return nodes.is_some();
	}

	fn ascent(&mut self, index: usize, nodes: Vec<Node<'a>>) -> Option<Vec<Node<'a>>> {
		let cursor = self.cursor;
		let nodes = self.ascents.get(index).ascent(self, nodes);
		if nodes.is_none() {
			self.cursor = cursor;
		}

		return nodes;
	}

	fn ascent_predicate(&mut self, index: usize, nodes: Vec<Node<'a>>) -> bool {
		let cursor = self.cursor;
		let nodes = self.ascents.get(index).ascent(self, nodes);
		self.cursor = cursor;
		return nodes.is_some();
	}

	pub fn parse(&mut self, program: usize) -> Option<Node<'a>> {
		let node = if let Some(mut nodes) = self.descents.get(program).descent(self) {
			nodes.pop()
		} else {
			println!("PARSING ERROR");
			return None;
		};

		return if self.done() {
			node
		} else {
			let token = &self.tokens[self.reach];
			println!("PARSING ERROR, UNEXPECTED TOKEN: {:?} - {}", &self.text[token.left()..token.right()], token.element.name);
			None
		};
	}
}
