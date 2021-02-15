mod code;
mod primitive;

use crate::memory::Ref;
use crate::nodes::Executable;
use crate::runtime::data::Tag;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::scope::GcScope;
use crate::runtime::utilities::{ Callable, ReturnReference };
use crate::runtime::utilities::memoizes::Memoizes;
use crate::runtime::utilities::parameters;
use crate::runtime::value::GcValue;

use code::GenericImplementationCode;
use primitive::GenericImplementationPrimitive;

pub type GenericCode<'a>      = Generic<'a, GenericImplementationCode>;
pub type GenericPrimitive<'a> = Generic<'a, GenericImplementationPrimitive<'a>>;

pub trait GenericImplementation<'a> {
    fn length(&self) -> usize;
    fn call(&self, engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a>;
}

pub struct Generic<'a, T: GenericImplementation<'a>> {
    pub tag: Tag,
    scope: GcScope<'a>,
    memoizes: Memoizes<'a>,
    implementation: T,
}

impl<'a, T: GenericImplementation<'a>> Generic<'a, T> {
    fn new(tag: Tag, scope: GcScope<'a>, implementation: T) -> Self {
        Self {
            tag,
            scope,
            memoizes: Memoizes::new(),
            implementation
        }
    }
}

impl<'a> GenericCode<'a> {
    pub fn new_code(tag: Tag, scope: GcScope<'a>, parameters: Ref<[Ref<str>]>, node: Ref<dyn Executable>) -> Self {
        Self::new(tag, scope, GenericImplementationCode::new(parameters, node))
    }
}

impl<'a> GenericPrimitive<'a> {
    pub fn new_primitive(tag: Tag, scope: GcScope<'a>, parameters: Vec<Box<str>>, callback: &'a Callable<'a>) -> Self {
        Self::new(tag, scope, GenericImplementationPrimitive::new(parameters, callback))
    }
}

impl<'a, T: GenericImplementation<'a>> Generic<'a, T> {
    pub fn call(&mut self, engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
        parameters::length(arguments.len(), self.implementation.length())?;
        if let Some(reference) = self.memoizes.get(&arguments) {
            return Ok(reference);
        }

        let reference = engine.frame(self.scope, &|engine| self.implementation.call(engine, arguments.clone()))?;

        self.memoizes.record(arguments.into_boxed_slice(), reference);
        Ok(reference)
    }
}

impl<'a, T: GenericImplementation<'a>> GcTrace for Generic<'a, T> {
    fn trace(&mut self) {
        self.scope.trace();
        self.memoizes.trace();
    }
}
