pub mod flow;
pub mod jump;

use crate::runtime::error::Error;
use crate::runtime::reference::GcReference;
use crate::runtime::value::GcValue;

pub use flow::Flow;
pub use jump::Jump;

pub type Return<T> = Result<T, Error>;

pub type ReturnFlow<'a> = Return<Flow<'a>>;

pub type ReturnReference<'a> = Return<GcReference<'a>>;

pub type ReturnValue<'a> = Return<GcValue<'a>>;
