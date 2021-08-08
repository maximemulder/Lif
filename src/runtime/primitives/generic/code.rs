use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::generic::GenericImplementation;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::value::Value;
use crate::walker::WNode;

pub struct GenericCode {
    node: Ref<WNode>,
}

impl GenericCode {
    pub fn new(node: Ref<WNode>) -> Self {
        Self {
            node,
        }
    }
}

impl<'a> GenericImplementation<'a> for GenericCode {
    fn call(&self, engine: &mut Engine<'a>, _: &mut [Value<'a>]) -> ReturnReference<'a> {
        engine.walk(Ref::as_ref(&self.node))?.none()
    }
}
