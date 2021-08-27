use crate::memory::Ref;
use crate::parser::CNode;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnJump };
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::AStatement;

pub struct AStatements {
    statements: Box<[SNode<AStatement>]>,
}

impl AStatements {
    pub fn new(statements: Box<[SNode<AStatement>]>) -> Self {
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

impl ANode for AStatements {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(node.children().iter()
            .map(|child| SNode::build(Ref::new(child)))
            .collect()
        )
    }
}
