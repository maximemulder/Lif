use crate::memory::Ref;
use crate::nodes::Executable;
use crate::runtime::data::Tag;

pub struct Generic {
    pub tag: Tag,
    pub generics: Ref<[Ref<str>]>,
    pub node: Ref<dyn Executable>,
}

impl Generic {
    pub fn new(tag: Tag, generics: Ref<[Ref<str>]>, node: Ref<dyn Executable>) -> Self {
        Self {
            tag,
            generics,
            node,
        }
    }
}
