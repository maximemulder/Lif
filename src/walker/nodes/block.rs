use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ Executable, Node };

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
            get!(engine.execute(&self.statements)?);
            Flow::new(if let Some(expression) = self.expression.as_ref() {
                get!(engine.execute(expression)?)
            } else {
                engine.undefined()
            })
        })
    }
}
