use crate::nodes::block::Block;
use crate::nodes::Node;
use crate::runtime::{ Engine, Reference };

pub trait Callable<'a> {
	fn call(&self, engine: &mut Engine<'a>, arguments: Vec<Reference>) -> Reference;
	fn duplicate(&self) -> Box<dyn Callable<'a> + 'a>;
}

#[derive(Clone)]
pub struct Primitive<'a> {
	callback: &'a dyn Fn(&mut Engine<'a>, Vec<Reference>) -> Reference,
}

impl<'a> Primitive<'a> {
	pub fn new(callback: &'a dyn Fn(&mut Engine<'a>, Vec<Reference>) -> Reference) -> Self {
		return Self {
			callback,
		};
	}
}

impl<'a> Callable<'a> for Primitive<'a> {
	fn call(&self, engine: &mut Engine<'a>, arguments: Vec<Reference>) -> Reference {
		return (self.callback)(engine, arguments);
	}

	fn duplicate(&self) -> Box<dyn Callable<'a> + 'a> {
		return Box::new(self.clone());
	}
}

#[derive(Clone)]
pub struct Function<'a> {
	scope: usize,
	parameters: &'a Vec<Box<str>>,
	block: &'a Block,
}

impl<'a> Function<'a> {
	pub fn new(scope: usize, parameters: &'a Vec<Box<str>>, block: &'a Block) -> Self {
		return Self {
			scope,
			parameters,
			block,
		};
	}
}

impl<'a> Callable<'a> for Function<'a> {
	fn call(&self, engine: &mut Engine<'a>, arguments: Vec<Reference>) -> Reference {
		let frame = engine.push_frame(self.scope);
		for (parameter, argument) in self.parameters.iter().zip(arguments) {
			let reference = engine.new_reference(engine.read(argument));
			engine.new_variable(&parameter, reference);
		}

		let reference = self.block.execute(engine);
		engine.pop_frame(frame);
		return reference;
	}

	fn duplicate(&self) -> Box<dyn Callable<'a> + 'a> {
		return Box::new(self.clone());
	}
}
