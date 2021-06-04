use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Control, flow, flow_control_is, flow_loop_reference, ReturnFlow };

pub struct ForIn {
    identifier: Ref<str>,
    expression: Node,
    body:       Node,
}

impl ForIn {
    pub fn new(identifier: Ref<str>, expression: Node, body: Node) -> Self {
        Self {
            identifier,
            expression,
            body,
        }
    }
}

impl Executable for ForIn {
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut elements = Vec::new();
        for element in {
            let reference = engine.execute(&self.expression)?;
            flow(flow(reference.read())?.get_cast_array(engine))?.elements().iter().copied().clone()
        } {
            engine.set_variable(&self.identifier, element);
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
