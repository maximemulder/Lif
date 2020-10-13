use crate::runtime::ReturnReference;
use crate::runtime::data::{ Callable, Tag };
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::gc::GcTraceable;
use crate::runtime::value::GcValue;

#[derive(Clone)]
pub struct Primitive<'a, 'b> {
	tag: Tag,
    parameters: Box<[GcValue<'a, 'b>]>,
    callback: &'b dyn Fn(&mut Engine<'a, 'b>, Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b>,
}

impl<'a, 'b> Primitive<'a, 'b> {
    pub fn new(tag: Tag, parameters: Box<[GcValue<'a, 'b>]>, callback: &'b dyn Fn(&mut Engine<'a, 'b>, Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b>) -> Self {
        Self {
			tag,
            parameters,
            callback,
        }
    }
}

impl<'a, 'b> Callable<'a, 'b> for Primitive<'a, 'b> {
    fn execute(&self, engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
        if arguments.len() != self.parameters.len() {
            return Err(Error::new_arguments(self.parameters.len(), arguments.len()));
        }

        for (parameter, argument) in self.parameters.iter().zip(&arguments) {
            argument.cast(*parameter)?;
        }

        (self.callback)(engine, arguments)
    }

    fn duplicate<'c>(&'c self) -> Box<dyn Callable<'a, 'b> + 'c> {
        Box::new(self.clone())
    }
}

impl GcTraceable for Primitive<'_, '_> {
    fn trace(&mut self) {}
}
