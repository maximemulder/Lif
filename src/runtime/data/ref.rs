use crate::ast::Pos;
use crate::memory::Mut;
use crate::runtime::Value;
use crate::runtime::eval::errors::error_undefined;
use crate::runtime::flow::Res;
use crate::runtime::variable::Variable;
use crate::runtime::gc::GcTrace;

#[derive(Clone, Copy)]
pub struct Ref<'a> {
    variable: Mut<Variable<'a>>,
}

impl<'a> Ref<'a> {
    pub fn new(variable: *mut Variable<'a>) -> Self {
        Self {
            variable: Mut::new(variable)
        }
    }

    pub fn read(&self, pos: Pos) -> Res<Value<'a>> {
        match self.variable.content() {
            Some(value) => Ok(value),
            None => error_undefined(pos),
        }
    }

    pub fn write(&mut self, pos: Pos, value: Value<'a>) -> Res<()> {
        self.variable.write(pos, value)
    }
}

impl GcTrace for Ref<'_> {
    fn trace(&mut self) {
        self.variable.trace();
    }
}
