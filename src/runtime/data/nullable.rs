use crate::runtime::gc::GcTrace;
use crate::runtime::value::Value;

pub struct Nullable<'a> {
    pub option: Option<Value<'a>>,
}

impl<'a> Nullable<'a> {
    pub fn new(option: Option<Value<'a>>) -> Self {
        Self {
            option,
        }
    }
}

impl GcTrace for Nullable<'_> {
    fn trace(&mut self) {
        if let Some(mut value) = self.option {
            value.trace();
        }
    }
}
