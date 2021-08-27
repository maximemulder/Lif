use crate::memory::Ref;
use crate::parser::CNode;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, JumpType, ReturnFlow };
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::{ ABlock, AExpression };
use crate::walker::traits::WStructure;

pub struct AFor {
    identifier: Ref<str>,
    expression: SNode<AExpression>,
    body:       SNode<ABlock>,
}

impl AFor {
    pub fn new(identifier: Ref<str>, expression: SNode<AExpression>, body: SNode<ABlock>) -> Self {
        Self {
            identifier,
            expression,
            body,
        }
    }
}

impl ANode for AFor {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(node.front(1).text(), SNode::build(node.front(3)), SNode::build(node.front(4)))
    }
}

impl WStructure for AFor {
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
