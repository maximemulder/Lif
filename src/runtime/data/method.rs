use crate::runtime::gc::GcTrace;
use crate::runtime::value::GcValue;

pub struct Method<'a> {
    pub function: GcValue<'a>,
    pub this: GcValue<'a>,
}

impl<'a> Method<'a> {
    pub fn new(function: GcValue<'a>, this: GcValue<'a>) -> Self {
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
