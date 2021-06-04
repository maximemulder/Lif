use crate::nodes::Executable;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnFlow;

pub struct False;

impl False {
    pub fn new() -> Self {
        Self
    }
}

impl Executable for False {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        Ok(engine.new_boolean(false))
    }
}
