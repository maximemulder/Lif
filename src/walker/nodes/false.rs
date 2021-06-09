use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::Executable;

pub struct False;

impl False {
    pub fn new() -> Self {
        Self
    }
}

impl Executable for False {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        Flow::new(engine.new_boolean(false))
    }
}
