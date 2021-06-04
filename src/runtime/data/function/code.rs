use crate::memory::Ref;
use crate::nodes::Node;
use crate::runtime::data::function::FunctionImplementation;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::r#return::{ Control, Flow, ReturnReference };
use crate::runtime::utilities::Arguments;
use crate::runtime::utilities::variable::Variable;

pub struct FunctionCode {
    block: Ref<Node>,
}

impl FunctionCode {
    pub fn new(block: Ref<Node>) -> Self {
        Self {
            block,
        }
    }
}

impl<'a> FunctionImplementation<'a> for FunctionCode {
    fn call(&self, engine: &mut Engine<'a>, parameters: &[Variable<'a>], rest: &Option<Variable<'a>>, arguments: Arguments<'a>) -> ReturnReference<'a> {
        for (parameter, argument) in parameters.iter().zip(arguments.iter().copied()) {
            parameter.build(engine).set_value(argument);
        }

        if let Some(rest) = rest {
            let elements = arguments[parameters.len()..].iter()
                .copied()
                .map(|argument| engine.new_reference(argument))
                .collect();

            let value = engine.new_array_any_value(elements);
            rest.build(engine).set_value(value);
        }

        let executable = Ref::as_ref(&self.block);
        match engine.execute(executable) {
            Ok(_) => Ok(engine.undefined()),
            Err(flow) => match flow {
                Flow::Jump(jump) => {
                    if jump.control == Control::Return && jump.reference.is_defined() {
                        Ok(engine.new_constant(jump.reference.get_value()))
                    } else {
                        Err(Error::new_jump())
                    }
                }
                Flow::Error(error) => Err(error),
            }
        }
    }
}
