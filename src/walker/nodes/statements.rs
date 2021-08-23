use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnJump };
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

    pub fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnJump<'a> {
        for statement in self.statements.iter() {
            jump!(statement.get().walk(engine)?);
        }

        Jump::none()
    }
}
