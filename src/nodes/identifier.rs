use crate::nodes::Executable;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Identifier<'a> {
    identifier: &'a str,
}

impl<'a> Identifier<'a> {
    pub fn new(identifier: &'a str) -> Self {
        Self {
            identifier,
        }
    }
}

impl<'a> Executable<'a> for Identifier<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        engine.get_variable(&self.identifier)
    }
}
