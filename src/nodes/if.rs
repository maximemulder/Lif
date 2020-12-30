use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct If {
    condition: Node,
    then:      Node,
    r#else:    Option<Node>,
}

impl If {
    pub fn new(condition: Node, then: Node, r#else: Option<Node>) -> Self {
        Self {
            condition,
            then,
            r#else,
        }
    }
}

impl Executable for If {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let reference = execute!(engine, Ref::from_ref(&self.condition));
        let condition = *reference.read()?.get_cast_boolean(engine)?;
        if condition {
            engine.execute(Ref::from_ref(&self.then))
        } else if let Some(r#else) = self.r#else.as_ref() {
            engine.execute(Ref::from_ref(r#else))
        } else {
            Ok(engine.undefined())
        }
    }
}
