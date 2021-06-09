use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnFlow };
use crate::walker::{ Executable, Node };
use crate::walker::utilities;

pub struct Continue {
    expression: Option<Node>,
}

impl Continue {
    pub fn new(expression: Option<Node>) -> Self {
        Self {
            expression,
        }
    }
}

impl Executable for Continue {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        utilities::new_jump(engine, Jump::Continue, self.expression.as_ref())
    }
}
