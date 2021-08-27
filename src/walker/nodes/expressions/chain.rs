use crate::memory::Ref;
use crate::parser::CNode;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::AExpression;
use crate::walker::traits::WExpression;

pub struct AChain {
    expression: SNode<AExpression>,
    member:     Ref<str>,
}

impl AChain {
    pub fn new(expression: SNode<AExpression>, member: Ref<str>) -> Self {
        Self {
            expression,
            member,
        }
    }
}

impl ANode for AChain {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(SNode::build(node.front(0)), node.front(2).text())
    }
}

impl WExpression for AChain {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let value = flow!(self.expression.get().walk(engine)?).read()?;
        let name = engine.new_string(self.member.to_string());
        Flow::reference(value.call_method(engine, "__cn__", &mut [name.read()?])?)
    }
}
