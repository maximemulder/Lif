use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::utilities::{ Control, Flow, Jump, ReturnFlow };

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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut elements = Vec::new();
        while {
            let reference = engine.execute(&self.condition)?;
            *reference.read().map_err(Flow::Error)?.get_cast_boolean(engine).map_err(Flow::Error)?
        } {
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
