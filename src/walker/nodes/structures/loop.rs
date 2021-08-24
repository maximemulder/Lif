use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, JumpType, ReturnFlow };
use crate::walker::ANode;
use crate::walker::nodes::ABlock;
use crate::walker::traits::WStructure;

pub struct ALoop {
    body: ANode<ABlock>,
}

impl ALoop {
    pub fn new(body: ANode<ABlock>) -> Self {
        Self {
            body,
        }
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
