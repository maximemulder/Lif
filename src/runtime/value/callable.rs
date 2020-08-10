use crate::runtime::Engine;
use crate::nodes::block::Block;
use crate::nodes::Node;

pub trait Callable {
	fn call(&self, engine: &mut Engine, expressions: Vec<usize>) -> Option<usize>;
}

pub struct Primitive<'a> {
	callback: &'a dyn for<'b> Fn(&'b Engine, Vec<usize>) -> Option<usize>,
}

impl<'a> Primitive<'a> {
	pub fn new(callback: &'a dyn for<'b> Fn(&'b Engine, Vec<usize>) -> Option<usize>) -> Self {
		return Self {
			callback,
		};
	}
}

impl Callable for Primitive<'_> {
	fn call(&self, engine: &mut Engine, expressions: Vec<usize>) -> Option<usize> {
		return (self.callback)(engine, expressions);
	}
}

pub struct Function<'a> {
	scope: usize,
	parameters: Vec<Box<str>>,
	block: &'a Block,
}

impl<'a> Function<'a> {
	pub fn new(scope: usize, parameters: Vec<Box<str>>, block: &'a Block) -> Self {
		return Self {
			scope,
			parameters,
			block,
		};
	}
}

impl Callable for Function<'_> {
	fn call(&self, engine: &mut Engine, expressions: Vec<usize>) -> Option<usize> {
		return self.block.execute(engine);
	}
}
