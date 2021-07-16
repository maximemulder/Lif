use crate::runtime::error::Error;
use crate::runtime::engine::Engine;
use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::r#return::{ Return, ReturnValue };
use crate::runtime::value::GcValue;

pub type GcReference<'a> = GcRef<Reference<'a>>;

pub struct Reference<'a> {
    value: Option<GcValue<'a>>,
    r#type: Type<'a>,
}

enum Type<'a> {
    Variable(GcValue<'a>),
    Constant,
}

impl<'a> Reference<'a> {
    pub fn new_variable(value: Option<GcValue<'a>>, r#type: GcValue<'a>) -> Self {
        Self {
            value,
            r#type: Type::Variable(r#type),
        }
    }

    pub fn new_constant(value: Option<GcValue<'a>>) -> Self {
        Self {
            value,
            r#type: Type::Constant,
        }
    }

    pub fn read(&self) -> ReturnValue<'a> {
        self.value.ok_or_else(error_undefined)
    }

    pub fn write(&mut self, engine: &Engine<'a>, value: GcValue<'a>) -> Return<()> {
        match self.r#type {
            Type::Variable(r#type) => {
                value.cast(engine, r#type)?;
                self.set_value(value);
            },
            Type::Constant => if self.value.is_none() {
                self.set_value(value);
            } else {
                return Err(error_write_constant());
            },
        }

        Ok(())
    }

    pub fn is_defined(&self) -> bool {
        self.value.is_some()
    }

    pub fn is_undefined(&self) -> bool {
        !self.is_defined()
    }

    pub fn get_value(&self) -> GcValue<'a> {
        self.value.unwrap()
    }

    pub fn set_value(&mut self, value: GcValue<'a>) {
        self.value = Some(value);
    }
}

impl GcTrace for Reference<'_> {
    fn trace(&mut self) {
        if let Some(value) = self.value.as_mut() {
            value.trace();
        }
    }
}

fn error_undefined() -> Error {
    Error::new_runtime("Cannot read an undefined reference.")
}

fn error_write_constant() -> Error {
    Error::new_runtime("Cannot write data into a constant.")
}
