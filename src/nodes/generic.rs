use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::data::GenericCode;
use crate::runtime::engine::Engine;
use crate::runtime::utilities::ReturnFlow;

pub struct Generic {
    name: Option<Ref<str>>,
    parameters: Box<[Ref<str>]>,
    node: Node,
}

impl Generic {
    pub fn new(name: Option<Ref<str>>, parameters: Box<[Ref<str>]>, node: Node) -> Self {
        Self {
            name,
            parameters,
            node,
        }
    }
}

impl Executable for Generic {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let parameters = self.parameters.iter()
            .map(|parameter| Box::from(parameter.as_ref()))
            .collect();

        Ok(engine.new_generic(Ref::as_option(&self.name), parameters, GenericCode::new(Ref::new(&self.node))))
    }
}
