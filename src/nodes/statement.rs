use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnFlow;

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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        get!(engine.execute(&self.node)?);
        Ok(flow!(engine.undefined()))
    }
}
