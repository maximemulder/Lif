use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::utilities::ReturnReference;

pub struct Statement {
    node: Node,
}

impl Statement {
    pub fn new(node: Node) -> Self {
        Self {
            node,
        }
    }
}

impl Executable for Statement {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        execute!(engine, &self.node);
        Ok(engine.undefined())
    }
}
