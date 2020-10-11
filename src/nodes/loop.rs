use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

pub struct Loop<'a> {
    body: Node<'a>,
}

impl<'a> Loop<'a> {
    pub fn new(body: Node<'a>) -> Self {
        Self {
            body,
        }
    }
}

impl<'a> Executable<'a> for Loop<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
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
