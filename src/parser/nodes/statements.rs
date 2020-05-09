use crate::elements;
use crate::node::Node;
use crate::parser::{ Next, Parser };

use super::statement::statement;

pub fn statements<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return parser.sequence(&elements::PRODUCTION_STATEMENTS, &Next::Function(&statement));
}
