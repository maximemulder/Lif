use crate::memory::Ref;
use crate::nodes::Node;
use crate::runtime::data::function::FunctionImplementation;
use crate::runtime::engine::{ Control, Engine };
use crate::runtime::error::Error;
use crate::runtime::gc::GcTrace;
use crate::runtime::scope::GcScope;
use crate::runtime::utilities::{ Arguments, ReturnReference };
use crate::runtime::utilities::parameters;

pub struct FunctionImplementationCode<'a> {
    scope: GcScope<'a>,
    parameters: Ref<[Node]>,
    block: Ref<Node>,
}

impl<'a> FunctionImplementationCode<'a> {
    pub fn new(scope: GcScope<'a>, parameters: Ref<[Node]>, block: Ref<Node>) -> Self {
        Self {
            scope,
            parameters,
            block,
        }
    }
}

impl<'a> FunctionImplementation<'a> for FunctionImplementationCode<'a> {
    fn length(&self) -> usize {
        self.parameters.len()
    }

    fn call(&self, engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
        parameters::length(arguments.len(), self.parameters.len())?;
        let reference = engine.frame(self.scope, &|engine| {
            for (parameter, argument) in self.parameters.iter().zip(arguments.iter()) {
                let mut reference = engine.execute(parameter)?;
                reference.write(*argument)?;
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
