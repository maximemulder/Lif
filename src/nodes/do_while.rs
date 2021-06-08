use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnFlow };

pub struct DoWhile {
    body:      Node,
    condition: Node,
}

impl DoWhile {
    pub fn new(body: Node, condition: Node) -> Self {
        Self {
            body,
            condition,
        }
    }
}

impl Executable for DoWhile {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut elements = Vec::new();
        loop {
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

            let reference = get!(engine.execute(&self.condition)?);
            let condition = !*reference.read()?.get_cast_boolean(engine)?;
            if condition {
                break;
            }
        }

        Ok(flow!(engine.new_array_any(elements)))
    }
}
