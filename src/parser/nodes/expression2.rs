use crate::elements;
use crate::parser::{ Content, Parser };
use crate::tree::Tree;

use super::expression1::expression1;

pub fn expression2<'a, 'b>(parser: &mut Parser<'a, 'b>) -> Option<Tree<'a, 'b>> {
	if let Some(children) = parser.commit(vec![
		&Content::Production(&expression1),
		&Content::Token(&elements::SYMBOL_PLUS),
		&Content::Production(&expression2),
	]) {
		return Some(Tree::new(&elements::PRODUCTION_EXPRESSION, children));
	}

	return expression1(parser);
}
