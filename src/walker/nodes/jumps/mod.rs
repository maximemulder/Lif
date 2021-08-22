mod r#continue;
mod r#break;
mod r#return;

pub use r#continue::AContinue;
pub use r#break::ABreak;
pub use r#return::AReturn;

use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, Jump, ReturnFlow };
use crate::walker::ANode;
use crate::walker::nodes::{ AExpression, AExpressionTrait };

pub trait AJumpTrait {
	fn jump(&self) -> Jump;
	fn expression(&self) -> Option<&ANode<AExpression>>;
}

pub struct AJump {
	jump: Box<ANode<dyn AJumpTrait>>,
}

impl AJump {
	pub fn new(jump: Box<ANode<dyn AJumpTrait>>) -> Self {
		Self {
			jump,
		}
	}
}

impl AExpressionTrait for AJump {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
		let reference = if let Some(node) = self.jump.get().expression() {
			let value = get!(node.get().walk(engine)?).read()?;
			engine.new_constant(value)
		} else {
			engine.undefined()
		};

		Flow::new_jump(reference, self.jump.get().jump())
    }
}
