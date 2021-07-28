use crate::runtime::data::Class;
use crate::runtime::engine::Engine;
use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::reference::GcReference;
use crate::runtime::r#return::Return;
use crate::runtime::value::Value;

pub struct Variable<'a> {
    pub name: Box<str>,
    pub r#type: Option<GcRef<Class<'a>>>,
}

impl<'a> Variable<'a> {
    pub fn new(name: Box<str>, r#type: Option<GcRef<Class<'a>>>) -> Self {
        Self {
            name,
            r#type,
        }
    }

    pub fn build(&self, engine: &mut Engine<'a>) -> GcReference<'a> {
        let reference = engine.new_variable(None, self.r#type.unwrap_or(engine.primitives.any));
        engine.set_variable(self.name.as_ref(), reference);
        reference
    }

    pub fn check(&self, value: Value<'a>) -> Return<()> {
        if let Some(r#type) = self.r#type {
            value.cast(r#type)?;
        }

        Ok(())
    }
}

impl<'a> GcTrace for Variable<'a> {
    fn trace(&mut self) {
        if let Some(r#type) = self.r#type.as_mut() {
            r#type.trace();
        }
    }
}
