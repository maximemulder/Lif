use crate::element::Element;
use crate::token::Token;

pub enum Child<'a, 'b> {
	Tree(Tree<'a, 'b>),
	Token(&'a Token<'b>),
}

pub struct Tree<'a, 'b> {
	pub element: &'static Element,
	pub children: Vec<Child<'a, 'b>>,
}

impl<'a, 'b> Tree<'a, 'b> {
	pub const fn new(element: &'static Element, children: Vec<Child<'a, 'b>>) -> Self {
		return Self {
			element,
			children,
		};
	}

	pub fn tokens(&self) -> Vec<&'a Token<'b>> {
		let mut tokens = Vec::new();
		for child in self.children.iter() {
			match child {
				Child::Token(token) => tokens.push(*token),
				Child::Tree(tree) => tokens.append(&mut tree.tokens()),
			}
		}

		return tokens;
	}
}
