use crate::runtime::ReturnReference;
use crate::runtime::data::Callable;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::gc::GcTraceable;
use crate::runtime::value::GcValue;

#[derive(Clone)]
pub struct Primitive<'a, 'b> {
	parameters: Box<[GcValue<'a, 'b>]>,
	callback: &'b dyn Fn(&mut Engine<'a, 'b>, Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b>,
}

impl<'a, 'b> Primitive<'a, 'b> {
	pub fn new(parameters: Box<[GcValue<'a, 'b>]>, callback: &'b dyn Fn(&mut Engine<'a, 'b>, Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b>) -> Self {
		return Self {
			parameters,
			callback,
		};
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

		return (self.callback)(engine, arguments);
	}

	fn duplicate<'c>(&'c self) -> Box<dyn Callable<'a, 'b> + 'c> {
		return Box::new(self.clone());
	}
}

impl GcTraceable for Primitive<'_, '_> {
	fn trace(&mut self) {}
}
