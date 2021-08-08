use crate::runtime::data::PrimitiveGeneric;
use crate::runtime::engine::Engine;
use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::primitives::Generic;
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

impl<'a> PrimitiveGeneric<'a> for Nullable<'a> {
    fn get_generic(engine: &Engine<'a>) -> GcRef<Generic<'a>> {
        engine.environment.nullable
    }
}

impl GcTrace for Nullable<'_> {
    fn trace(&mut self) {
        if let Some(mut value) = self.option {
            value.trace();
        }
    }
}
