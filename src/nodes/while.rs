use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

pub struct While<'a> {
    condition: Node<'a>,
    body:      Node<'a>,
}

impl<'a> While<'a> {
    pub fn new(condition: Node<'a>, body: Node<'a>) -> Self {
        return Self {
            condition,
            body,
        };
    }
}

impl<'a> Executable<'a> for While<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        let mut array = Vec::new();
        while {
            let reference = execute!(engine, &self.condition);
            *reference.read()?.get_cast_boolean(engine)?
        } {
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

        return Ok(engine.new_array(array));
    }
}
