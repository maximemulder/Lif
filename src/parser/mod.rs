mod nodes;

use crate::element::Element;
use crate::node::Node;
use nodes::program::program;

pub fn run<'a, 'b, 'c>(tokens: &'c Vec<Node<'a, 'b>>) -> Option<Node<'a, 'b>> {
	return program(&mut Parser::new(tokens));
}

pub struct Parser<'a, 'b, 'c> {
	tokens: &'c Vec<Node<'a, 'b>>,
	cursor: usize,
}

pub enum Next<'a, 'b, 'c, 'd> {
	Element(&'a Element),
	Function(&'d dyn Fn(&mut Parser<'a, 'b, 'c>) -> Option<Node<'a, 'b>>),
}

impl<'a, 'b, 'c> Parser<'a, 'b, 'c> {
	pub fn new(tokens: &'c Vec<Node<'a, 'b>>) -> Self {
		return Self {
			tokens,
			cursor: 0,
		};
	}

	fn rollback(&mut self, children: Vec<Node<'a, 'b>>) {
		for child in children {
			self.cursor -= child.length();
		}
	}

	fn next(&mut self, next: &Next<'a, 'b, 'c, '_>) -> Option<Node<'a, 'b>> {
		return match next {
			Next::Element(element) => self.token(element),
			Next::Function(function) => function(self),
		};
	}

	fn token(&mut self, element: &'a Element) -> Option<Node<'a, 'b>> {
		if let Some(token) = self.tokens.get(self.cursor) {
			if token.element == element {
				self.cursor += 1;
				return Some(token.clone());
			}
		}

		return None;
	}

	fn production(&mut self, element: &'a Element, nexts: Vec<&Next<'a, 'b, 'c, '_>>) -> Option<Node<'a, 'b>> {
		let mut children = Vec::new();
		for next in nexts {
			if let Some(child) = self.next(&next) {
				children.push(child);
			} else {
				self.rollback(children);
				return None;
			}
		}

		return Some(Node::new_production(element, children));
	}

	fn commit_list(&mut self, next: &Next<'a, 'b, 'c, '_>) -> Vec<Node<'a, 'b>> {
		let mut children = Vec::new();
		while let Some(child) = self.next(next) {
			children.push(child);
		}

		return children;
	}
}
