mod code;
mod primitive;

use crate::runtime::data::Tag;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::scope::GcScope;
use crate::runtime::utilities::parameters::Parameters;
use crate::runtime::value::GcValue;

pub use code::FunctionCode;
pub use primitive::FunctionPrimitive;

pub trait FunctionImplementation<'a> {
    fn call(&self, engine: &mut Engine<'a>, parameters: &Parameters<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a>;
}

pub struct Function<'a> {
    tag: Tag,
    scope: GcScope<'a>,
    parameters: Parameters<'a>,
    r#return: Option<GcValue<'a>>,
    implementation: Box<dyn FunctionImplementation<'a> + 'a>,
}

impl<'a> Function<'a> {
    pub fn new(tag: Tag, scope: GcScope<'a>, parameters: Parameters<'a>, r#return: Option<GcValue<'a>>, implementation: impl FunctionImplementation<'a> + 'a) -> Self {
        Self {
            tag,
            scope,
            parameters,
            r#return,
            implementation: Box::new(implementation),
        }
    }

    pub fn tag(&self) -> &Tag {
        &self.tag
    }

    pub fn scope(&self) -> GcScope<'a> {
        self.scope
    }
}

impl<'a> Function<'a> {
    pub fn call(&self, engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
        self.parameters.validate(engine, arguments)?;
        let reference = engine.run_frame(self.scope, |engine| self.implementation.call(engine, &self.parameters, arguments))?;
        if let Some(r#return) = self.r#return {
            reference.read()?.cast(r#return)?;
        }

        Ok(reference)
    }
}

impl GcTrace for Function<'_> {
    fn trace(&mut self) {
        self.scope.trace();
        self.parameters.trace();
        if let Some(mut r#return) = self.r#return {
            r#return.trace();
        }
    }
}
