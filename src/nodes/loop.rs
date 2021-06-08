use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnFlow };

pub struct Loop {
    body: Node,
}

impl Loop {
    pub fn new(body: Node) -> Self {
        Self {
            body,
        }
    }
}

impl Executable for Loop {
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
        }

        Ok(flow!(engine.new_array_any(elements)))
    }
}
