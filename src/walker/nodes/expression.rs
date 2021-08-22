use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ ReturnFlow, ReturnReference };
use crate::walker::ANode;
use crate::walker::nodes::{ AExecutableTrait, AStatementTrait };

pub trait AExpressionTrait {
	fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a>;
}

pub struct AExpression {
	expression: Box<ANode<dyn AExpressionTrait>>,
}

impl AExpression {
	pub fn new(expression: Box<ANode<dyn AExpressionTrait>>) -> Self {
		Self {
			expression,
		}
	}

	pub fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
		engine.run_gc(|engine| self.expression.get().walk(engine))
	}
}

impl AExecutableTrait for AExpression {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        self.walk(engine)?.none()
    }
}

impl AStatementTrait for AExpression {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        self.walk(engine)
    }
}
