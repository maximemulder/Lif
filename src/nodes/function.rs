use super::{ Engine, Node, SyntaxNode };
use super::block::Block;
use super::parameters::Parameters;
use crate::runtime::Value;

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
	fn execute(&self, engine: &mut Engine) -> Option<usize> {
		return Some(engine.new_value(Value::new_function(engine, self.parameters, &self.block)));
		// return self.block.execute(engine);
	}
}
