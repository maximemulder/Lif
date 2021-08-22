use crate::runtime::error::Error;
use crate::runtime::reference::GcReference;
use crate::runtime::r#return::{ Jump, ReturnFlow, ReturnReference };

pub struct Flow<'a> {
    pub reference: GcReference<'a>,
    pub jump: Jump,
}

impl<'a> Flow<'a> {
    pub fn new(reference: GcReference<'a>) -> ReturnFlow<'a> {
        Self::new_jump(reference, Jump::None)
    }

    pub fn new_tmp(reference: GcReference<'a>) -> Self {
        Self {
            reference,
            jump: Jump::None,
        }
    }

    pub fn new_jump(reference: GcReference<'a>, jump: Jump) -> ReturnFlow<'a> {
        Ok(Self {
            reference,
            jump,
        })
    }

    pub fn none(&self) -> ReturnReference<'a> {
        if self.jump == Jump::None {
            Ok(self.reference)
        } else {
            Err(error_jump(self.jump))
        }
    }
}

fn error_jump(jump: Jump) -> Error {
    Error::new_runtime(&format!("No jump allowed in this context (found jump `{}`).", jump))
}
