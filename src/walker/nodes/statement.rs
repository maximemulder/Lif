use crate::memory::Ref;
use crate::parser::CNode;
use crate::parser::elements;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnJump };
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::{ ADefinition, AExpression, AStructure };
use crate::walker::traits::WStatement;

pub struct AStatement {
    node: Box<SNode<dyn WStatement>>,
}

impl AStatement {
    pub fn new(node: Box<SNode<dyn WStatement>>) -> Self {
        Self {
            node,
        }
    }

    pub fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnJump<'a> {
        jump!(self.node.get().walk(engine)?);
        Jump::none()
    }
}

impl ANode for AStatement {
    fn build(node: Ref<CNode>) -> Self {
        let child = node.front(0);
        Self::new(match child.element {
            &elements::productions::DEFINITION => Box::new(SNode::<ADefinition>::build(child)),
            &elements::productions::STRUCTURE  => Box::new(SNode::<AStructure>::build(child)),
            &elements::productions::EXPRESSION => Box::new(SNode::<AExpression>::build(child)),
            _ => panic!(),
        })
    }
}
