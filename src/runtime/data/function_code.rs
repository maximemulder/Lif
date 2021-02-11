use crate::memory::Ref;
use crate::nodes::Node;
use crate::runtime::data::Tag;
use crate::runtime::engine::{ Control, Engine };
use crate::runtime::error::Error;
use crate::runtime::gc::GcTrace;
use crate::runtime::scope::GcScope;
use crate::runtime::utilities::ReturnReference;
use crate::runtime::utilities::parameters;
use crate::runtime::value::GcValue;

pub struct FunctionCode<'a> {
    pub tag: Tag,
    scope: GcScope<'a>,
    parameters: Ref<[Node]>,
    r#type: Option<GcValue<'a>>,
    block: Ref<Node>,
}

impl<'a> FunctionCode<'a> {
    pub fn new(tag: Tag, scope: GcScope<'a>, parameters: Ref<[Node]>, r#type: Option<GcValue<'a>>, block: Ref<Node>) -> Self {
        Self {
            tag,
            scope,
            parameters,
            r#type,
            block,
        }
    }

    pub fn call(&self, engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
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

        if engine.control_consume(Control::Return) {
            if let Some(r#type) = self.r#type {
                let value = reference.read()?;
                value.cast(r#type)?;
                return Ok(engine.new_constant(value));
            }

            if reference.is_defined() {
                return Ok(engine.new_constant(reference.get_value()));
            }
        }

        Ok(engine.undefined())
    }
}

impl GcTrace for FunctionCode<'_> {
    fn trace(&mut self) {
        self.scope.trace();
        if let Some(mut r#type) = self.r#type {
            r#type.trace();
        }
    }
}
