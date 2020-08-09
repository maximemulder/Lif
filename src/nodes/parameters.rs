use super::token::Token;
use super::SyntaxNode;

pub struct Parameters;

impl Parameters {
	pub fn build(node: &SyntaxNode) -> Vec<Box<str>> {
		let mut identifiers = Vec::new();
		for (i, child) in node.children().iter().enumerate()  {
			if i % 2 == 1 {
				continue;
			}

			identifiers.push(Token::build(child));
		}

		return identifiers;
	}
}
