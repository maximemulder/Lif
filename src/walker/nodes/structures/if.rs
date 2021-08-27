use crate::memory::Ref;
use crate::parser::CNode;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::{ ABlock, AExpression };
use crate::walker::traits::WStructure;

pub struct AIf {
    condition: SNode<AExpression>,
    then:      SNode<ABlock>,
    r#else:    Option<SNode<ABlock>>,
}

impl AIf {
    pub fn new(condition: SNode<AExpression>, then: SNode<ABlock>, r#else: Option<SNode<ABlock>>) -> Self {
        Self {
            condition,
            then,
            r#else,
        }
    }
}

impl ANode for AIf {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(
            SNode::build(node.front(1)),
            SNode::build(node.front(2)),
            node.children().get(4).map(|child| SNode::build(Ref::new(child)))
        )
    }
}

impl WStructure for AIf {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let reference = flow!(self.condition.get().walk(engine)?);
        let condition = reference.read()?.get_cast_boolean(engine)?;
        if condition {
            self.then.get().walk(engine)
        } else if let Some(r#else) = self.r#else.as_ref() {
            r#else.get().walk(engine)
        } else {
            Flow::reference(engine.undefined())
        }
    }
}
