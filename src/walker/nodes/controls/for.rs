use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, JumpType, ReturnFlow };
use crate::walker::ANode;
use crate::walker::nodes::{ ABlock, AExpression, AControlTrait };

pub struct AFor {
    identifier: Ref<str>,
    expression: ANode<AExpression>,
    body:       ANode<ABlock>,
}

impl AFor {
    pub fn new(identifier: Ref<str>, expression: ANode<AExpression>, body: ANode<ABlock>) -> Self {
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
            let reference = flow!(self.expression.get().walk(engine)?);
            reference.read()?.get_cast_array(engine)?.elements().iter().copied().clone()
        } {
            engine.set_variable(&self.identifier, element);
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
