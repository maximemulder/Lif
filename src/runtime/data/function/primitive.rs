use crate::runtime::data::function::FunctionImplementation;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::utilities::Callable;
use crate::runtime::utilities::parameters::Parameters;
use crate::runtime::value::GcValue;

pub struct FunctionPrimitive<'a> {
    callback: &'a Callable<'a>,
}

impl<'a> FunctionPrimitive<'a> {
    pub fn new(callback: &'a Callable<'a>) -> Self {
        Self {
            callback,
        }
    }
}

impl<'a> FunctionImplementation<'a> for FunctionPrimitive<'a> {
    fn call(&self, engine: &mut Engine<'a>, _: &Parameters<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
        (self.callback)(engine, arguments)
    }
}
