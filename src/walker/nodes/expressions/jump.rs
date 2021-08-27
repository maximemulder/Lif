use crate::memory::Ref;
use crate::parser::CNode;
use crate::parser::elements;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, Jump, JumpType, ReturnFlow };
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::AExpression;
use crate::walker::traits::WExpression;

pub struct AJump {
    r#type: JumpType,
    expression: Option<SNode<AExpression>>,
}

impl AJump {
    pub fn new(r#type: JumpType, expression: Option<SNode<AExpression>>) -> Self {
        Self {
            r#type,
            expression,
        }
    }
}

impl ANode for AJump {
    fn build(node: Ref<CNode>) -> Self {
        let child = node.front(0);
        Self::new(match child.element {
            &elements::keywords::CONTINUE => JumpType::Continue,
            &elements::keywords::BREAK    => JumpType::Break,
            &elements::keywords::RETURN   => JumpType::Return,
            _ => panic!(),
        }, node.children().get(1).map(|child| SNode::build(Ref::new(child))))
    }
}

impl WExpression for AJump {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let reference = if let Some(expression) = &self.expression {
            let value = flow!(expression.get().walk(engine)?).read()?;
            engine.new_constant(value)
        } else {
            engine.undefined()
        };

        Flow::jump(Jump::new(reference, self.r#type))
    }
}
