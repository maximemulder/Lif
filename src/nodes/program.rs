use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        execute!(engine, &self.statements);
        Ok(engine.undefined())
    }
}
