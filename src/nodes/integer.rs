use crate::nodes::{ Node, SyntaxNode };
use crate::runtime::{ Engine, Object, Reference };

pub struct Integer {
	integer: usize,
}

impl Integer {
	pub fn build(node: &SyntaxNode) -> Self {
		return Self {
			integer: node.text().parse::<usize>().unwrap(),
		};
	}
}

impl Node for Integer {
	fn execute<'a>(&'a self, engine: &Engine<'a>) -> Reference {
		return engine.new_object(Object::new_integer(engine, self.integer));
	}
}
