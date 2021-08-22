use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, Jump, ReturnFlow };
use crate::walker::ANode;
use crate::walker::nodes::{ ABlock, AControlTrait };

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

impl AControlTrait for ALoop {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut elements = Vec::new();
        loop {
            let flow = self.body.get().walk(engine)?;
            let reference = get_loop!(flow);
            if reference.is_defined() {
                elements.push(engine.new_reference(reference.get_value()))
            }

            if flow.jump == Jump::Continue {
                continue;
            }

            if flow.jump == Jump::Break {
                break;
            }
        }

        Flow::new(engine.new_array_any(elements))
    }
}
