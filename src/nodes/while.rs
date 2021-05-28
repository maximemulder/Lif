use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::jump::Jump;
use crate::runtime::utilities::ReturnReference;

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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let mut array = Vec::new();
        while {
            let reference = execute!(engine, &self.condition);
            *reference.read()?.get_cast_boolean(engine)?
        } {
            let reference = engine.execute(&self.body)?;
            if engine.jump == Jump::Return {
                return Ok(reference);
            }

            if reference.is_defined() {
                array.push(engine.new_reference(reference.get_value()));
            }

            if engine.jump_swap(Jump::Continue, Jump::None) {
                continue;
            }

            if engine.jump_swap(Jump::Break, Jump::None) {
                break;
            }
        }

        Ok(engine.new_array_any(array))
    }
}
