use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::{ Control, Engine };

pub struct ForIn<'a> {
    identifier: &'a str,
    expression: Node<'a>,
    body:       Node<'a>,
}

impl<'a> ForIn<'a> {
    pub fn new(identifier: &'a str, expression: Node<'a>, body: Node<'a>) -> Self {
        Self {
            identifier,
            expression,
            body,
        }
    }
}

impl<'a> Executable<'a> for ForIn<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        let mut array = Vec::new();
        for element in {
            let reference = execute!(engine, &self.expression);
            reference.read()?.get_cast_array(engine)?.clone()
        } {
            engine.add_variable(&self.identifier, element);
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
