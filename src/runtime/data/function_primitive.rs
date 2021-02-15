use crate::runtime::data::Tag;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::utilities::{ Arguments, Callable, ReturnReference };
use crate::runtime::utilities::parameters;
use crate::runtime::value::GcValue;

pub struct FunctionPrimitive<'a> {
    pub tag: Tag,
    parameters: Box<[GcValue<'a>]>,
    callback: &'a Callable<'a>,
}

impl<'a> FunctionPrimitive<'a> {
    pub fn new(tag: Tag, parameters: Box<[GcValue<'a>]>, callback: &'a Callable<'a>) -> Self {
        Self {
            tag,
            parameters,
            callback,
        }
    }

    pub fn call(&self, engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
        parameters::length(arguments.len(), self.parameters.len())?;
        for (parameter, argument) in self.parameters.iter().zip(arguments.as_ref()) {
            argument.cast(*parameter)?;
        }

        (self.callback)(engine, arguments)
    }
}

impl GcTrace for FunctionPrimitive<'_> {
    fn trace(&mut self) {
        for parameter in self.parameters.iter_mut() {
            parameter.trace();
        }
    }
}
