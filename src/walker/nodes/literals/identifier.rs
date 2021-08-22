use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnReference;
use crate::walker::nodes::ALiteralTrait;

pub struct AIdentifier {
    identifier: Ref<str>,
}

impl AIdentifier {
    pub fn new(identifier: Ref<str>) -> Self {
        Self {
            identifier,
        }
    }
}

impl ALiteralTrait for AIdentifier {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        engine.get_variable(&self.identifier)
    }
}
