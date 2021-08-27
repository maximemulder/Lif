use crate::memory::Ref;
use crate::parser::CNode;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnReference;
use crate::walker::ANode;
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

impl ANode for AIdentifier {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(node.text())
    }
}

impl WLiteral for AIdentifier {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        engine.get_variable(&self.identifier)
    }
}
