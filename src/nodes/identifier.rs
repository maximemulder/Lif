use crate::nodes::{ Node, SyntaxNode };
use crate::runtime::{ Engine, Reference };

pub struct Identifier {
	identifier: Box<str>,
}

impl Identifier {
	pub fn build(node: &SyntaxNode) -> Self {
		return Self {
			identifier: Box::from(node.text()),
		};
	}
}

impl Node for Identifier {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Reference {
		return engine.get_variable(&self.identifier);
	}
}
