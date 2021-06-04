use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::reference::GcReference;
use crate::runtime::r#return::Return;
use crate::runtime::value::GcValue;

pub struct Variable<'a> {
    pub name: Box<str>,
    pub r#type: Option<GcValue<'a>>,
}

impl<'a> Variable<'a> {
    pub fn new_unchecked(name: Box<str>, r#type: Option<GcValue<'a>>) -> Self {
        Self {
            name,
            r#type,
        }
    }

    pub fn new(engine: &Engine<'a>, name: Box<str>, r#type: Option<GcValue<'a>>) -> Return<Self> {
        if let Some(r#type) = r#type {
            r#type.cast(engine.primitives.class)?;
        }

        Ok(Self::new_unchecked(name, r#type))
    }

    pub fn build(&self, engine: &mut Engine<'a>) -> GcReference<'a> {
        let reference = engine.new_variable(None, self.r#type.unwrap_or(engine.primitives.any));
        engine.set_variable(self.name.as_ref(), reference);
        reference
    }

    pub fn cast(&self, value: GcValue<'a>) -> Return<()> {
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
