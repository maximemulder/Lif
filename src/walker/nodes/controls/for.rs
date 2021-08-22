use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, Jump, ReturnFlow };
use crate::walker::{ ANode, WNode };
use crate::walker::nodes::{ ABlock, AControlTrait };

pub struct AFor {
    identifier: Ref<str>,
    expression: WNode,
    body:       ANode<ABlock>,
}

impl AFor {
    pub fn new(identifier: Ref<str>, expression: WNode, body: ANode<ABlock>) -> Self {
        Self {
            identifier,
            expression,
            body,
        }
    }
}

impl AControlTrait for AFor {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut elements = Vec::new();
        for element in {
            let reference = get!(engine.walk(&self.expression)?);
            reference.read()?.get_cast_array(engine)?.elements().iter().copied().clone()
        } {
            engine.set_variable(&self.identifier, element);
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
