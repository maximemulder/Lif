use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ Walkable, WNode };

pub struct If {
    condition: WNode,
    then:      WNode,
    r#else:    Option<WNode>,
}

impl If {
    pub fn new(condition: WNode, then: WNode, r#else: Option<WNode>) -> Self {
        Self {
            condition,
            then,
            r#else,
        }
    }
}

impl Walkable for If {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let reference = get!(engine.walk(&self.condition)?);
        let condition = reference.read()?.get_cast_boolean(engine)?;
        if condition {
            engine.walk(&self.then)
        } else if let Some(r#else) = self.r#else.as_ref() {
            engine.walk(r#else)
        } else {
            Flow::new(engine.undefined())
        }
    }
}
