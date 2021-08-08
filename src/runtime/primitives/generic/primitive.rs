use crate::runtime::engine::Engine;
use crate::runtime::primitives::generic::GenericImplementation;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::utilities::Callable;
use crate::runtime::value::Value;

pub struct GenericPrimitive<'a> {
    callback: &'a Callable<'a>,
}

impl<'a> GenericPrimitive<'a> {
    pub fn new(callback: &'a Callable<'a>) -> Self {
        Self {
            callback,
        }
    }
}

impl<'a> GenericImplementation<'a> for GenericPrimitive<'a> {
    fn call(&self, engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
        (self.callback)(engine, arguments)
    }
}
