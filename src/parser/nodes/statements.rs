use crate::elements;
use crate::parser::{ Content, Parser };
use crate::tree::Tree;

use super::statement::statement;

pub fn statements<'a, 'b>(parser: &mut Parser<'a, 'b>) -> Option<Tree<'a, 'b>> {
	return Some(Tree::new(&elements::PRODUCTION_STATEMENTS, parser.commit_list(&Content::Production(&statement))));
}
