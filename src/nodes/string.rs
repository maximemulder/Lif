use crate::nodes::Node;
use crate::runtime::ReturnReference;
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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		return Ok(engine.new_string(self.string.to_string()));
	}
}
