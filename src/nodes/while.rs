use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Control, flow, flow_control_is, flow_loop_reference, ReturnFlow };

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
            let reference = engine.execute(&self.condition)?;
            *flow(flow(reference.read())?.get_cast_boolean(engine))?
        } {
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
