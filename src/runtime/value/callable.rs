use crate::runtime::Engine;
use crate::nodes::block::Block;
use crate::nodes::Node;

pub trait Callable<'a> {
	fn call(&self, engine: &mut Engine<'a>, expressions: Vec<usize>) -> Option<usize>;
}

pub struct Primitive<'a, 'b> {
	callback: &'b dyn for<'c> Fn(&'c Engine<'a>, Vec<usize>) -> Option<usize>,
}

impl<'a, 'b> Primitive<'a, 'b> {
	pub fn new(callback: &'b dyn for<'c> Fn(&'c Engine<'a>, Vec<usize>) -> Option<usize>) -> Self {
		return Self {
			callback,
		};
	}
}

impl<'a> Callable<'a> for Primitive<'a, '_> {
	fn call(&self, engine: &mut Engine<'a>, expressions: Vec<usize>) -> Option<usize> {
		return (self.callback)(engine, expressions);
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
	fn call(&self, engine: &mut Engine<'a>, expressions: Vec<usize>) -> Option<usize> {
		let frame = engine.push_frame(self.scope);
		let value = self.block.execute(engine);
		engine.pop_frame(frame);
		return value;
	}
}
