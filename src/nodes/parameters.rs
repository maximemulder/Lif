use super::token::token;
use super::SyntaxNode;

pub fn parameters(node: &SyntaxNode) -> Vec<Box<str>> {
	let mut identifiers = Vec::new();
	for (i, child) in node.children().iter().enumerate()  {
		if i % 2 == 1 {
			continue;
		}

		identifiers.push(token(child));
	}

	return identifiers;
}
