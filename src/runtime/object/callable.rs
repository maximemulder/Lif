use crate::nodes::block::Block;
use crate::nodes::Node;
use crate::runtime::{ Engine, Reference };

pub trait Callable<'a> {
	fn call(&self, engine: &Engine<'a>, arguments: Vec<Reference>) -> Reference;
}

pub struct Primitive<'a, 'b> {
	callback: &'b dyn for<'c> Fn(&'c Engine<'a>, Vec<Reference>) -> Reference,
}

impl<'a, 'b> Primitive<'a, 'b> {
	pub fn new(callback: &'b dyn for<'c> Fn(&'c Engine<'a>, Vec<Reference>) -> Reference) -> Self {
		return Self {
			callback,
		};
	}
}

impl<'a> Callable<'a> for Primitive<'a, '_> {
	fn call(&self, engine: &Engine<'a>, arguments: Vec<Reference>) -> Reference {
		return (self.callback)(engine, arguments);
	}
}

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
	fn call(&self, engine: &Engine<'a>, arguments: Vec<Reference>) -> Reference {
		let frame = engine.push_frame(self.scope);
		for (parameter, argument) in self.parameters.iter().zip(arguments) {
			engine.new_variable(&parameter, engine.new_reference(engine.read(argument)));
		}

		let reference = self.block.execute(engine);
		engine.pop_frame(frame);
		return reference;
	}
}
