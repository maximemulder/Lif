use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, Jump, ReturnFlow };
use crate::walker::ANode;
use crate::walker::nodes::{ ABlock, AExpression, AControlTrait };

pub struct AWhile {
    condition: ANode<AExpression>,
    body:      ANode<ABlock>,
}

impl AWhile {
    pub fn new(condition: ANode<AExpression>, body: ANode<ABlock>) -> Self {
        Self {
            condition,
            body,
        }
    }
}

impl AControlTrait for AWhile {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut elements = Vec::new();
        while {
            let reference = get!(self.condition.get().walk(engine)?);
            reference.read()?.get_cast_boolean(engine)?
        } {
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
