use crate::runtime::engine::Engine;
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
        let definition = self.definition.get().walk(engine)?;
        engine.set_variable(definition.read()?.get_tag(engine).get_name().unwrap(), definition);
        Jump::none()
    }
}
