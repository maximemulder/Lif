use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnFlow };

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
            let reference = get!(engine.execute(&self.expression)?);
            reference.read()?.get_cast_array(engine)?.elements().iter().copied().clone()
        } {
            engine.set_variable(&self.identifier, element);
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
