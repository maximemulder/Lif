use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

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
        }

        Ok(engine.new_array(array))
    }
}
