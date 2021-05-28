use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::jump::Jump;
use crate::runtime::utilities::ReturnReference;

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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let mut array = Vec::new();
        loop {
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
