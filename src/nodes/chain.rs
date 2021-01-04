use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Chain {
    expression: Node,
    member:     Ref<str>,
}

impl Chain {
    pub fn new(expression: Node, member: Ref<str>) -> Self {
        Self {
            expression,
            member,
        }
    }
}

impl Executable for Chain {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let value = execute!(engine, Ref::from_ref(&self.expression)).read()?;
        let name = engine.new_string(self.member.to_string());
        value.call_method(engine, "__cn__", vec![name.read()?])
    }
}
