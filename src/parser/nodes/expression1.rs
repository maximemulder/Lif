use crate::elements;
use crate::parser::{ Content, Parser };
use crate::tree::Tree;

pub fn expression1<'a, 'b>(parser: &mut Parser<'a, 'b>) -> Option<Tree<'a, 'b>> {
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
