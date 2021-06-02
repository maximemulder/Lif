use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::jump::Jump;
use crate::runtime::utilities::ReturnReference;

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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let mut elements = Vec::new();
        for element in {
            let reference = execute!(engine, &self.expression);
            reference.read()?.get_cast_array(engine)?.elements().iter().copied().clone()
        } {
            engine.set_variable(&self.identifier, element);
            let reference = engine.execute(&self.body)?;
            if engine.jump == Jump::Return {
                return Ok(reference);
            }

            if reference.is_defined() {
                elements.push(engine.new_reference(reference.get_value()));
            }

            if engine.jump_swap(Jump::Continue, Jump::None) {
                continue;
            }

            if engine.jump_swap(Jump::Break, Jump::None) {
                break;
            }
        }

        Ok(engine.new_array_any(elements))
    }
}
