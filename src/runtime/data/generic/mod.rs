mod code;
mod primitive;

use crate::runtime::data::Tag;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::scope::GcScope;
use crate::runtime::utilities::{ Arguments, ReturnReference };
use crate::runtime::utilities::constructors::Constructors;
use crate::runtime::utilities::parameters;
use crate::runtime::value::GcValue;

pub use code::GenericCode;
pub use primitive::GenericPrimitive;

pub trait GenericImplementation<'a> {
    fn call(&self, engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a>;
}

pub struct Generic<'a> {
    pub tag: Tag,
    scope: GcScope<'a>,
    constructors: Constructors<'a>,
    parameters: Box<[Box<str>]>,
    implementation: Box<dyn GenericImplementation<'a> + 'a>,
}

impl<'a> Generic<'a> {
    pub fn new(tag: Tag, scope: GcScope<'a>, parameters: Box<[Box<str>]>, implementation: impl GenericImplementation<'a> + 'a) -> Self {
        Self {
            tag,
            scope,
            constructors: Constructors::new(),
            parameters,
            implementation: Box::new(implementation),
        }
    }
}

impl<'a> Generic<'a> {
    pub fn call(&mut self, engine: &mut Engine<'a>, generic: GcValue<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
        parameters::length(arguments.len(), self.parameters.len())?;
        if let Some(value) = self.constructors.get(&arguments) {
            return Ok(engine.new_reference(value));
        }

        let reference = engine.frame(self.scope, &|engine| {
            for (parameter, argument) in self.parameters.iter().zip(arguments.iter().copied()) {
                let reference = engine.new_constant(argument);
                engine.add_variable(parameter, reference);
            }

            self.implementation.call(engine, arguments.clone())
        })?;


        let constructor = self.constructors.record(engine, generic, arguments, reference.get_value());
        let mut value = reference.read()?;
        if value.class == engine.primitives.class {
            value.data_class_mut().constructor = Some(constructor);
        }

        Ok(reference)
    }
}

impl GcTrace for Generic<'_> {
    fn trace(&mut self) {
        self.scope.trace();
        self.constructors.trace();
    }
}
