use crate::nodes::Node;
use crate::nodes::block::Block;
use crate::runtime::engine::Engine;
use crate::runtime::reference::GcReference;

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcReference<'a> {
		return engine.new_function(&self.parameters, &self.block);
	}
}
