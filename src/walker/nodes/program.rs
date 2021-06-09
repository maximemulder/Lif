use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ Executable, Node };

pub struct Program {
    statements: Node,
}

impl Program {
    pub fn new(statements: Node) -> Self {
        Self {
            statements,
        }
    }
}

impl Executable for Program {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        engine.execute(&self.statements)?.none()?;
        Flow::new(engine.undefined())
    }
}
