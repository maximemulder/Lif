use crate::runtime::gc::GcTrace;
use crate::runtime::value::GcValue;

pub struct Nullable<'a> {
    pub option: Option<GcValue<'a>>,
}

impl<'a> Nullable<'a> {
    pub fn new(option: Option<GcValue<'a>>) -> Self {
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
