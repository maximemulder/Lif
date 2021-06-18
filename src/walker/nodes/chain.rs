use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ Walkable, WNode };

pub struct Chain {
    expression: WNode,
    member:     Ref<str>,
}

impl Chain {
    pub fn new(expression: WNode, member: Ref<str>) -> Self {
        Self {
            expression,
            member,
        }
    }
}

impl Walkable for Chain {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let value = get!(engine.walk(&self.expression)?).read()?;
        let name = engine.new_string(self.member.to_string());
        Flow::new(value.call_method(engine, "__cn__", Box::new([name.read()?]))?)
    }
}
