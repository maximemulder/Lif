use crate::elements;
use crate::parser::{ Content, Parser };
use crate::tree::Tree;

use super::statements::statements;

pub fn program<'a, 'b>(parser: &mut Parser<'a, 'b>) -> Option<Tree<'a, 'b>> {
	if let Some(children) = parser.commit(vec![&Content::Production(&statements)]) {
		return Some(Tree::new(&elements::PRODUCTION_PROGRAM, children));
	}

	return None;
}
