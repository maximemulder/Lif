use crate::memory::Ref;
use crate::nodes::Executable;
use crate::runtime::data::generic::GenericImplementation;
use crate::runtime::engine::Engine;
use crate::runtime::utilities::{ Arguments, ReturnReference };

pub struct GenericImplementationCode {
    node: Ref<dyn Executable>,
}

impl GenericImplementationCode {
    pub fn new(node: Ref<dyn Executable>) -> Self {
        Self {
            node,
        }
    }
}

impl<'a> GenericImplementation<'a> for GenericImplementationCode {
    fn call(&self, engine: &mut Engine<'a>, _: Arguments<'a>) -> ReturnReference<'a> {
        engine.execute(Ref::as_ref(&self.node))
    }
}
