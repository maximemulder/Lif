use crate::nodes::Node;
use crate::nodes::block::Block;
use crate::nodes::declaration::Declaration;
use crate::nodes::expression::Expression;
use crate::runtime::engine::Engine;
use crate::runtime::reference::GcReference;

pub struct Function {
	parameters: Vec<Declaration>,
	r#type: Option<Expression>,
	block: Block,
}

impl Function {
	pub fn new(parameters: Vec<Declaration>, r#type: Option<Expression>, block: Block) -> Self {
		return Self {
			parameters,
			r#type,
			block,
		};
	}
}

impl Node for Function {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> GcReference<'a> {
		let r#type = self.r#type.as_ref().map(|r#type| r#type.execute(engine).read());
		return engine.new_function(&self.parameters, r#type, &self.block);
	}
}
