use crate::memory::Ref;
use crate::parser::CNode;
use crate::parser::elements;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnFlow, ReturnJump };
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::{ ABlock, AIf, ALoop, AWhile, AFor };
use crate::walker::traits::{ WStructure, WExpression, WStatement };

pub struct AStructure {
    structure: Box<SNode<dyn WStructure>>,
}

impl AStructure {
    pub fn new(structure: Box<SNode<dyn WStructure>>) -> Self {
        Self {
            structure,
        }
    }

    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        self.structure.get().walk(engine)
    }
}

impl ANode for AStructure {
    fn build(node: Ref<CNode>) -> Self {
        let child = node.front(0);
        Self::new(match *child.element {
            elements::structures::BLOCK => Box::new(SNode::<ABlock>::build(child)),
            elements::structures::IF    => Box::new(SNode::<AIf>::build(child)),
            elements::structures::LOOP  => Box::new(SNode::<ALoop>::build(child)),
            elements::structures::WHILE => Box::new(SNode::<AWhile>::build(child)),
            elements::structures::FOR   => Box::new(SNode::<AFor>::build(child)),
            _ => panic!(),
        })
    }
}

impl WExpression for AStructure {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        self.walk(engine)
    }
}

impl WStatement for AStructure {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnJump<'a> {
        Jump::flow(self.walk(engine)?)
    }
}
