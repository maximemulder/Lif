use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Generic<'a> {
    name: Option<&'a str>,
    parameters: Box<[&'a str]>,
    node: Node<'a>,
}

impl<'a> Generic<'a> {
    pub fn new(name: Option<&'a str>, parameters: Box<[&'a str]>, node: Node<'a>) -> Self {
        Self {
            name,
            parameters,
            node,
        }
    }
}

impl<'a> Executable<'a> for Generic<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        Ok(engine.new_generic(self.name, &self.parameters, &self.node))
    }
}
