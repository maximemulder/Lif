use crate::nodes::{ Executable, Node };
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Control, Flow, Jump, ReturnFlow };

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
    fn execute<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let mut elements = Vec::new();
        loop {
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
