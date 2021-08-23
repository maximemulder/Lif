use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, Jump, JumpType, ReturnFlow };
use crate::walker::ANode;
use crate::walker::nodes::{ AExpression, AExpressionTrait };

use std::ops::Deref;

pub struct AJump {
	r#type: JumpType,
    expression: Option<ANode<AExpression>>
}

impl AJump {
	pub fn new(r#type: Ref<str>, expression: Option<ANode<AExpression>>) -> Self {
		Self {
			r#type: match r#type.deref() {
                "continue"  => JumpType::Continue,
				"break"     => JumpType::Break,
                "return"    => JumpType::Return,
                _ => panic!(),
            },
			expression,
		}
	}
}

impl AExpressionTrait for AJump {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
		let reference = if let Some(expression) = &self.expression {
			let value = flow!(expression.get().walk(engine)?).read()?;
			engine.new_constant(value)
		} else {
			engine.undefined()
		};

		Flow::jump(Jump::new(reference, self.r#type))
    }
}
