use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnFlow };
use crate::walker::{ Executable, Node };
use crate::walker::utilities;

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
        utilities::new_jump(engine, Jump::Break, self.expression.as_ref())
    }
}
