use crate::memory::Ref;
use crate::nodes::Executable;
use crate::runtime::engine::Engine;
use crate::runtime::utilities::ReturnReference;

pub struct String {
    string: Ref<str>,
}

impl String {
    pub fn new(string: Ref<str>) -> Self {
        Self {
            string: Ref::from_ref(&string[1 .. string.len() - 1]),
        }
    }
}

impl Executable for String {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        Ok(engine.new_string(self.string.to_string()))
    }
}
