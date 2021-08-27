use crate::memory::Ref;
use crate::parser::CNode;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::ADeclaration;
use crate::walker::traits::WExpression;

pub struct ALet {
    declaration: SNode<ADeclaration>,
}

impl ALet {
    pub fn new(declaration: SNode<ADeclaration>) -> Self {
        Self {
            declaration,
        }
    }
}

impl ANode for ALet {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(SNode::build(node.front(1)))
    }
}

impl WExpression for ALet {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        Flow::reference(self.declaration.get().walk(engine)?.build(engine))
    }
}
