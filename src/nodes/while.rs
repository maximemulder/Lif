use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnFlow };

pub struct While {
    condition: Node,
    body:      Node,
}

impl While {
    pub fn new(condition: Node, body: Node) -> Self {
        Self {
            condition,
            body,
        }
    }
}

impl Executable for While {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut elements = Vec::new();
        while {
            let reference = get!(engine.execute(&self.condition)?);
            *reference.read()?.get_cast_boolean(engine)?
        } {
            let flow = engine.execute(&self.body)?;
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

        Ok(flow!(engine.new_array_any(elements)))
    }
}
