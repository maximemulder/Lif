use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::generic::GenericImplementation;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::value::Value;
use crate::walker::ANode;
use crate::walker::nodes::AStructureTrait;

pub struct GenericCode {
    node: Ref<ANode<dyn AStructureTrait>>,
}

impl GenericCode {
    pub fn new(node: Ref<ANode<dyn AStructureTrait>>) -> Self {
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
