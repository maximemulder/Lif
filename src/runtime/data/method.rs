use crate::runtime::gc::GcTrace;
use crate::runtime::value::Value;

pub struct Method<'a> {
    pub function: Value<'a>,
    pub this: Value<'a>,
}

impl<'a> Method<'a> {
    pub fn new(function: Value<'a>, this: Value<'a>) -> Self {
        Self {
            function,
            this,
        }
    }
}

impl GcTrace for Method<'_> {
    fn trace(&mut self) {
        self.function.trace();
        self.this.trace();
    }
}
