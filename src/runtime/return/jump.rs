use crate::runtime::error::Error;
use crate::runtime::reference::GcReference;
use crate::runtime::r#return::{ Flow, Return, ReturnJump };

use std::fmt;

#[derive(Clone, Copy)]
pub struct Jump<'a> {
    pub reference: GcReference<'a>,
    pub r#type: JumpType,
}

impl<'a> Jump<'a> {
    pub fn new(reference: GcReference<'a>, r#type: JumpType) -> Self {
        Self {
            reference,
            r#type,
        }
    }

    pub fn none() -> ReturnJump<'a> {
        Ok(None)
    }

    pub fn get(option: Option<Jump<'a>>) -> Return<()> {
        if let Some(jump) = option {
            Err(error_jump(jump.r#type))
        } else {
            Ok(())
        }
    }

    pub fn flow(flow: Flow<'a>) -> ReturnJump<'a> {
        Ok(if let Flow::Jump(jump) = flow {
            Some(jump)
        } else {
            None
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum JumpType {
    Continue,
    Break,
    Return,
}

impl fmt::Display for JumpType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Continue => "Continue",
            Self::Break    => "Break",
            Self::Return   => "Return",
        })
    }
}

fn error_jump(jump: JumpType) -> Error {
    Error::new_runtime(&format!("No jump allowed in this context (found jump `{}`).", jump))
}
