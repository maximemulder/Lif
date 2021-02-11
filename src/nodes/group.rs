use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::utilities::ReturnReference;

pub struct Group {
    expression: Node,
}

impl Group {
    pub fn new(expression: Node) -> Self {
        Self {
            expression,
        }
    }
}

impl Executable for Group {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        engine.execute(&self.expression)
    }
}
