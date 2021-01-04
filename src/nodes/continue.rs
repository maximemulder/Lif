use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        engine.control_new(Control::Continue, self.expression.as_ref())
    }
}
