use crate::nodes::Node;
use crate::nodes::block::Block;
use crate::nodes::declaration::Declaration;
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };
use crate::runtime::error::Error;
use crate::runtime::gc::GcTraceable;
use crate::runtime::scope::GcScope;
use crate::runtime::value::GcValue;

pub trait Callable<'a, 'b>: GcTraceable {
	fn execute(&self, engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b>;
	fn duplicate<'slf>(&'slf self) -> Box<dyn Callable<'a, 'b> + 'slf>;
}

#[derive(Clone)]
pub struct Primitive<'a, 'b> {
	callback: &'b dyn Fn(&mut Engine<'a, 'b>, Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b>,
}

impl<'a, 'b> Primitive<'a, 'b> {
	pub fn new(callback: &'b dyn Fn(&mut Engine<'a, 'b>, Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b>) -> Self {
		return Self {
			callback,
		};
	}
}

impl<'a, 'b> Callable<'a, 'b> for Primitive<'a, 'b> {
	fn execute(&self, engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
		return (self.callback)(engine, arguments);
	}

	fn duplicate<'slf>(&'slf self) -> Box<dyn Callable<'a, 'b> + 'slf> {
		return Box::new(self.clone());
	}
}

impl GcTraceable for Primitive<'_, '_> {
	fn trace(&mut self) {}
}

#[derive(Clone)]
pub struct Function<'a, 'b> {
	scope: GcScope<'a, 'b>,
	generics: &'b Vec<&'a str>,
	parameters: &'b Vec<Declaration<'a>>,
	r#type: Option<GcValue<'a, 'b>>,
	block: &'b Block<'a>,
}

impl<'a, 'b> Function<'a, 'b> {
	pub fn new(scope: GcScope<'a, 'b>, generics: &'b Vec<&'a str>, parameters: &'b Vec<Declaration<'a>>, r#type: Option<GcValue<'a, 'b>>, block: &'b Block<'a>) -> Self {
		return Self {
			scope,
			generics,
			parameters,
			r#type,
			block,
		};
	}
}

impl<'a, 'b> Callable<'a, 'b> for Function<'a, 'b> {
	fn execute(&self, engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
		engine.push_frame(self.scope);
		for (parameter, argument) in self.generics.iter().zip(&arguments[..self.generics.len()]) {
			engine.add_constant_value(parameter, *argument);
		}

		for (parameter, argument) in self.parameters.iter().zip(arguments) {
			let mut reference = parameter.execute(engine)?;
			reference.write(argument)?;
		}

		let reference = self.block.execute(engine)?;
		engine.pop_frame();

		if engine.control_is(Control::Break) || engine.control_is(Control::Continue) {
			return Err(Error::new_runtime("Trying to loop control out of a function."));
		}

		if engine.control_consume(Control::Return) {
			if let Some(r#type) = self.r#type {
				let value = reference.read()?;
				value.cast(r#type)?;
				return Ok(engine.new_constant(value));
			}

			if reference.is_defined() {
				return Ok(engine.new_constant(reference.get_value()));
			}
		}

		return Ok(engine.undefined());
	}

	fn duplicate<'slf>(&'slf self) -> Box<dyn Callable<'a, 'b> + 'slf> {
		return Box::new(self.clone());
	}
}

impl GcTraceable for Function<'_, '_> {
	fn trace(&mut self) {
		self.scope.trace();
	}
}
