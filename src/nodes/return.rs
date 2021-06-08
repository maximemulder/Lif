use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnFlow };

pub struct Return {
    expression: Option<Node>}

impl Return {
    pub fn new(expression: Option<Node>) -> Self {
        Self {
            expression,
        }
    }
}

impl Executable for Return {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        engine.jump_new(Jump::Return, self.expression.as_ref())
    }
}
