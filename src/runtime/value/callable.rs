use crate::runtime::Engine;

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

// use crate::nodes::block::Block;

pub struct Function {
	scope: usize,
	parameters: Vec<Box<str>>,
	// block: Block,
}

impl Function {
	pub fn new(scope: usize, parameters: Vec<Box<str>>/*, block: Block */) -> Self {
		return Self {
			scope,
			parameters,
			/* block, */
		};
	}
}

impl Callable for Function {
	fn call(&self, engine: &mut Engine, expressions: Vec<usize>) -> Option<usize> {
		return None;
	}
}
