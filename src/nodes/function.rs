use crate::nodes::Node;
use crate::nodes::block::Block;
use crate::nodes::declaration::Declaration;
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

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
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
		let r#type = if let Some(r#type) = self.r#type.as_ref() {
			Some(r#type.execute(engine)?.read()?)
		} else {
			None
		};

		return Ok(engine.new_function(&self.parameters, r#type, &self.block));
	}
}
