use crate::memory::Ref;
use crate::nodes::Node;
use crate::runtime::data::function::FunctionImplementation;
use crate::runtime::engine::{ Control, Engine };
use crate::runtime::error::Error;
use crate::runtime::gc::GcTrace;
use crate::runtime::scope::GcScope;
use crate::runtime::utilities::{ Arguments, ReturnReference };
use crate::runtime::utilities::variable::Variable;

pub struct FunctionImplementationCode<'a> {
    scope: GcScope<'a>,
    block: Ref<Node>,
}

impl<'a> FunctionImplementationCode<'a> {
    pub fn new(scope: GcScope<'a>, block: Ref<Node>) -> Self {
        Self {
            scope,
            block,
        }
    }
}

impl<'a> FunctionImplementation<'a> for FunctionImplementationCode<'a> {
    fn call(&self, engine: &mut Engine<'a>, parameters: &[Variable<'a>], rest: &Option<Variable<'a>>, arguments: Arguments<'a>) -> ReturnReference<'a> {
        let reference = engine.frame(self.scope, &|engine| {
            for (parameter, argument) in parameters.into_iter().zip(arguments.iter().copied()) {
                let mut reference = parameter.build(engine);
                reference.set_value(argument);
            }

            if let Some(rest) = rest {
                let mut elements = Vec::new();
                for i in parameters.len() .. arguments.len() {
                    elements.push(engine.new_reference(arguments[i]))
                }

                let value = engine.new_array_any_value(elements);
                let mut reference = rest.build(engine);
                reference.write(value)?;
            }

            let executable = Ref::as_ref(&self.block);
            engine.execute(executable)
        })?;

        if engine.control_is(Control::Break) || engine.control_is(Control::Continue) {
            return Err(Error::new_control());
        }

        if engine.control_consume(Control::Return) && reference.is_defined() {
            return Ok(engine.new_constant(reference.get_value()));
        }

        Ok(engine.undefined())
    }
}

impl GcTrace for FunctionImplementationCode<'_> {
    fn trace(&mut self) {
        self.scope.trace();
    }
}
