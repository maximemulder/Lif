use crate::memory::Ref;
use crate::nodes::Executable;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };

pub struct Identifier {
    identifier: Ref<str>,
}

impl Identifier {
    pub fn new(identifier: Ref<str>) -> Self {
        Self {
            identifier,
        }
    }
}

impl Executable for Identifier {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        Flow::new(engine.get_variable(&self.identifier)?)
    }
}
