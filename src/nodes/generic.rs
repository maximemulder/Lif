use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Generic<'a> {
    parameters: Box<[&'a str]>,
    node: Node<'a>,
}

impl<'a> Generic<'a> {
    pub fn new(parameters: Box<[&'a str]>, node: Node<'a>) -> Self {
        Self {
            parameters,
            node,
        }
    }
}

impl<'a> Executable<'a> for Generic<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        Ok(engine.new_generic(&self.parameters, &self.node))
    }
}
