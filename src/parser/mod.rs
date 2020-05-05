use crate::elements;
use crate::token::Token;
use crate::tree::{ Child, Tree };

fn cancel<'a, 'b>(tokens: &mut Vec<&'a Token<'b>>, children: Vec<Child<'a, 'b>>) {
	for child in children {
		match child {
			Child::Token(token) => {tokens.insert(0, token);},
			Child::Tree(tree) => {tokens.splice(..0, tree.tokens());},
		}
	}
}

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
			cancel(tokens, children);
			return None;
		}

		return Some(Tree::new(&elements::PRODUCTION_STATEMENT, children));
	}

	return None;
}

fn expression<'a, 'b>(tokens: &Vec<&'a Token<'b>>) -> Option<Tree<'a, 'b>> {
	return None;
}
/*

fn statement<'a>(tokens: &Vec<Token<'a>>) -> Option<Tree<'a>> {
	if let Some(expression) = expression(tokens) {
		let children = vec![Child::Tree(expression)];
		let token = tokens.first();
		if token.is_none() {
			return None;
		}

		if token.unwrap().element == &elements::SYMBOL_SEMICOLON {
			children.push(Child::Token(tokens.remove(0)));
		} else {
			tokens.splice(1..1, expression.tokens());
		}

		return Some(Tree::new(&elements::PRODUCTION_STATEMENT, children));
	}

	return None;
}

fn expression<'a, 'b>(tokens: &Vec<Token<'a>>) -> Option<Tree<'a, 'b>> {
	return None;
}
*/
/*struct Production {

}*/

/*pub fn expression(tokens: &Vec<Token>) -> Option<Production> {
	let token = tokens.first();
	if token.is_none() {
		return None;
	}

	match {

	}
}*/
/*
pub fn root(tokens: &Vec<Token>) {
	let token = tokens.first();
	if token.is_none() {
		return;
	}

	let element = token.unwrap().element;

	match element {
		&elements::NUMBER => number(tokens),
		_ => return,
	}
}

fn number(tokens: &Vec<Token>) {
	expression(tokens);
}

fn expression(tokens: &Vec<Token>) {
	let token = tokens.first();
	if token.is_none() {
		return;
	}

	let element = token.unwrap().element;

	match element {
		&elements::SYMBOL_PLUS => expression_addition(tokens),
		_ => return,
	}
}

fn expression_addition(tokens: &Vec<Token>) {
	expression(tokens)
}
*/