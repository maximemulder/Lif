use crate::nodes::{ Executable, Node };
use crate::runtime::engine::{ Control, Engine };
use crate::runtime::utilities::ReturnReference;

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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        let mut array = Vec::new();
        loop {
            let reference = engine.execute(&self.body)?;
            if engine.control_is(Control::Return) {
                return Ok(reference);
            }

            if reference.is_defined() {
                array.push(engine.new_reference(reference.get_value()));
            }

            if engine.control_consume(Control::Break) {
                break;
            }

            if engine.control_consume(Control::Continue) {
                continue;
            }

            let reference = execute!(engine, &self.condition);
            let condition = !*reference.read()?.get_cast_boolean(engine)?;
            if condition {
                break;
            }
        }

        Ok(engine.new_array(array))
    }
}
