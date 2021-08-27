use crate::memory::Ref;
use crate::parser::CNode;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::Return;
use crate::runtime::utilities::variable::Variable;
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::AType;

pub struct ADeclaration {
    name: Ref<str>,
    r#type: SNode<AType>,
}

impl ANode for ADeclaration {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(node.front(0).text(), SNode::build(node.front(1)))
    }
}

impl ADeclaration {
    pub fn new(name: Ref<str>, r#type: SNode<AType>) -> Self {
        Self {
            name,
            r#type,
        }
    }

    pub fn walk<'a>(&self, engine: &mut Engine<'a>) -> Return<Variable<'a>> {
        Ok(Variable::new(Box::from(self.name.as_ref()), self.r#type.get().walk(engine)?))
    }
}
