use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

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
        let mut array = Vec::new();
        for element in {
            let reference = execute!(engine, Ref::from_ref(&self.expression));
            reference.read()?.get_cast_array(engine)?.clone()
        } {
            engine.add_variable(&self.identifier, element);
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
