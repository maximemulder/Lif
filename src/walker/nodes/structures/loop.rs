use crate::memory::Ref;
use crate::parser::CNode;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, JumpType, ReturnFlow };
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::ABlock;
use crate::walker::traits::WStructure;

pub struct ALoop {
    body: SNode<ABlock>,
}

impl ALoop {
    pub fn new(body: SNode<ABlock>) -> Self {
        Self {
            body,
        }
    }
}

impl ANode for ALoop {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(SNode::build(node.front(1)))
    }
}

impl WStructure for ALoop {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut elements = Vec::new();
        loop {
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
