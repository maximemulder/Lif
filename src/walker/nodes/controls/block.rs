use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::WNode;
use crate::walker::nodes::AControlTrait;

pub struct ABlock {
    statements: WNode,
    expression: Option<WNode>,
}

impl ABlock {
    pub fn new(statements: WNode, expression: Option<WNode>) -> Self {
        Self {
            statements,
            expression,
        }
    }
}

impl AControlTrait for ABlock {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        engine.run_scope(|engine| {
            get!(engine.walk(&self.statements)?);
            Flow::new(if let Some(expression) = self.expression.as_ref() {
                get!(engine.walk(expression)?)
            } else {
                engine.undefined()
            })
        })
    }
}
