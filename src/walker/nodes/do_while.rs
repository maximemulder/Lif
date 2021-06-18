use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, Jump, ReturnFlow };
use crate::walker::{ Walkable, WNode };

pub struct DoWhile {
    body:      WNode,
    condition: WNode,
}

impl DoWhile {
    pub fn new(body: WNode, condition: WNode) -> Self {
        Self {
            body,
            condition,
        }
    }
}

impl Walkable for DoWhile {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut elements = Vec::new();
        loop {
            let flow = engine.walk(&self.body)?;
            let reference = get_loop!(flow);
            if reference.is_defined() {
                elements.push(engine.new_reference(reference.get_value()))
            }

            if flow.jump == Jump::Continue {
                continue;
            }

            if flow.jump == Jump::Break {
                break;
            }

            let reference = get!(engine.walk(&self.condition)?);
            let condition = !*reference.read()?.get_cast_boolean(engine)?;
            if condition {
                break;
            }
        }

        Flow::new(engine.new_array_any(elements))
    }
}
