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

pub trait FunctionImplementation<'a> {
    fn length(&self) -> usize;
    fn call(&self, engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a>;
}

pub struct Function<'a, T: FunctionImplementation<'a>> {
    pub tag: Tag,
    r#type: Option<GcValue<'a>>,
    implementation: T,
}

impl<'a, T: FunctionImplementation<'a>> Function<'a, T> {
    pub fn new(tag: Tag, r#type: Option<GcValue<'a>>, implementation: T) -> Self {
        Self {
            tag,
            r#type,
            implementation
        }
    }
}

impl<'a> FunctionCode<'a> {
    pub fn new_code(tag: Tag, r#type: Option<GcValue<'a>>, scope: GcScope<'a>, parameters: Ref<[Node]>, block: Ref<Node>) -> Self {
        Self::new(tag, r#type, FunctionImplementationCode::new(scope, parameters, block))
    }
}

impl<'a> FunctionPrimitive<'a> {
    pub fn new_primitive(tag: Tag, r#type: Option<GcValue<'a>>, parameters: Box<[GcValue<'a>]>, callback: &'a Callable<'a>) -> Self {
        Self::new(tag, r#type, FunctionImplementationPrimitive::new(parameters, callback))
    }
}

impl<'a, T: FunctionImplementation<'a>> Function<'a, T> {
    pub fn call(&self, engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
        parameters::length(arguments.len(), self.implementation.length())?;
        let reference = self.implementation.call(engine, arguments.clone())?;
        if let Some(r#type) = self.r#type {
            reference.read()?.cast(r#type)?;
        }

        Ok(reference)
    }
}

impl<'a, T: FunctionImplementation<'a>> GcTrace for Function<'a, T> {
    fn trace(&mut self) {
        if let Some(mut r#type) = self.r#type {
            r#type.trace();
        }
    }
}
