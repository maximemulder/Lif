use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::utilities::ReturnFlow;

pub struct Block {
    statements: Node,
    expression: Option<Node>,
}

impl Block {
    pub fn new(statements: Node, expression: Option<Node>) -> Self {
        Self {
            statements,
            expression,
        }
    }
}

impl Executable for Block {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        engine.run_scope(|engine| {
            engine.execute(&self.statements)?;
            Ok(if let Some(expression) = &self.expression {
                engine.execute(expression)?
            } else {
                engine.undefined()
            })
        })
    }
}
