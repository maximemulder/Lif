use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnReference };
use crate::walker::ANode;
use crate::walker::nodes::AStatements;
use crate::walker::traits::WExecutable;

pub struct AProgram {
    statements: ANode<AStatements>,
}

impl AProgram {
    pub fn new(statements: ANode<AStatements>) -> Self {
        Self {
            statements,
        }
    }
}

impl WExecutable for AProgram {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        Jump::get(self.statements.get().walk(engine)?)?;
        Ok(engine.undefined())
    }
}
