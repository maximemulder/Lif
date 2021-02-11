use crate::runtime::data::Tag;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::scope::GcScope;
use crate::runtime::utilities::{ Callable, ReturnReference };
use crate::runtime::utilities::memoizes::Memoizes;
use crate::runtime::utilities::parameters;
use crate::runtime::value::GcValue;

pub struct GenericPrimitive<'a> {
    pub tag: Tag,
    scope: GcScope<'a>,
    parameters: Vec<Box<str>>,
    callback: &'a Callable<'a>,
    memoizes: Memoizes<'a>,
}

impl<'a> GenericPrimitive<'a> {
    pub fn new(tag: Tag, scope: GcScope<'a>, parameters: Vec<Box<str>>, callback: &'a Callable<'a>) -> Self {
        Self {
            tag,
            scope,
            parameters,
            callback,
            memoizes: Memoizes::new(),
        }
    }

    pub fn call(&mut self, engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
        parameters::length(arguments.len(), self.parameters.len())?;
        if let Some(reference) = self.memoizes.get(&arguments) {
            return Ok(reference);
        }

        let reference = engine.frame(self.scope, &|engine| {
            for (parameter, argument) in self.parameters.iter().zip(arguments.iter()) {
                let reference = engine.new_constant(*argument);
                engine.add_variable(parameter, reference);
            }

             (self.callback)(engine, arguments.clone())
        })?;

        self.memoizes.record(arguments.into_boxed_slice(), reference);
        Ok(reference)
    }
}

impl GcTrace for GenericPrimitive<'_> {
    fn trace(&mut self) {
        self.memoizes.trace();
    }
}
