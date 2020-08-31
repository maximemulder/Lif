use crate::nodes::Node;
use crate::nodes::block::Block;
use crate::runtime::engine::{ Control, Engine };
use crate::runtime::gc::GcTraceable;
use crate::runtime::reference::GcReference;
use crate::runtime::scope::GcScope;
use crate::runtime::value::GcValue;

pub trait Callable<'a>: GcTraceable {
	fn call(&self, engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> GcReference<'a>;
	fn duplicate(&self) -> Box<dyn Callable<'a> + 'a>;
}

#[derive(Clone)]
pub struct Primitive<'a> {
	callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> GcReference<'a>,
}

impl<'a> Primitive<'a> {
	pub fn new(callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> GcReference<'a>) -> Self {
		return Self {
			callback,
		};
	}
}

impl<'a> Callable<'a> for Primitive<'a> {
	fn call(&self, engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> GcReference<'a> {
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
	parameters: &'a Vec<Box<str>>,
	block: &'a Block,
}

impl<'a> Function<'a> {
	pub fn new(scope: GcScope<'a>, parameters: &'a Vec<Box<str>>, block: &'a Block) -> Self {
		return Self {
			scope,
			parameters,
			block,
		};
	}
}

impl<'a> Callable<'a> for Function<'a> {
	fn call(&self, engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> GcReference<'a> {
		let frame = engine.push_frame(self.scope);
		for (parameter, argument) in self.parameters.iter().zip(arguments) {
			let reference = engine.new_reference(Some(argument), true);
			engine.add_variable(&parameter, reference);
		}

		let mut reference = self.block.execute(engine);
		reference = match &engine.control {
			Some(control) => match control {
				Control::Break | Control::Continue => panic!(),
				Control::Return => reference,
			},
			None => engine.new_undefined(),
		};

		engine.pop_frame(frame);
		return reference;
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
