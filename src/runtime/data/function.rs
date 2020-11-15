use crate::nodes::Node;
use crate::runtime::ReturnReference;
use crate::runtime::data::{ Callable, Tag };
use crate::runtime::engine::{ Control, Engine };
use crate::runtime::error::Error;
use crate::runtime::gc::GcTrace;
use crate::runtime::scope::GcScope;
use crate::runtime::value::GcValue;

#[derive(Clone)]
pub struct Function<'a, 'b> {
    tag: Tag,
    scope: GcScope<'a, 'b>,
    parameters: &'b [Node<'a>],
    r#type: Option<GcValue<'a, 'b>>,
    block: &'b Node<'a>,
}

impl<'a, 'b> Function<'a, 'b> {
    pub fn new(tag: Tag, scope: GcScope<'a, 'b>, parameters: &'b [Node<'a>], r#type: Option<GcValue<'a, 'b>>, block: &'b Node<'a>) -> Self {
        Self {
            tag,
            scope,
            parameters,
            r#type,
            block,
        }
    }
}

impl<'a, 'b> Callable<'a, 'b> for Function<'a, 'b> {
    fn execute(&self, engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
        if arguments.len() != self.parameters.len() {
            return Err(Error::new_arguments(self.parameters.len(), arguments.len()));
        }

        engine.push_frame(self.scope);
        for (parameter, argument) in self.parameters.iter().zip(arguments) {
            let mut reference = engine.execute(parameter)?;
            reference.write(argument)?;
        }

        let reference = engine.execute(self.block)?;
        engine.pop_frame();

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

    fn duplicate<'c>(&'c self) -> Box<dyn Callable<'a, 'b> + 'c> {
        Box::new(self.clone())
    }

    fn get_tag(&self) -> Tag {
        self.tag.clone()
    }
}

impl GcTrace for Function<'_, '_> {
    fn trace(&mut self) {
        self.scope.trace();
    }
}
