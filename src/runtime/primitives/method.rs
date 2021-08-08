use crate::runtime::data::PrimitiveClass;
use crate::runtime::engine::Engine;
use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::primitives::Class;
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

impl<'a> PrimitiveClass<'a> for Method<'a> {
    fn get_class(engine: &Engine<'a>) -> GcRef<Class<'a>> {
        engine.environment.method
    }
}

impl GcTrace for Method<'_> {
    fn trace(&mut self) {
        self.function.trace();
        self.this.trace();
    }
}
