use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnReference;
use crate::walker::ANode;
use crate::walker::nodes::{ AStatements, AExecutableTrait };

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

impl AExecutableTrait for AProgram {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        self.statements.get().walk(engine)?.none()
    }
}
