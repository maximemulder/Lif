use crate::runtime::gc::{GcRef, GcTrace};

pub type GcString<'a> = GcRef<String>;

pub struct String(pub Box<str>);

impl AsRef<str> for String {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl GcTrace for String {}
