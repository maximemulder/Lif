use crate::nodes::Executable;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };

pub struct True;

impl True {
    pub fn new() -> Self {
        Self
    }
}

impl Executable for True {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        Flow::new(engine.new_boolean(true))
    }
}
