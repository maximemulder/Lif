use crate::runtime::data::generic::GenericImplementation;
use crate::runtime::engine::Engine;
use crate::runtime::utilities::{ Callable, ReturnReference };
use crate::runtime::value::GcValue;

pub struct GenericImplementationPrimitive<'a> {
    parameters: Vec<Box<str>>,
    callback: &'a Callable<'a>,
}

impl<'a> GenericImplementationPrimitive<'a> {
    pub fn new(parameters: Vec<Box<str>>, callback: &'a Callable<'a>) -> Self {
        Self {
            parameters,
            callback,
        }
    }
}

impl<'a> GenericImplementation<'a> for GenericImplementationPrimitive<'a> {
    fn length(&self) -> usize {
        self.parameters.len()
    }

    fn call(&self, engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
        for (parameter, argument) in self.parameters.iter().zip(arguments.iter()) {
            let reference = engine.new_constant(*argument);
            engine.add_variable(parameter, reference);
        }

        (self.callback)(engine, arguments.clone())
    }
}
