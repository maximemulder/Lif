mod code;
mod primitive;

use crate::runtime::data::Tag;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::gc::GcTrace;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::scope::GcScope;
use crate::runtime::utilities::Arguments;
use crate::runtime::utilities::variable::Variable;
use crate::runtime::value::GcValue;

pub use code::FunctionCode;
pub use primitive::FunctionPrimitive;

pub trait FunctionImplementation<'a> {
    fn call(&self, engine: &mut Engine<'a>, parameters: &[Variable<'a>], rest: &Option<Variable<'a>>, arguments: Arguments<'a>) -> ReturnReference<'a>;
}

pub struct Function<'a> {
    tag: Tag,
    scope: GcScope<'a>,
    parameters: Box<[Variable<'a>]>,
    rest: Option<Variable<'a>>,
    r#return: Option<GcValue<'a>>,
    implementation: Box<dyn FunctionImplementation<'a> + 'a>,
}

impl<'a> Function<'a> {
    pub fn new(tag: Tag, scope: GcScope<'a>, parameters: Box<[Variable<'a>]>, rest: Option<Variable<'a>>, r#return: Option<GcValue<'a>>, implementation: impl FunctionImplementation<'a> + 'a) -> Self {
        Self {
            tag,
            scope,
            parameters,
            rest,
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
    pub fn call(&self, engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
        match &self.rest {
            Some(_) => if arguments.len() < self.parameters.len() {
                return Err(error_arguments(self.parameters.len(), arguments.len()));
            },
            None => if arguments.len() != self.parameters.len() {
                return Err(error_arguments(self.parameters.len(), arguments.len()));
            },
        }

        for (parameter, argument) in self.parameters.iter().zip(arguments.iter().copied()) {
            parameter.cast(argument)?;
        }

        let reference = engine.run_frame(self.scope, |engine| self.implementation.call(engine, &self.parameters, &self.rest, arguments.clone()))?;
        if let Some(r#return) = self.r#return {
            reference.read()?.cast(r#return)?;
        }

        Ok(reference)
    }
}

impl GcTrace for Function<'_> {
    fn trace(&mut self) {
        self.scope.trace();
        for parameter in self.parameters.iter_mut() {
            parameter.trace();
        }

        if let Some(rest) = self.rest.as_mut() {
            rest.trace();
        }

        if let Some(mut r#return) = self.r#return {
            r#return.trace();
        }
    }
}

fn error_arguments(parameters: usize, arguments: usize) -> Error {
    Error::new_runtime(&format!("Provided {} arguments while the function expects at least {} parameters.", arguments, parameters))
}
