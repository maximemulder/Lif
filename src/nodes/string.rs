use crate::memory::Ref;
use crate::nodes::Executable;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnFlow;

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

impl Executable for String {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        Ok(flow!(engine.new_string(self.string.to_string())))
    }
}
