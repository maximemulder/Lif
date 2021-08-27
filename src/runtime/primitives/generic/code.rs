use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::generic::GenericImplementation;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::value::Value;
use crate::walker::SNode;
use crate::walker::traits::WDefinition;

pub struct GenericCode {
    node: Ref<SNode<dyn WDefinition>>,
}

impl GenericCode {
    pub fn new(node: Ref<SNode<dyn WDefinition>>) -> Self {
        Self {
            node,
        }
    }
}

impl<'a> GenericImplementation<'a> for GenericCode {
    fn call(&self, engine: &mut Engine<'a>, _: &mut [Value<'a>]) -> ReturnReference<'a> {
        self.node.get().walk(engine)
    }
}
