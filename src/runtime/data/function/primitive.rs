use crate::runtime::data::function::FunctionImplementation;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::utilities::{ Arguments, Callable, ReturnReference };
use crate::runtime::utilities::parameters;
use crate::runtime::value::GcValue;

pub struct FunctionImplementationPrimitive<'a> {
    parameters: Box<[GcValue<'a>]>,
    callback: &'a Callable<'a>,
}

impl<'a> FunctionImplementationPrimitive<'a> {
    pub fn new(parameters: Box<[GcValue<'a>]>, callback: &'a Callable<'a>) -> Self {
        Self {
            parameters,
            callback,
        }
    }
}

impl<'a> FunctionImplementation<'a> for FunctionImplementationPrimitive<'a> {
    fn length(&self) -> usize {
        self.parameters.len()
    }

    fn call(&self, engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
        parameters::length(arguments.len(), self.parameters.len())?;
        for (parameter, argument) in self.parameters.iter().zip(arguments.as_ref()) {
            argument.cast(*parameter)?;
        }

        (self.callback)(engine, arguments)
    }
}

impl GcTrace for FunctionImplementationPrimitive<'_> {
    fn trace(&mut self) {
        for parameter in self.parameters.iter_mut() {
            parameter.trace();
        }
    }
}
