use crate::runtime::engine::Engine;
use crate::runtime::gc::GcRef;
use crate::runtime::primitives::Class;
use crate::runtime::r#return::Return;
use crate::walker::ANode;
use crate::walker::nodes::AExpression;

pub struct AType {
    r#type: Option<ANode<AExpression>>,
}

impl AType {
    pub fn new(r#type: Option<ANode<AExpression>>) -> Self {
        Self {
            r#type,
        }
    }

    pub fn walk<'a>(&self, engine: &mut Engine<'a>) -> Return<Option<GcRef<Class<'a>>>> {
		self.r#type.as_ref().map(|r#type| r#type.get().walk(engine)?.get()?.read()?.get_cast_class(engine)).transpose()
    }
}
