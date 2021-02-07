use crate::memory::Ref;
use crate::nodes::Executable;
use crate::runtime::ReturnReference;
use crate::runtime::data::Tag;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::scope::GcScope;
use crate::runtime::value::GcValue;

pub struct GenericStandard<'a> {
    pub tag: Tag,
    pub scope: GcScope<'a>,
    pub generics: Ref<[Ref<str>]>,
    pub node: Ref<dyn Executable>,
}

impl<'a> GenericStandard<'a> {
    pub fn new(tag: Tag, scope: GcScope<'a>, generics: Ref<[Ref<str>]>, node: Ref<dyn Executable>) -> Self {
        Self {
            tag,
            scope,
            generics,
            node,
        }
    }

    pub fn call(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
        let generic = arguments[0].data_generic();
        engine.push_frame(generic.scope);
        for (parameter, argument) in generic.generics.iter().zip(arguments[1].data_array()) {
            let reference = engine.new_reference(argument.read()?);
            engine.add_variable(parameter, reference);
        }

        let reference = engine.execute(Ref::as_ref(&generic.node))?;
        engine.pop_frame();
        Ok(reference)
    }

}

impl GcTrace for GenericStandard<'_> {
    fn trace(&mut self) {
        self.scope.trace();
    }
}
