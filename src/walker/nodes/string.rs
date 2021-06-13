use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::Walkable;

pub struct String {
    string: Ref<str>,
}

impl String {
    pub fn new(string: Ref<str>) -> Self {
        Self {
            string: Ref::new(&string[1 .. string.len() - 1]),
        }
    }
}

impl Walkable for String {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        Flow::new(engine.new_string(self.string.to_string()))
    }
}
