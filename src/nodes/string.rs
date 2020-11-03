use crate::nodes::Executable;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct String<'a> {
    string: &'a str,
}

impl<'a> String<'a> {
    pub fn new(string: &'a str) -> Self {
        Self {
            string: &string[1 .. string.len() - 1],
        }
    }
}

impl<'a> Executable<'a> for String<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        Ok(engine.new_string(self.string.to_string()))
    }
}
