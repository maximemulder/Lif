use crate::memory::Ref;
use crate::nodes::Node;
use crate::runtime::ReturnReference;
use crate::runtime::data::{ Callable, Tag };
use crate::runtime::engine::{ Control, Engine };
use crate::runtime::error::Error;
use crate::runtime::gc::GcTrace;
use crate::runtime::scope::GcScope;
use crate::runtime::value::GcValue;

#[derive(Clone)]
pub struct Function<'a> {
    tag: Tag,
    scope: GcScope<'a>,
    parameters: Ref<[Node]>,
    r#type: Option<GcValue<'a>>,
    block: Ref<Node>,
}

impl<'a> Function<'a> {
    pub fn new(tag: Tag, scope: GcScope<'a>, parameters: Ref<[Node]>, r#type: Option<GcValue<'a>>, block: Ref<Node>) -> Self {
        Self {
            tag,
            scope,
            parameters,
            r#type,
            block,
        }
    }
}

impl<'a> Callable<'a> for Function<'a> {
    fn execute(&self, engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
        if arguments.len() != self.parameters.len() {
            return Err(Error::new_arguments(self.parameters.len(), arguments.len()));
        }

        engine.push_frame(self.scope);
        for (parameter, argument) in self.parameters.iter().zip(arguments) {
            let mut reference = engine.execute(parameter)?;
            reference.write(argument)?;
        }

        let executable = Ref::as_ref(&self.block);
        let reference = engine.execute(executable)?;
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

    fn duplicate<'c>(&'c self) -> Box<dyn Callable<'a> + 'c> {
        Box::new(self.clone())
    }

    fn get_tag(&self) -> Tag {
        self.tag.clone()
    }
}

impl GcTrace for Function<'_> {
    fn trace(&mut self) {
        self.scope.trace();
        if let Some(mut r#type) = self.r#type {
            r#type.trace();
        }
    }
}
