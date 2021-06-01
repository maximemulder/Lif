use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::data::GenericCode;
use crate::runtime::engine::Engine;
use crate::runtime::utilities::ReturnReference;

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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let mut parameters = Vec::new();
        for parameter in self.parameters.iter() {
            parameters.push(Box::from(parameter.as_ref()));
        }

        Ok(engine.new_generic(Ref::as_option(&self.name), parameters.into_boxed_slice(), GenericCode::new(Ref::new(&self.node))))
    }
}
