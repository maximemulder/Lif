use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::Return;
use crate::runtime::utilities::variable::Variable;
use crate::walker::ANode;
use crate::walker::nodes::AType;

pub struct ADeclaration {
    name: Ref<str>,
    r#type: ANode<AType>,
}

impl ADeclaration {
    pub fn new(name: Ref<str>, r#type: ANode<AType>) -> Self {
        Self {
            name,
            r#type,
        }
    }

    pub fn walk<'a>(&self, engine: &mut Engine<'a>) -> Return<Variable<'a>> {
        Ok(Variable::new(Box::from(self.name.as_ref()), self.r#type.get().walk(engine)?))
    }
}
