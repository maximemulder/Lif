use crate::memory::Ref;
use crate::nodes::Executable;
use crate::runtime::data::Tag;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::scope::GcScope;
use crate::runtime::utilities::ReturnReference;
use crate::runtime::utilities::memoizes::Memoizes;
use crate::runtime::utilities::parameters;
use crate::runtime::value::GcValue;

pub struct GenericCode<'a> {
    pub tag: Tag,
    scope: GcScope<'a>,
    parameters: Ref<[Ref<str>]>,
    node: Ref<dyn Executable>,
    memoizes: Memoizes<'a>,
}

impl<'a> GenericCode<'a> {
    pub fn new(tag: Tag, scope: GcScope<'a>, parameters: Ref<[Ref<str>]>, node: Ref<dyn Executable>) -> Self {
        Self {
            tag,
            scope,
            parameters,
            node,
            memoizes: Memoizes::new(),
        }
    }

    pub fn call(&mut self, engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
        parameters::length(arguments.len(), self.parameters.len())?;
        if let Some(reference) = self.memoizes.get(&arguments) {
            return Ok(reference);
        }

        let reference = engine.frame(self.scope, &|engine| {
            for (parameter, argument) in self.parameters.iter().zip(arguments.iter()) {
                let reference = engine.new_reference(*argument);
                engine.add_variable(parameter, reference);
            }

            engine.execute(Ref::as_ref(&self.node))
        })?;

        self.memoizes.record(arguments.into_boxed_slice(), reference);
        Ok(reference)
    }
}

impl GcTrace for GenericCode<'_> {
    fn trace(&mut self) {
        self.scope.trace();
        self.memoizes.trace();
    }
}
