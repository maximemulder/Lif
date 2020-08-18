use crate::runtime::Engine;
use super::{ Node, Product };
use super::block::Block;

pub struct Function {
	parameters: Vec<Box<str>>,
	block: Block,
}

impl Function {
	pub fn new(parameters: Vec<Box<str>>, block: Block) -> Self {
		return Self {
			parameters,
			block,
		};
	}
}

impl Node for Function {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Product<'a> {
		return Product::new(engine.new_function(&self.parameters, &self.block));
	}
}
