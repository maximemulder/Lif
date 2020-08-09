use super::token::Token;
use super::SyntaxNode;

pub struct Parameters {
	identifiers: Vec<Box<str>>,
}

impl Parameters {
	pub fn build(node: &SyntaxNode) -> Parameters {
		let mut identifiers = Vec::new();
		for (i, child) in node.children().iter().enumerate()  {
			if i % 2 == 1 {
				continue;
			}

			identifiers.push(Token::build(child));
		}

		return Parameters {
			identifiers,
		};
	}
}
