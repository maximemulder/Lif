use crate::nodes::{ Node, Product };
use crate::runtime::engine::Engine;

pub struct String {
	string: Box<str>,
}

impl String {
	pub fn new(string: Box<str>) -> Self {
		return Self {
			string,
		};
	}
}

impl Node for String {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product<'a> {
		return Product::new(engine.new_string(self.string.to_string()));
	}
}
