use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };

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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let reference = get!(engine.execute(&self.condition)?);
        let condition = *reference.read()?.get_cast_boolean(engine)?;
        if condition {
            engine.execute(&self.then)
        } else if let Some(r#else) = self.r#else.as_ref() {
            engine.execute(r#else)
        } else {
            Flow::new(engine.undefined())
        }
    }
}
