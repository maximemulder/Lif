mod nodes;

use crate::element::Element;
use crate::token::Token;
use crate::tree::{ Child, Tree };
use nodes::program::program;

pub fn run<'a, 'b>(tokens: &'a Vec<Token<'b>>) -> Option<Tree<'a, 'b>> {
	return program(&mut Parser::new(tokens));
}

pub struct Parser<'a, 'b> {
	tokens: &'a Vec<Token<'b>>,
	cursor: usize,
}

pub enum Content<'a, 'b, 'c> {
	Token(&'c Element),
	Production(&'c dyn Fn(&mut Parser<'a, 'b>) -> Option<Tree<'a, 'b>>),
}

impl<'a, 'b> Parser<'a, 'b> {
	pub fn new(tokens: &'a Vec<Token<'b>>) -> Self {
		return Self {
			tokens,
			cursor: 0,
		};
	}

	fn rollback(&mut self, children: Vec<Child<'a, 'b>>) {
		for child in children {
			self.cursor -= child.length();
		}
	}

	fn next<'c>(&mut self, content: &'c Content<'a, 'b, 'c>) -> Option<Child<'a, 'b>> {
		match content {
			Content::Token(element) => {
				if let Some(token) = self.tokens.get(self.cursor) {
					if &token.element == element {
						self.cursor += 1;
						return Some(Child::Token(token));
					}
				}
			},
			Content::Production(function) => {
				if let Some(tree) = function(self) {
					return Some(Child::Tree(tree));
				}
			},
		}

		return None;
	}

	fn commit<'c>(&mut self, contents: Vec<&'c Content<'a, 'b, 'c>>) -> Option<Vec<Child<'a, 'b>>> {
		let mut children = Vec::new();
		for content in contents {
			if let Some(child) = self.next(&content) {
				children.push(child);
			} else {
				self.rollback(children);
				return None;
			}
		}

		return Some(children);
	}

	fn commit_list<'c>(&mut self, content: &'c Content<'a, 'b, 'c>) -> Vec<Child<'a, 'b>> {
		let mut children = Vec::new();
		loop {
			if let Some(child) = self.next(content) {
				children.push(child);
			} else {
				break;
			}
		}

		return children;
	}
}
