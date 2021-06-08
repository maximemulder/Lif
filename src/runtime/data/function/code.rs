use crate::memory::Ref;
use crate::nodes::Node;
use crate::runtime::data::function::FunctionImplementation;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::r#return::{ Jump, ReturnReference };
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
        let flow = engine.execute(executable)?;
        if flow.jump == Jump::Return && flow.reference.is_defined() {
            Ok(engine.new_constant(flow.reference.get_value()))
        } else if flow.jump == Jump::None {
            Ok(engine.undefined())
        } else {
            Err(error_jump())
        }
    }
}


fn error_jump() -> Error {
    Error::new_runtime("Incorrect jump use.")
}
