use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::utilities::{ Control, ReturnFlow };

pub struct Break {
    expression: Option<Node>,
}

impl Break {
    pub fn new(expression: Option<Node>) -> Self {
        Self {
            expression,
        }
    }
}

impl Executable for Break {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        engine.jump_new(Control::Break, self.expression.as_ref())
    }
}
