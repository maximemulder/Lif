use crate::elements;
use crate::parser::{ Content, Parser };
use crate::tree::Tree;

use super::expression2::expression2;

pub fn statement<'a, 'b>(parser: &mut Parser<'a, 'b>) -> Option<Tree<'a, 'b>> {
	if let Some(children) = parser.commit(vec![
		&Content::Production(&expression2),
		&Content::Token(&elements::SYMBOL_SEMICOLON),
	]) {
		return Some(Tree::new(&elements::PRODUCTION_STATEMENT, children));
	}

	return None;
}
