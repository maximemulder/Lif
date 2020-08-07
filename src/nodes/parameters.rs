use super::identifier::Identifier;
use super::{ Engine, Node, SyntaxNode };

pub struct Parameters {
	identifiers: Vec<Identifier>,
}

impl Parameters {
	pub fn build(node: &SyntaxNode) -> Parameters {
		let mut identifiers = Vec::new();
		for (i, child) in node.children().iter().enumerate()  {
			if i % 2 == 1 {
				continue;
			}

			identifiers.push(Identifier::build(child));
		}

		return Parameters {
			identifiers,
		};
	}
}

impl Node for Parameters {
	fn execute(&self, engine: &mut Engine) {

	}
}
