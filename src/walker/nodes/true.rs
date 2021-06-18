use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::Walkable;

pub struct True;

impl True {
    pub fn new() -> Self {
        Self
    }
}

impl Walkable for True {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        Flow::new(engine.new_boolean(true))
    }
}
