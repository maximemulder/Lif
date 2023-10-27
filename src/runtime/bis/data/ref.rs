use crate::memory::Mut;
use crate::runtime::bis::Value;
use crate::runtime::gc::GcTrace;

#[derive(Clone, Copy)]
pub struct Ref<'a> {
    value: Mut<Option<Value<'a>>>,
}

impl<'a> Ref<'a> {
    pub fn new(value: *mut Option<Value<'a>>) -> Self {
        Self {
            value: Mut::new(value)
        }
    }

    pub fn read(&self) -> Option<Value<'a>> {
        *self.value
    }

    pub fn write(&mut self, value: Value<'a>) {
        *self.value = Some(value);
    }
}

impl GcTrace for Ref<'_> {
    fn trace(&mut self) {
        if let Some(value) = self.value.as_mut() {
            value.trace();
        }
    }
}
