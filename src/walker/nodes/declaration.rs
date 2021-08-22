use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::Return;
use crate::runtime::utilities::variable::Variable;
use crate::walker::ANode;
use crate::walker::nodes::AType;

pub struct ADeclaration {
    identifier: Ref<str>,
    r#type: ANode<AType>,
}

impl ADeclaration {
    pub fn new(identifier: Ref<str>, r#type: ANode<AType>) -> Self {
        Self {
            identifier,
            r#type,
        }
    }

    pub fn walk<'a>(&self, engine: &mut Engine<'a>) -> Return<Variable<'a>> {
        Ok(Variable::new(Box::from(self.identifier.as_ref()), self.r#type.get().walk(engine)?))
    }
}
