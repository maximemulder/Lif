use crate::nodes::Node;
use crate::nodes::block::Block;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcRef;
use crate::runtime::reference::Reference;

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcRef<Reference<'a>> {
		return engine.new_function(&self.parameters, &self.block);
	}
}
