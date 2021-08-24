use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnFlow, ReturnJump, ReturnReference };
use crate::walker::ANode;
use crate::walker::traits::{ WExecutable, WExpression, WStatement };

pub struct AExpression {
    expression: Box<ANode<dyn WExpression>>,
}

impl AExpression {
    pub fn new(expression: Box<ANode<dyn WExpression>>) -> Self {
        Self {
            expression,
        }
    }

    pub fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        engine.run_gc(|engine| self.expression.get().walk(engine))
    }
}

impl WExecutable for AExpression {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        self.walk(engine)?.get()
    }
}

impl WStatement for AExpression {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnJump<'a> {
        Jump::flow(self.walk(engine)?)
    }
}
