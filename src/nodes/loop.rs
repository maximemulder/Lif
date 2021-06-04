use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Control, flow_control_is, flow_loop_reference, ReturnFlow };

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
            let r#return = engine.execute(&self.body);
            if let Some(reference) = flow_loop_reference(&r#return) {
                if reference.is_defined() {
                    elements.push(engine.new_reference(reference.get_value()))
                }
            } else {
                return r#return;
            };

            if flow_control_is(&r#return, Control::Continue) {
                continue;
            }

            if flow_control_is(&r#return, Control::Break) {
                break;
            }
        }

        Ok(engine.new_array_any(elements))
    }
}
