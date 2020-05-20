mod nodes;

use crate::element::Element;
use crate::node::Node;
use nodes::program::program;

pub fn run<'a, 'b, 'c>(tokens: &'c Vec<Node<'a, 'b>>) -> Result<Node<'a, 'b>, ()> {
	return Ok(program(&mut Parser::new(tokens)));
}

pub struct Parser<'a, 'b, 'c> {
	tokens: &'c Vec<Node<'a, 'b>>,
	cursor: usize,
}

impl<'a, 'b, 'c> Parser<'a, 'b, 'c> {
	pub fn new(tokens: &'c Vec<Node<'a, 'b>>) -> Self {
		return Self {
			tokens,
			cursor: 0,
		};
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

	fn back(&mut self) {
		self.cursor -= 1;
	}
}
