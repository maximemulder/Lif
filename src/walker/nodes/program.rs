use crate::memory::Ref;
use crate::parser::CNode;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnReference };
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::AStatements;
use crate::walker::traits::WExecutable;

pub struct AProgram {
    statements: SNode<AStatements>,
}

impl AProgram {
    pub fn new(statements: SNode<AStatements>) -> Self {
        Self {
            statements,
        }
    }
}

impl ANode for AProgram {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(SNode::build(node.front(0)))
    }
}

impl WExecutable for AProgram {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        Jump::get(self.statements.get().walk(engine)?)?;
        Ok(engine.undefined())
    }
}
