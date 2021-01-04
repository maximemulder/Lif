use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

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
            let reference = execute!(engine, Ref::from_ref(&self.condition));
            *reference.read()?.get_cast_boolean(engine)?
        } {
            let reference = engine.execute(Ref::from_ref(&self.body))?;
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
