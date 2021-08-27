use crate::memory::Ref;
use crate::parser::CNode;
use crate::parser::elements;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnReference;
use crate::walker::ANode;
use crate::walker::traits::WLiteral;

pub struct ABoolean {
    boolean: bool,
}

impl ABoolean {
    pub fn new(boolean: bool) -> Self {
        Self {
            boolean,
        }
    }
}

impl ANode for ABoolean {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(match node.element {
            &elements::keywords::TRUE => true,
            &elements::keywords::FALSE => false,
            _ => panic!(),
        })
    }
}

impl WLiteral for ABoolean {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        Ok(engine.new_boolean(self.boolean))
    }
}
