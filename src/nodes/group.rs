use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Group<'a> {
    expression: Node<'a>,
}

impl<'a> Group<'a> {
    pub fn new(expression: Node<'a>) -> Self {
        return Self {
            expression,
        };
    }
}

impl<'a> Executable<'a> for Group<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        return engine.execute(&self.expression);
    }
}
