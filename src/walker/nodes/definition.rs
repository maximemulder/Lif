use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::GenericCode;
use crate::runtime::r#return::{ Jump, ReturnJump };
use crate::walker::ANode;
use crate::walker::traits::{ WDefinition, WStatement };

pub struct ADefinition {
    definition: Box<ANode<dyn WDefinition>>,
}

impl ADefinition {
    pub fn new(definition: Box<ANode<dyn WDefinition>>) -> Self {
        Self {
            definition
        }
    }
}

impl WStatement for ADefinition {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnJump<'a> {
        let definition = if let Some(generics) = self.definition.get().generics().get().build() {
            engine.new_generic(self.definition.get().name(), generics, GenericCode::new(Ref::new(self.definition.as_ref())))
        } else {
            self.definition.get().walk(engine)?
        };

        engine.set_variable(self.definition.get().name(), definition);
        Jump::none()
    }
}
