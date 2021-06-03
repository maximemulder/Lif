use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::jump::Jump;
use crate::runtime::utilities::{ Flow, ReturnFlow };

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

            let reference = engine.execute(&self.condition)?;
            let condition = !*reference.read().map_err(Flow::Error)?.get_cast_boolean(engine).map_err(Flow::Error)?;
            if condition {
                break;
            }
        }

        Ok(engine.new_array_any(elements))
    }
}
