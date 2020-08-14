use crate::nodes::block::Block;
use crate::nodes::Node;
use crate::runtime::{ Engine, Reference };
use dyn_clone::DynClone;

pub trait Callable<'a> : DynClone {
	fn call(&self, engine: &Engine<'a>, arguments: Vec<Reference>) -> Reference;
}

#[derive(Clone)]
pub struct Primitive<'a, 'b> {
	callback: &'b dyn Fn(&Engine<'a>, Vec<Reference>) -> Reference,
}

impl<'a, 'b> Primitive<'a, 'b> {
	pub fn new(callback: &'b dyn Fn(&Engine<'a>, Vec<Reference>) -> Reference) -> Self {
		return Self {
			callback,
		};
	}
}

impl<'a, 'b> Callable<'a> for Primitive<'a, 'b> {
	fn call(&self, engine: &Engine<'a>, arguments: Vec<Reference>) -> Reference {
		return (self.callback)(engine, arguments);
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
