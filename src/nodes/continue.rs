use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

pub struct Continue<'a> {
    expression: Option<Node<'a>>,
}

impl<'a> Continue<'a> {
    pub fn new(expression: Option<Node<'a>>) -> Self {
        Self {
            expression,
        }
    }
}

impl<'a> Executable<'a> for Continue<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        engine.control_new(Control::Continue, &self.expression)
    }
}
