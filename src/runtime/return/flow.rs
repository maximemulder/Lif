use crate::runtime::error::Error;
use crate::runtime::reference::GcReference;
use crate::runtime::r#return::{ Jump, JumpType, ReturnFlow, ReturnReference };

#[derive(Clone, Copy)]
pub enum Flow<'a> {
    Reference(GcReference<'a>),
    Jump(Jump<'a>),
}

impl<'a> Flow<'a> {
    pub fn reference(reference: GcReference<'a>) -> ReturnFlow<'a> {
        Ok(Self::Reference(reference))
    }

    pub fn jump(jump: Jump<'a>) -> ReturnFlow<'a> {
        Ok(Self::Jump(jump))
    }

    pub fn get(self) -> ReturnReference<'a> {
        match self {
            Self::Reference(reference) => Ok(reference),
            Self::Jump(jump) => Err(error_jump(jump.r#type)),
        }
    }

    pub fn get_reference(self) -> GcReference<'a> {
        match self {
            Self::Reference(reference) => reference,
            Self::Jump(jump) => jump.reference,
        }
    }

    pub fn is_jump(self, r#type: JumpType) -> bool {
        if let Self::Jump(jump) = self {
            if jump.r#type == r#type {
                return true;
            }
        }

        false
    }

    pub fn is_jump_reference(self, r#type: JumpType) -> Option<GcReference<'a>> {
        if let Self::Jump(jump) = self {
            if jump.r#type == r#type {
                return Some(jump.reference);
            }
        }

        None
    }
}

fn error_jump(jump: JumpType) -> Error {
    Error::new_runtime(&format!("No jump allowed in this context (found jump `{}`).", jump))
}
