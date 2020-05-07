mod nodes;

use crate::element::Element;
use crate::token::Token;
use crate::tree::{ Child, Tree };
use nodes::program::program;

pub fn run<'a, 'b>(tokens: &'a Vec<Token<'b>>) -> Option<Tree<'a, 'b>> {
	return program(&mut Parser::new(tokens));
}

pub struct Parser<'a, 'b> {
	tokens: Vec<&'a Token<'b>>,
}

pub enum Content<'a, 'b, 'c> {
	Token(&'c Element),
	Production(&'c dyn Fn(&mut Parser<'a, 'b>) -> Option<Tree<'a, 'b>>),
}

impl<'a, 'b> Parser<'a, 'b> {
	pub fn new(tokens: &'a Vec<Token<'b>>) -> Self {
		let mut references = Vec::new();
		for token in tokens.iter() {
			references.push(token);
		}

		return Self {
			tokens: references,
		};
	}

	fn rollback(&mut self, children: Vec<Child<'a, 'b>>) {
		for child in children {
			match child {
				Child::Token(token) => {self.tokens.insert(0, token);},
				Child::Tree(tree) => {self.tokens.splice(..0, tree.tokens());},
			}
		}
	}

	fn next<'c>(&mut self, content: &'c Content<'a, 'b, 'c>) -> Option<Child<'a, 'b>> {
		match content {
			Content::Token(element) => {
				let token = self.tokens.first();
				if token.is_none() || &token.unwrap().element != element {
					return None;
				}

				return Some(Child::Token(self.tokens.remove(0)));
			},
			Content::Production(function) => {
				let tree = function(self);
				if tree.is_none() {
					return None;
				}

				return Some(Child::Tree(tree.unwrap()));
			},
		}
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
