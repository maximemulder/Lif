use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnReference;
use crate::walker::traits::WLiteral;

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

impl WLiteral for AIdentifier {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        engine.get_variable(&self.identifier)
    }
}
