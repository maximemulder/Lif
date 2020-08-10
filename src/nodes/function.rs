use crate::runtime::{ Engine, Object, Reference };
use super::{ Node, SyntaxNode };
use super::block::Block;
use super::parameters::Parameters;

pub struct Function {
	parameters: Vec<Box<str>>,
	block: Block,
}

impl Function {
	pub fn build(node: &SyntaxNode) -> Function {
		return Function {
			parameters: Parameters::build(&node.children()[2]),
			block:      Block::build(&node.children()[4]),
		};
	}
}

impl Node for Function {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Reference {
		return engine.new_object(Object::new_function(engine, &self.parameters, &self.block));
	}
}
