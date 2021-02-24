use crate::memory::Ref;
use crate::nodes::Node;
use crate::runtime::data::function::FunctionImplementation;
use crate::runtime::engine::{ Control, Engine };
use crate::runtime::error::Error;
use crate::runtime::gc::GcTrace;
use crate::runtime::scope::GcScope;
use crate::runtime::utilities::{ Arguments, ReturnReference };
use crate::runtime::value::GcValue;

pub struct FunctionImplementationCode<'a> {
    scope: GcScope<'a>,
    names: Box<[Ref<str>]>,
    block: Ref<Node>,
}

impl<'a> FunctionImplementationCode<'a> {
    pub fn new(scope: GcScope<'a>, names: Box<[Ref<str>]>, block: Ref<Node>) -> Self {
        Self {
            scope,
            names,
            block,
        }
    }
}

impl<'a> FunctionImplementation<'a> for FunctionImplementationCode<'a> {
    fn call(&self, engine: &mut Engine<'a>, parameters: &[GcValue<'a>], arguments: Arguments<'a>) -> ReturnReference<'a> {
        let reference = engine.frame(self.scope, &|engine| {
            for ((name, parameter), argument) in self.names.iter().zip(parameters.iter()).zip(arguments.iter()) {
                let reference = engine.new_variable(Some(*argument), *parameter);
                engine.add_variable(name, reference);
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
