use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
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
        Ok(engine.new_generic(Ref::as_option(&self.name), Ref::from_ref(&self.parameters), Ref::from_ref(&self.node)))
    }
}
