use crate::nodes::Executable;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct True;

impl True {
	pub fn new() -> Self {
		return Self;
	}
}

impl<'a> Executable<'a> for True {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		return Ok(engine.new_boolean(true));
	}
}
