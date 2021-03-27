use crate::runtime::gc::GcTrace;
use crate::runtime::value::GcValue;


pub struct Parameter<'a> {
    pub name: Box<str>,
    pub r#type: GcValue<'a>,
}

impl<'a> Parameter<'a> {
    pub fn new(name: Box<str>, r#type: GcValue<'a>) -> Self {
        Self {
            name,
            r#type,
        }
    }
}

impl<'a> GcTrace for Parameter<'a> {
    fn trace(&mut self) {
        self.r#type.trace();
    }
}
