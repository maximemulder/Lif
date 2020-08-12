use crate::nodes::{ Node, SyntaxNode };
use crate::runtime::{ Engine, Reference };

pub struct String {
	string: Box<str>,
}

impl String {
	pub fn build(node: &SyntaxNode) -> Self {
		let text = node.text();
		return Self {
			string: Box::from(&text[1 .. text.len() - 1]),
		};
	}
}

impl Node for String {
	fn execute<'a>(&'a self, engine: &Engine<'a>) -> Reference {
		return engine.new_string(self.string.to_string());
	}
}
