use crate::runtime::{ Engine, Reference };
use super::{ Node, SyntaxNode };
use super::token::token;

pub struct Declaration {
	identifier: Box<str>,
}

impl Declaration {
	pub fn build(node: &SyntaxNode) -> Declaration {
		return Declaration {
			identifier: token(&node.children()[1]),
		};
	}
}

impl Node for Declaration {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Reference {
		let reference = engine.new_undefined();
		engine.new_variable(&self.identifier, reference);
		return reference;
	}
}
