use crate::ast::Pos;
use crate::runtime::data::{Ref, GcClass};
use crate::runtime::flow::Res;
use crate::runtime::value::Value;
use crate::runtime::gc::GcTrace;

use super::eval::errors::error_type;

pub struct Variable<'a> {
    r#type: GcClass<'a>,
    content: Option<Value<'a>>,
}

impl<'a> Variable<'a> {
    pub fn value(r#type: GcClass<'a>, value: Value<'a>) -> Self {
        Self { r#type, content: Some(value) }
    }

    pub fn undefined(r#type: GcClass<'a>) -> Self {
        Self { r#type, content: None }
    }

    pub fn content(&self) -> Option<Value<'a>> {
        self.content
    }

    pub fn get_ref(&mut self) -> Ref<'a> {
        Ref::new(self)
    }

    pub fn write(&mut self, pos: Pos, value: Value<'a>) -> Res<()> {
        if !value.isa(self.r#type) {
            return error_type(pos, value, self.r#type)
        }

        self.content = Some(value);
        Ok(())
    }
}

impl GcTrace for Variable<'_> {
    fn trace(&mut self) {
        match self.content {
            Some(mut value) => value.trace(),
            None => (),
        }
    }
}
