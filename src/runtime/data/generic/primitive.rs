use crate::runtime::data::generic::GenericImplementation;
use crate::runtime::engine::Engine;
use crate::runtime::utilities::{ Arguments, Callable, ReturnReference };

pub struct GenericImplementationPrimitive<'a> {
    callback: &'a Callable<'a>,
}

impl<'a> GenericImplementationPrimitive<'a> {
    pub fn new(callback: &'a Callable<'a>) -> Self {
        Self {
            callback,
        }
    }
}

impl<'a> GenericImplementation<'a> for GenericImplementationPrimitive<'a> {
    fn call(&self, engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
        (self.callback)(engine, arguments)
    }
}
