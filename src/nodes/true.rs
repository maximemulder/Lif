use crate::nodes::Executable;
use crate::runtime::engine::Engine;
use crate::runtime::utilities::ReturnReference;

pub struct True;

impl True {
    pub fn new() -> Self {
        Self
    }
}

impl Executable for True {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        Ok(engine.new_boolean(true))
    }
}
