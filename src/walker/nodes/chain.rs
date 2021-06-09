use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ Executable, Node };

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
        let value = get!(engine.execute(&self.expression)?).read()?;
        let name = engine.new_string(self.member.to_string());
        Flow::new(value.call_method(engine, "__cn__", Box::new([name.read()?]))?)
    }
}
