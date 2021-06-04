use crate::runtime::error::Error;
use crate::runtime::r#return::Jump;

pub enum Flow<'a> {
    Jump(Jump<'a>),
    Error(Error),
}
