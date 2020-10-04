use crate::nodes::Executable;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct String<'a> {
	string: &'a str,
}

impl<'a> String<'a> {
	pub fn new(string: &'a str) -> Self {
		return Self {
			string,
		};
	}
}

impl<'a> Executable<'a> for String<'a> {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		return Ok(engine.new_string(self.string.to_string()));
	}
}
