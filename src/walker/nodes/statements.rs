use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::ANode;
use crate::walker::nodes::AStatement;

pub struct AStatements {
    statements: Box<[ANode<AStatement>]>,
}

impl AStatements {
    pub fn new(statements: Box<[ANode<AStatement>]>) -> Self {
        Self {
            statements,
        }
    }

    pub fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        for statement in self.statements.iter() {
            get!(statement.get().walk(engine)?);
        }

        Flow::new(engine.undefined())
    }
}
