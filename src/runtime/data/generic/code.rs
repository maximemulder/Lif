use crate::memory::Ref;
use crate::nodes::Executable;
use crate::runtime::data::generic::GenericImplementation;
use crate::runtime::engine::Engine;
use crate::runtime::utilities::{ Arguments, ReturnReference };

pub struct GenericImplementationCode {
    parameters: Ref<[Ref<str>]>,
    node: Ref<dyn Executable>,
}

impl GenericImplementationCode {
    pub fn new(parameters: Ref<[Ref<str>]>, node: Ref<dyn Executable>) -> Self {
        Self {
            parameters,
            node,
        }
    }
}

impl<'a> GenericImplementation<'a> for GenericImplementationCode {
    fn length(&self) -> usize {
        self.parameters.len()
    }

    fn call(&self, engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
        for (parameter, argument) in self.parameters.iter().zip(arguments.iter()) {
            let reference = engine.new_reference(*argument);
            engine.add_variable(parameter, reference);
        }

        engine.execute(Ref::as_ref(&self.node))
    }
}

