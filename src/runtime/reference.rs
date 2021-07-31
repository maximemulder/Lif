use crate::runtime::data::Class;
use crate::runtime::error::Error;
use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::r#return::{ Return, ReturnValue };
use crate::runtime::value::Value;

pub type GcReference<'a> = GcRef<Reference<'a>>;

pub struct Reference<'a> {
    value: Option<Value<'a>>,
    r#type: Option<GcRef<Class<'a>>>,
}

impl<'a> Reference<'a> {
    pub fn new_variable(value: Option<Value<'a>>, r#type: GcRef<Class<'a>>) -> Self {
        Self {
            value,
            r#type: Some(r#type),
        }
    }

    pub fn new_constant(value: Option<Value<'a>>) -> Self {
        Self {
            value,
            r#type: None,
        }
    }

    pub fn read(&self) -> ReturnValue<'a> {
        self.value.ok_or_else(error_undefined)
    }

    pub fn write(&mut self, value: Value<'a>) -> Return<()> {
        match self.r#type {
            Some(r#type) => {
                value.cast(r#type)?;
                self.set_value(value);
            },
            None => if self.value.is_none() {
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

    pub fn get_value(&self) -> Value<'a> {
        self.value.unwrap()
    }

    pub fn set_value(&mut self, value: Value<'a>) {
        self.value = Some(value);
    }
}

impl GcTrace for Reference<'_> {
    fn trace(&mut self) {
        if let Some(value) = self.value.as_mut() {
            value.trace();
        }

        if let Some(r#type) = self.r#type.as_mut() {
            r#type.trace();
        }
    }
}

fn error_undefined() -> Error {
    Error::new_runtime("Cannot read an undefined reference.")
}

fn error_write_constant() -> Error {
    Error::new_runtime("Cannot write data into a constant.")
}
