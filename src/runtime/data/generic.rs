use crate::memory::Ref;
use crate::nodes::Executable;
use crate::runtime::data::Tag;
use crate::runtime::gc::GcTrace;
use crate::runtime::scope::GcScope;

pub struct Generic<'a> {
    pub tag: Tag,
    pub scope: GcScope<'a>,
    pub generics: Ref<[Ref<str>]>,
    pub node: Ref<dyn Executable>,
}

impl<'a> Generic<'a> {
    pub fn new(tag: Tag, scope: GcScope<'a>, generics: Ref<[Ref<str>]>, node: Ref<dyn Executable>) -> Self {
        Self {
            tag,
            scope,
            generics,
            node,
        }
    }
}

impl GcTrace for Generic<'_> {
    fn trace(&mut self) {
        self.scope.trace();
    }
}
