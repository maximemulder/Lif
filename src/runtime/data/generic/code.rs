use crate::memory::Ref;
use crate::nodes::Executable;
use crate::runtime::data::generic::GenericImplementation;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::utilities::Arguments;

pub struct GenericCode {
    node: Ref<dyn Executable>,
}

impl GenericCode {
    pub fn new(node: Ref<dyn Executable>) -> Self {
        Self {
            node,
        }
    }
}

impl<'a> GenericImplementation<'a> for GenericCode {
    fn call(&self, engine: &mut Engine<'a>, _: Arguments<'a>) -> ReturnReference<'a> {
        Ok(engine.execute(Ref::as_ref(&self.node))?.none()?)
    }
}
