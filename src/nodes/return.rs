use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

pub struct Return<'a> {
    expression: Option<Node<'a>>}

impl<'a> Return<'a> {
    pub fn new(expression: Option<Node<'a>>) -> Self {
        return Self {
            expression,
        };
    }
}

impl<'a> Executable<'a> for Return<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        return engine.control_new(Control::Return, &self.expression);
    }
}
