pub mod control;
pub mod flow;
pub mod jump;

use crate::runtime::error::Error;
use crate::runtime::reference::GcReference;
use crate::runtime::value::GcValue;

pub use control::Control;
pub use flow::Flow;
pub use jump::Jump;

pub type Return<T> = Result<T, Error>;

pub type ReturnReference<'a> = Return<GcReference<'a>>;

pub type ReturnValue<'a> = Return<GcValue<'a>>;

pub type ReturnFlow<'a> = Result<GcReference<'a>, Flow<'a>>;

pub fn flow<'a, T>(result: Result<T, Error>) -> Result<T, Flow<'a>> {
    result.map_err(Flow::Error)
}
