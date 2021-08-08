use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::GenericCode;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ Walkable, WNode };

pub struct Generic {
    name: Option<Ref<str>>,
    parameters: Box<[Ref<str>]>,
    node: WNode,
}

impl Generic {
    pub fn new(name: Option<Ref<str>>, parameters: Box<[Ref<str>]>, node: WNode) -> Self {
        Self {
            name,
            parameters,
            node,
        }
    }
}

impl Walkable for Generic {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let parameters = self.parameters.iter()
            .map(|parameter| Box::from(parameter.as_ref()))
            .collect();

        Flow::new(engine.new_generic(Ref::as_option(&self.name), parameters, GenericCode::new(Ref::new(&self.node))))
    }
}
