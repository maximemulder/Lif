use crate::memory::Ref;
use crate::nodes::Node;
use crate::runtime::data::function::FunctionImplementation;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::jump::Jump;
use crate::runtime::utilities::{ Arguments, ReturnReference };
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
            let mut elements = Vec::new();
            for i in parameters.len() .. arguments.len() {
                elements.push(engine.new_reference(arguments[i]))
            }

            let value = engine.new_array_any_value(elements);
            rest.build(engine).set_value(value);
        }

        let executable = Ref::as_ref(&self.block);
        let reference = engine.execute(executable)?;

        if engine.jump == Jump::Break || engine.jump == Jump::Continue {
            return Err(Error::new_jump());
        }

        if engine.jump_swap(Jump::Return, Jump::None) && reference.is_defined() {
            return Ok(engine.new_constant(reference.get_value()));
        }

        Ok(engine.undefined())
    }
}
