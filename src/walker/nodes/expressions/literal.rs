use crate::memory::Ref;
use crate::parser::CNode;
use crate::parser::elements;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::{ ABoolean, AFloat, AIdentifier, AInteger, AString };
use crate::walker::traits::{ WLiteral, WExpression };

pub struct ALiteral {
    literal: Box<SNode<dyn WLiteral>>,
}

impl ALiteral {
    pub fn new(literal: Box<SNode<dyn WLiteral>>) -> Self {
        Self {
            literal,
        }
    }
}

impl ANode for ALiteral {
    fn build(node: Ref<CNode>) -> Self {
        let child = node.front(0);
        Self::new(match child.element {
            &elements::keywords::TRUE       => Box::new(SNode::<ABoolean>::build(child)),
            &elements::keywords::FALSE      => Box::new(SNode::<ABoolean>::build(child)),
            &elements::literals::INTEGER    => Box::new(SNode::<AInteger>::build(child)),
            &elements::literals::FLOAT      => Box::new(SNode::<AFloat>::build(child)),
            &elements::literals::STRING     => Box::new(SNode::<AString>::build(child)),
            &elements::literals::IDENTIFIER => Box::new(SNode::<AIdentifier>::build(child)),
            _ => panic!(),
        })
    }
}

impl WExpression for ALiteral {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        Flow::reference(self.literal.get().walk(engine)?)
    }
}
