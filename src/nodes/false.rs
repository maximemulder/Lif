use crate::nodes::Executable;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct False;

impl False {
	pub fn new() -> Self {
		return Self;
	}
}

impl<'a> Executable<'a> for False {
	fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
		return Ok(engine.new_boolean(false));
	}
}
