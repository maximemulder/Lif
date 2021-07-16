mod code;
mod primitive;

use crate::runtime::data::{ Class, Tag };
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::scope::GcScope;
use crate::runtime::utilities::constructors::Constructors;
use crate::runtime::utilities::parameters;
use crate::runtime::value::GcValue;

pub use code::GenericCode;
pub use primitive::GenericPrimitive;

pub trait GenericImplementation<'a> {
    fn call(&self, engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a>;
}

pub struct Generic<'a> {
    tag: Tag,
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

    pub fn tag(&self) -> &Tag {
        &self.tag
    }

    pub fn scope(&self) -> GcScope<'a> {
        self.scope
    }
}

impl<'a> Generic<'a> {
    pub fn call(&mut self, engine: &mut Engine<'a>, generic: GcValue<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
        parameters::length(arguments.len(), self.parameters.len())?;
        if let Some(value) = self.constructors.get(engine, arguments) {
            return Ok(engine.new_reference(value));
        }

        let reference = engine.run_frame(self.scope, |engine| {
            for (parameter, argument) in self.parameters.iter().zip(arguments.iter().copied()) {
                let reference = engine.new_constant(argument);
                engine.set_variable(parameter, reference);
            }

            self.implementation.call(engine, arguments)
        })?;

        let values = Vec::from(arguments);
        let constructor = self.constructors.record(engine, generic, values.into_boxed_slice(), reference.get_value());
        let mut value = reference.read()?;
        if value.class == engine.primitives.class {
            value.get_mut::<Class>(engine).set_constructor(constructor);
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
