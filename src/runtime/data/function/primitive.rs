use crate::runtime::data::function::{ FunctionImplementation, Parameter };
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::utilities::{ Arguments, Callable, ReturnReference };

pub struct FunctionImplementationPrimitive<'a> {
    callback: &'a Callable<'a>,
}

impl<'a> FunctionImplementationPrimitive<'a> {
    pub fn new(callback: &'a Callable<'a>) -> Self {
        Self {
            callback,
        }
    }
}

impl<'a> FunctionImplementation<'a> for FunctionImplementationPrimitive<'a> {
    fn call(&self, engine: &mut Engine<'a>, _: &[Parameter<'a>], _: &Option<Parameter<'a>>, arguments: Arguments<'a>) -> ReturnReference<'a> {
        (self.callback)(engine, arguments)
    }
}

impl GcTrace for FunctionImplementationPrimitive<'_> {
    fn trace(&mut self) {}
}
