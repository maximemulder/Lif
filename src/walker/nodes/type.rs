use crate::memory::Ref;
use crate::parser::CNode;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcRef;
use crate::runtime::primitives::Class;
use crate::runtime::r#return::Return;
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::AExpression;

pub struct AType {
    r#type: Option<SNode<AExpression>>,
}

impl AType {
    pub fn new(r#type: Option<SNode<AExpression>>) -> Self {
        Self {
            r#type,
        }
    }

    pub fn walk<'a>(&self, engine: &mut Engine<'a>) -> Return<Option<GcRef<Class<'a>>>> {
        self.r#type.as_ref().map(|r#type| r#type.get().walk(engine)?.get()?.read()?.get_cast_class(engine)).transpose()
    }
}

impl ANode for AType {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(node.children().get(1).map(|child| SNode::build(Ref::new(child))))
    }
}
