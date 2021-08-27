use crate::memory::Ref;
use crate::parser::CNode;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, JumpType, ReturnFlow };
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::{ ABlock, AExpression };
use crate::walker::traits::WStructure;

pub struct AWhile {
    condition: SNode<AExpression>,
    body:      SNode<ABlock>,
}

impl AWhile {
    pub fn new(condition: SNode<AExpression>, body: SNode<ABlock>) -> Self {
        Self {
            condition,
            body,
        }
    }
}

impl ANode for AWhile {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(SNode::build(node.front(1)), SNode::build(node.front(2)))
    }
}

impl WStructure for AWhile {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut elements = Vec::new();
        while {
            let reference = flow!(self.condition.get().walk(engine)?);
            reference.read()?.get_cast_boolean(engine)?
        } {
            let flow = self.body.get().walk(engine)?;
            let reference = flow_loop!(flow);
            if reference.is_defined() {
                elements.push(engine.new_reference(reference.get_value()))
            }

            if flow.is_jump(JumpType::Continue) {
                continue;
            }

            if flow.is_jump(JumpType::Break) {
                break;
            }
        }

        Flow::reference(engine.new_array_any(elements))
    }
}
