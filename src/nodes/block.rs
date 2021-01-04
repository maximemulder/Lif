use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        engine.push_scope();
        execute!(engine, &self.statements);
        let reference = if let Some(expression) = &self.expression {
            execute!(engine, expression)
        } else {
            engine.undefined()
        };

        engine.pop_scope();
        Ok(reference)
    }
}
