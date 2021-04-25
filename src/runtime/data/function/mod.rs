mod code;
mod primitive;

use crate::runtime::data::Tag;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::gc::GcTrace;
use crate::runtime::scope::GcScope;
use crate::runtime::utilities::{ Arguments, ReturnReference };
use crate::runtime::utilities::variable::Variable;
use crate::runtime::value::GcValue;

pub use code::FunctionCode;
pub use primitive::FunctionPrimitive;

pub trait FunctionImplementation<'a>: GcTrace {
    fn call(&self, engine: &mut Engine<'a>, parameters: &[Variable<'a>], rest: &Option<Variable<'a>>, arguments: Arguments<'a>) -> ReturnReference<'a>;
}

pub struct Function<'a> {
    pub tag: Tag,
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
}

/* impl<'a> FunctionCode<'a> {
    pub fn new_code(tag: Tag, scope: GcScope<'a>, parameters: Box<[Variable<'a>]>, rest: Option<Variable<'a>>, r#return: Option<GcValue<'a>>, block: Ref<Node>) -> Self {
        Self::new(tag, scope, parameters, rest, r#return, FunctionImplementationCode::new(block))
    }
}

impl<'a> FunctionPrimitive<'a> {
    pub fn new_primitive(tag: Tag, scope: GcScope<'a>, parameters: Box<[Variable<'a>]>, rest: Option<Variable<'a>>, r#return: Option<GcValue<'a>>, callback: &'a Callable<'a>) -> Self {
        Self::new(tag, scope, parameters, rest, r#return, FunctionImplementationPrimitive::new(callback))
    }
} */

impl<'a> Function<'a> {
    pub fn call(&self, engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
        match &self.rest {
            Some(_) => if arguments.len() < self.parameters.len() {
                return Err(Error::new_arguments(self.parameters.len(), arguments.len()));
            },
            None => if arguments.len() != self.parameters.len() {
                return Err(Error::new_arguments(self.parameters.len(), arguments.len()));
            },
        }

        for (parameter, argument) in self.parameters.iter().zip(arguments.iter().copied()) {
            parameter.cast(argument)?;
        }

        let reference = engine.frame(self.scope, &|engine| self.implementation.call(engine, &self.parameters, &self.rest, arguments.clone()))?;

        if let Some(r#return) = self.r#return {
            reference.read()?.cast(r#return)?;
        }

        Ok(reference)
    }
}

impl GcTrace for Function<'_> {
    fn trace(&mut self) {
        self.implementation.trace();
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
