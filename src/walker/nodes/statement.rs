use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ Executable, Node };

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
        Flow::new(engine.undefined())
    }
}
