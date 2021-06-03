use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::utilities::{ Flow, ReturnFlow };

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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let value = engine.execute(&self.expression)?.read().map_err(Flow::Error)?;
        let name = engine.new_string(self.member.to_string());
        value.call_method(engine, "__cn__", Box::new([name.read().map_err(Flow::Error)?])).map_err(Flow::Error)
    }
}
