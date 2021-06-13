use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, Jump, ReturnFlow };
use crate::walker::{ Walkable, WNode };

pub struct While {
    condition: WNode,
    body:      WNode,
}

impl While {
    pub fn new(condition: WNode, body: WNode) -> Self {
        Self {
            condition,
            body,
        }
    }
}

impl Walkable for While {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut elements = Vec::new();
        while {
            let reference = get!(engine.walk(&self.condition)?);
            *reference.read()?.get_cast_boolean(engine)?
        } {
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
        }

        Flow::new(engine.new_array_any(elements))
    }
}
