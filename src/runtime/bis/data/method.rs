use crate::runtime::gc::{GcRef, GcTrace};
use crate::runtime::bis::value::Value;

pub type GcMethod<'a> = GcRef<Method<'a>>;

pub struct Method<'a> {
    pub receiver: Value<'a>,
    pub function: Value<'a>,
}

impl<'a> Method<'a> {
    pub fn new(receiver: Value<'a>, function: Value<'a>) -> Self {
        Self { receiver, function }
    }
}

impl GcTrace for Method<'_> {
    fn trace(&mut self) {
        self.receiver.trace();
        self.function.trace();
    }
}
