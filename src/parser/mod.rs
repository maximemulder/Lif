mod nodes;

use crate::element::Element;
use crate::node::Node;
use nodes::program::program;

pub type Next<'a, 'b, 'c> = dyn Fn(&mut Parser<'a, 'b, 'c>) -> Result<Node<'a, 'b>, ()>;

pub fn run<'a, 'b, 'c>(tokens: &'c Vec<Node<'a, 'b>>) -> Result<Node<'a, 'b>, ()> {
	let mut parser = Parser::new(tokens);
	let node = program(&mut parser);
	return if parser.done() {
		Ok(node)
	} else {
		Err(())
	};
}

pub struct Parser<'a, 'b, 'c> {
	tokens: &'c Vec<Node<'a, 'b>>,
	cursor: usize,
}

impl<'a, 'b, 'c> Parser<'a, 'b, 'c> {
	fn new(tokens: &'c Vec<Node<'a, 'b>>) -> Self {
		return Self {
			tokens,
			cursor: 0,
		};
	}

	fn done(&self) -> bool {
		return self.cursor == self.tokens.len();
	}

	fn token(&mut self, element: &'a Element) -> Result<Node<'a, 'b>, ()> {
		if let Some(token) = self.tokens.get(self.cursor) {
			if token.element == element {
				self.cursor += 1;
				return Ok(token.clone());
			}
		}

		return Err(());
	}

	fn tokens(&mut self, elements: &[&'a Element]) -> Result<Node<'a, 'b>, ()> {
		for element in elements {
			if let Ok(token) = self.token(element) {
				return Ok(token);
			}
		}

		return Err(());
	}

	fn elements(&mut self, nexts: &[&Next<'a, 'b, 'c>]) -> Result<Vec<Node<'a, 'b>>, ()> {
		let cursor = self.cursor;
		let mut children = Vec::new();
		for next in nexts {
			if let Ok(child) = next(self) {
				children.push(child);
			} else {
				self.cursor = cursor;
				return Err(());
			}
		}

		return Ok(children);
	}
}
