use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::utilities::ReturnReference;

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
        let value = execute!(engine, &self.expression).read()?;
        let name = engine.new_string(self.member.to_string());
        value.call_method(engine, "__cn__", Box::new([name.read()?]))
    }
}
