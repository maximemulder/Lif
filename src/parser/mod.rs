mod node;

use crate::element::Element;
use crate::elements;
use crate::token::Token;
use crate::tree::{ Child, Tree };

pub fn run<'a, 'b>(tokens: &'a Vec<Token<'b>>) -> Option<Tree<'a, 'b>> {
	return root(&mut Parser::new(tokens));
}

struct Parser<'a, 'b, 'c> {
	tokens: Vec<&'a Token<'b>>,
	pub functions: Vec<&'c dyn Fn(&mut Parser<'a, 'b, 'c>) -> Option<Tree<'a, 'b>>>
}

enum Content<'a, 'b, 'c> {
	Token(&'c Element),
	Production(&'c dyn Fn(&mut Parser<'a, 'b, 'c>) -> Option<Tree<'a, 'b>>),
}

impl<'a, 'b, 'c> Parser<'a, 'b, 'c> {
	pub fn new(tokens: &'a Vec<Token<'b>>) -> Self {
		let mut references = Vec::new();
		for token in tokens.iter() {
			references.push(token);
		}

		return Self {
			tokens: references,
			functions: Vec::new(),
		};
	}

	fn execute(&mut self, function: &'c dyn Fn(&mut Parser<'a, 'b, 'c>) -> Option<Tree<'a, 'b>>) -> Option<Tree<'a, 'b>> {
		for i in 0..self.functions.len() {
			if self.functions[i] as *const dyn Fn(&mut Parser<'a, 'b, 'c>) -> Option<Tree<'a, 'b>> == function as *const dyn Fn(&mut Parser<'a, 'b, 'c>) -> Option<Tree<'a, 'b>> {
				return None;
			}
		}

		self.functions.push(function);
		return function(self);
	}

	fn rollback(&mut self, children: Vec<Child<'a, 'b>>) {
		for child in children {
			match child {
				Child::Token(token) => {self.tokens.insert(0, token);},
				Child::Tree(tree) => {self.tokens.splice(..0, tree.tokens());},
			}
		}
	}

	fn next(&mut self, content: &'c Content<'a, 'b, 'c>) -> Option<Child<'a, 'b>> {
		match content {
			Content::Token(element) => {
				let token = self.tokens.first();
				if token.is_none() || &token.unwrap().element != element {
					return None;
				}

				return Some(Child::Token(self.tokens.remove(0)));
			},
			Content::Production(function) => {
				let tree = self.execute(function);
				if tree.is_none() {
					return None;
				}

				return Some(Child::Tree(tree.unwrap()));
			},
		}
	}

	fn commit(&mut self, contents: Vec<&'c Content<'a, 'b, 'c>>) -> Option<Vec<Child<'a, 'b>>> {
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

	fn commit_list(&mut self, content: &'c Content<'a, 'b, 'c>) -> Vec<Child<'a, 'b>> {
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

fn root<'a, 'b, 'c>(parser: &mut Parser<'a, 'b, 'c>) -> Option<Tree<'a, 'b>> {
	if let Some(children) = parser.commit(vec![&Content::Production(&statements)]) {
		return Some(Tree::new(&elements::PRODUCTION_ROOT, children));
	}

	return None;
}

fn statements<'a, 'b, 'c>(parser: &mut Parser<'a, 'b, 'c>) -> Option<Tree<'a, 'b>> {
	return Some(Tree::new(&elements::PRODUCTION_STATEMENTS, parser.commit_list(&Content::Production(&statement))));
}

fn statement<'a, 'b, 'c>(parser: &mut Parser<'a, 'b, 'c>) -> Option<Tree<'a, 'b>> {
	if let Some(children) = parser.commit(vec![
		&Content::Production(&expression),
		&Content::Token(&elements::SYMBOL_SEMICOLON),
	]) {
		return Some(Tree::new(&elements::PRODUCTION_STATEMENT, children));
	}

	return None;
}

fn expression<'a, 'b, 'c>(parser: &mut Parser<'a, 'b, 'c>) -> Option<Tree<'a, 'b>> {
	if let Some(children) = parser.commit(vec![&Content::Production(&expression_addition)]) {
		return Some(Tree::new(&elements::PRODUCTION_EXPRESSION, children));
	}

	if let Some(children) = parser.commit(vec![&Content::Token(&elements::STRING)]) {
		return Some(Tree::new(&elements::PRODUCTION_EXPRESSION, children));
	}

	if let Some(children) = parser.commit(vec![&Content::Token(&elements::NUMBER)]) {
		return Some(Tree::new(&elements::PRODUCTION_EXPRESSION, children));
	}

	if let Some(children) = parser.commit(vec![&Content::Token(&elements::IDENTIFIER)]) {
		return Some(Tree::new(&elements::PRODUCTION_EXPRESSION, children));
	}

	return None;
}

fn expression_addition<'a, 'b, 'c>(parser: &mut Parser<'a, 'b, 'c>) -> Option<Tree<'a, 'b>> {
	if let Some(children) = parser.commit(vec![
		&Content::Production(&expression),
		&Content::Token(&elements::SYMBOL_PLUS),
		&Content::Production(&expression),
	]) {
		return Some(Tree::new(&elements::PRODUCTION_EXPRESSION, children));
	}

	return None;
}
