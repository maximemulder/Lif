use crate::memory::Ref;
use crate::parser::CNode;
use crate::parser::elements;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::GenericCode;
use crate::runtime::r#return::{ Jump, ReturnJump };
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::{ AClass, AFunction };
use crate::walker::traits::{ WDefinition, WStatement };

pub struct ADefinition {
    definition: Box<SNode<dyn WDefinition>>,
}

impl ADefinition {
    pub fn new(definition: Box<SNode<dyn WDefinition>>) -> Self {
        Self {
            definition
        }
    }
}

impl ANode for ADefinition {
    fn build(node: Ref<CNode>) -> Self {
        let child = node.front(0);
        Self::new(match child.element {
            &elements::definitions::CLASS    => Box::new(SNode::<AClass>::build(child)),
            &elements::definitions::FUNCTION => Box::new(SNode::<AFunction>::build(child)),
            _ => panic!(),
        })
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
