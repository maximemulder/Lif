mod node;

use crate::element::Element;
use crate::elements;
use crate::token::Token;
use crate::tree::{ Child, Tree };

pub fn run<'a, 'b>(tokens: &mut Vec<&'a Token<'b>>) -> Option<Tree<'a, 'b>> {
	return root(tokens);
}

enum Content<'a, 'b, 'c> {
	Token(&'c Element),
	Production(&'c dyn Fn(&mut Vec<&'a Token<'b>>) -> Option<Tree<'a, 'b>>),
}

fn rollback<'a, 'b>(tokens: &mut Vec<&'a Token<'b>>, children: Vec<Child<'a, 'b>>) {
	for child in children {
		match child {
			Child::Token(token) => {tokens.insert(0, token);},
			Child::Tree(tree) => {tokens.splice(..0, tree.tokens());},
		}
	}
}

fn commit<'a, 'b, 'c>(tokens: &mut Vec<&'a Token<'b>>, contents: Vec<Content<'a, 'b, 'c>>) -> Option<Vec<Child<'a, 'b>>> {
	let mut children = Vec::new();
	for content in contents.iter() {
		match content {
			Content::Token(element) => {
				let token = tokens.first();
				if token.is_none() || &token.unwrap().element != element {
					rollback(tokens, children);
					return None;
				}

				children.push(Child::Token(tokens.remove(0)));
			},
			Content::Production(function) => {
				let tree = function(tokens);
				if tree.is_none() {
					rollback(tokens, children);
					return None;
				}

				children.push(Child::Tree(tree.unwrap()));
			},
		}
	}

	return Some(children);
}

fn root<'a, 'b>(tokens: &mut Vec<&'a Token<'b>>) -> Option<Tree<'a, 'b>> {
	if let Some(children) = commit(tokens, vec![
		Content::Production(&statements),
	]) {
		return Some(Tree::new(&elements::PRODUCTION_ROOT, children));
	}

	return None;
}

fn statements<'a, 'b>(tokens: &mut Vec<&'a Token<'b>>) -> Option<Tree<'a, 'b>> {
	let mut children = Vec::new();
	while let Some(child) = statement(tokens) {
		children.push(Child::Tree(child));
	}

	return Some(Tree::new(&elements::PRODUCTION_STATEMENTS, children));
}

fn statement<'a, 'b>(tokens: &mut Vec<&'a Token<'b>>) -> Option<Tree<'a, 'b>> {
	if let Some(children) = commit(tokens, vec![
		Content::Production(&expression),
		Content::Token(&elements::SYMBOL_SEMICOLON),
	]) {
		return Some(Tree::new(&elements::PRODUCTION_STATEMENT, children));
	}

	return None;
}

fn expression<'a, 'b>(tokens: &mut Vec<&'a Token<'b>>) -> Option<Tree<'a, 'b>> {
	if tokens.len() > 0 {
		return Some(Tree::new(&elements::PRODUCTION_EXPRESSION, vec![Child::Token(tokens.remove(0))]));
	}

	return None;
}

/*
fn root<'a, 'b>(tokens: &mut Vec<&'a Token<'b>>) -> Option<Tree<'a, 'b>> {
	if let Some(statements) = statements(tokens) {
		return Some(Tree::new(&elements::PRODUCTION_ROOT, vec![
			Child::Tree(statements),
		]));
	}

	return None;
}

fn statements<'a, 'b>(tokens: &mut Vec<&'a Token<'b>>) -> Option<Tree<'a, 'b>> {
	let mut statements = Vec::new();
	while let Some(statement) = statement(tokens) {
		statements.push(Child::Tree(statement));
	}

	return Some(Tree::new(&elements::PRODUCTION_STATEMENTS, statements));
}

fn statement<'a, 'b>(tokens: &mut Vec<&'a Token<'b>>) -> Option<Tree<'a, 'b>> {
	let mut children = Vec::new();
	if let Some(expression) = expression(tokens) {
		children.push(Child::Tree(expression));
		let token = tokens.first();
		if token.is_none() {
			return None;
		}

		if token.unwrap().element == &elements::SYMBOL_SEMICOLON {
			children.push(Child::Token(tokens.remove(0)));
		} else {
			rollback(tokens, children);
			return None;
		}

		return Some(Tree::new(&elements::PRODUCTION_STATEMENT, children));
	}
	return commit(tokens, vec![Content::Production(&expression)]);

	return None;
}

fn expression<'a, 'b>(tokens: &Vec<&'a Token<'b>>) -> Option<Tree<'a, 'b>> {
	return None;
}

fn expression_import<'a, 'b>() {

}
*/