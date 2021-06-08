use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnFlow;

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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        engine.execute(&self.expression)
    }
}
