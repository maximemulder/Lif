use crate::elements;
use crate::node::Node;
use crate::parser::Parser;

use super::super::expression::expression;

fn structure_block_body<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Vec<Node<'a, 'b>> {
	let mut body = Vec::new();
	let mut statements = Vec::new();
	while let Ok(expression) = expression(parser) {
		if let Ok(semicolon) = parser.token(&elements::symbols::SEMICOLON) {
			statements.push(Node::new_production(&elements::productions::STATEMENT, vec![
				expression,
				semicolon,
			]))
		} else {
			body.push(expression);
			break;
		}
	}

	body.insert(0, Node::new_production(&elements::productions::STATEMENTS, statements));
	return body;
}

pub fn structure_block<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Result<Node<'a, 'b>, ()> {
	return Ok(Node::new_production(&elements::structures::BLOCK, {
		let mut children = Vec::new();
		children.push(parser.token(&elements::symbols::BRACE_L)?);
		children.append(&mut structure_block_body(parser));
		children.push(parser.token(&elements::symbols::BRACE_R)?);
		children
	}));
}
