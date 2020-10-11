use crate::nodes::Executable;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct False;

impl False {
    pub fn new() -> Self {
        Self
    }
}

impl<'a> Executable<'a> for False {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        Ok(engine.new_boolean(false))
    }
}
