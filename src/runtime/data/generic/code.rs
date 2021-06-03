use crate::memory::Ref;
use crate::nodes::Executable;
use crate::runtime::data::generic::GenericImplementation;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::utilities::{ Arguments, Flow, ReturnReference };

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
        match engine.execute(Ref::as_ref(&self.node)) {
            Ok(reference)  => Ok(reference),
            Err(flow) => Err(match flow {
                Flow::Jump(_) => Error::new_jump(),
                Flow::Error(error) => error,
            }),
        }
    }
}
