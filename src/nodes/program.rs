use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnFlow;

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
        engine.execute(&self.statements)?;
        Ok(engine.undefined())
    }
}
