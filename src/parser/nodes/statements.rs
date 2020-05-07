use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::statement::statement;

pub fn statements<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return Some(Node::new_production(&elements::PRODUCTION_STATEMENTS, parser.commit_list(&Next::Production(&statement))));
}
