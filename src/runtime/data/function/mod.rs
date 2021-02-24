mod code;
mod primitive;

use crate::memory::Ref;
use crate::nodes::Node;
use crate::runtime::data::Tag;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::scope::GcScope;
use crate::runtime::utilities::{ Arguments, Callable, ReturnReference };
use crate::runtime::utilities::parameters;
use crate::runtime::value::GcValue;

use code::FunctionImplementationCode;
use primitive::FunctionImplementationPrimitive;

pub type FunctionCode<'a>      = Function<'a, FunctionImplementationCode<'a>>;
pub type FunctionPrimitive<'a> = Function<'a, FunctionImplementationPrimitive<'a>>;

pub trait FunctionImplementation<'a>: GcTrace {
    fn call(&self, engine: &mut Engine<'a>, parameters: &[GcValue<'a>], arguments: Arguments<'a>) -> ReturnReference<'a>;
}

pub struct Function<'a, T: FunctionImplementation<'a>> {
    pub tag: Tag,
    parameters: Box<[GcValue<'a>]>,
    r#type: Option<GcValue<'a>>,
    implementation: T,
}

impl<'a, T: FunctionImplementation<'a>> Function<'a, T> {
    pub fn new(tag: Tag, parameters: Box<[GcValue<'a>]>, r#type: Option<GcValue<'a>>, implementation: T) -> Self {
        Self {
            tag,
            parameters,
            r#type,
            implementation
        }
    }
}

impl<'a> FunctionCode<'a> {
    pub fn new_code(tag: Tag, parameters: Box<[GcValue<'a>]>, names: Box<[Ref<str>]>, r#type: Option<GcValue<'a>>, scope: GcScope<'a>, block: Ref<Node>) -> Self {
        Self::new(tag, parameters, r#type, FunctionImplementationCode::new(scope, names, block))
    }
}

impl<'a> FunctionPrimitive<'a> {
    pub fn new_primitive(tag: Tag, parameters: Box<[GcValue<'a>]>, r#type: Option<GcValue<'a>>, callback: &'a Callable<'a>) -> Self {
        Self::new(tag, parameters, r#type, FunctionImplementationPrimitive::new(callback))
    }
}

impl<'a, T: FunctionImplementation<'a>> Function<'a, T> {
    pub fn call(&self, engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
        parameters::length(arguments.len(), self.parameters.len())?;
        for (parameter, argument) in self.parameters.iter().zip(arguments.as_ref()) {
            argument.cast(*parameter)?;
        }

        let reference = self.implementation.call(engine, &self.parameters, arguments.clone())?;
        if let Some(r#type) = self.r#type {
            reference.read()?.cast(r#type)?;
        }

        Ok(reference)
    }
}

impl<'a, T: FunctionImplementation<'a>> GcTrace for Function<'a, T> {
    fn trace(&mut self) {
        self.implementation.trace();
        for parameter in self.parameters.iter_mut() {
            parameter.trace();
        }

        if let Some(mut r#type) = self.r#type {
            r#type.trace();
        }
    }
}
