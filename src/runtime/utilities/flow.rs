use crate::runtime::reference::GcReference;
use crate::runtime::error::Error;

#[derive(PartialEq, Eq)]
pub enum Control {
    Continue,
    Break,
    Return
}

pub struct Jump<'a> {
    pub control: Control,
    pub reference: GcReference<'a>,
}

pub enum Flow<'a> {
    Jump(Jump<'a>),
    Error(Error),
}

/* impl<'a> Flow<'a> {
    pub fn error(error: Error) -> Self {
        Self
    }
} */

pub type ReturnFlow<'a> = Result<GcReference<'a>, Flow<'a>>;
