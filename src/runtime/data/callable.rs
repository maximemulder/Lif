use crate::nodes::Node;
use crate::nodes::block::Block;
use crate::nodes::declaration::Declaration;
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };
use crate::runtime::error::Error;
use crate::runtime::gc::GcTraceable;
use crate::runtime::scope::GcScope;
use crate::runtime::value::GcValue;

pub trait Callable<'a>: GcTraceable {
	fn execute(&self, engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a>;
	fn duplicate(&self) -> Box<dyn Callable<'a> + 'a>;
}

#[derive(Clone)]
pub struct Primitive<'a> {
	callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> ReturnReference<'a>,
}

impl<'a> Primitive<'a> {
	pub fn new(callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> ReturnReference<'a>) -> Self {
		return Self {
			callback,
		};
	}
}

impl<'a> Callable<'a> for Primitive<'a> {
	fn execute(&self, engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
		return (self.callback)(engine, arguments);
	}

	fn duplicate(&self) -> Box<dyn Callable<'a> + 'a> {
		return Box::new(self.clone());
	}
}

impl GcTraceable for Primitive<'_> {
	fn trace(&mut self) {}
}

#[derive(Clone)]
pub struct Function<'a> {
	scope: GcScope<'a>,
	parameters: &'a Vec<Declaration<'a>>,
	r#type: Option<GcValue<'a>>,
	block: &'a Block<'a>,
}

impl<'a> Function<'a> {
	pub fn new(scope: GcScope<'a>, parameters: &'a Vec<Declaration>, r#type: Option<GcValue<'a>>, block: &'a Block) -> Self {
		return Self {
			scope,
			parameters,
			r#type,
			block,
		};
	}
}

impl<'a> Callable<'a> for Function<'a> {
	fn execute(&self, engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
		engine.push_frame(self.scope);
		for (parameter, argument) in self.parameters.iter().zip(arguments) {
			let mut reference = parameter.execute(engine)?;
			reference.write(argument)?;
		}

		let reference = self.block.execute(engine)?;
		engine.pop_frame();

		return match &engine.control {
			Some(control) => match control {
				Control::Break | Control::Continue => Err(Error::new_runtime("Trying to loop control out of a function.")),
				Control::Return => {
					engine.control = None;
					let value = reference.read()?;
					if let Some(r#type) = self.r#type {
						value.cast(r#type)?;
					}

					Ok(engine.new_constant(Some(value)))
				},
			},
			None => Ok(engine.new_undefined()),
		};
	}

	fn duplicate(&self) -> Box<dyn Callable<'a> + 'a> {
		return Box::new(self.clone());
	}
}

impl GcTraceable for Function<'_> {
	fn trace(&mut self) {
		self.scope.trace();
	}
}
