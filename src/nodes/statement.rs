use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Statement<'a> {
    node: Node<'a>,
}

impl<'a> Statement<'a> {
    pub fn new(node: Node<'a>) -> Self {
        return Self {
            node,
        };
    }
}

impl<'a> Executable<'a> for Statement<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        execute!(engine, &self.node);
        return Ok(engine.undefined());
    }
}
