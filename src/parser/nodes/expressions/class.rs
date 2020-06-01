use crate::elements;
use crate::parser::Parser;
use crate::node::Node;

pub fn class<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::expressions::CLASS, vec![
		parser.token(&elements::keywords::CLASS)?,
		parser.token(&elements::symbols::BRACE_L)?,
		parser.token(&elements::symbols::BRACE_R)?,
	]));
}
