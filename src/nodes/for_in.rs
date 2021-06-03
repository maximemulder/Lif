use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::utilities::{ Control, Flow, Jump, ReturnFlow };

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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut elements = Vec::new();
        for element in {
            let reference = engine.execute(&self.expression)?;
            reference.read().map_err(Flow::Error)?.get_cast_array(engine).map_err(Flow::Error)?.elements().iter().copied().clone()
        } {
            engine.set_variable(&self.identifier, element);
            let r#return = engine.execute(&self.body);
            match r#return {
                Ok(reference) => if reference.is_defined() {
                    elements.push(reference);
                }
                Err(flow) => match flow {
                    Flow::Error(error) => return Err(Flow::Error(error)),
                    Flow::Jump(jump) => match jump.control {
                        Control::Continue => {
                            if jump.reference.is_defined() {
                                elements.push(jump.reference);
                            }
                            continue;
                        },
                        Control::Break => {
                            if jump.reference.is_defined() {
                                elements.push(jump.reference);
                            }
                            break;
                        },
                        Control::Return => {
                            return Err(Flow::Jump(Jump::new(Control::Return, jump.reference)));
                        },
                    },
                },
            };
        }

        Ok(engine.new_array_any(elements))
    }
}
