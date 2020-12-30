pub mod data;
pub mod engine;
pub mod error;
pub mod gc;
pub mod primitives;
pub mod reference;
pub mod scope;
pub mod value;

use error::Error;
use reference::GcReference;

pub type Return<T> = Result<T, Error>;

pub type ReturnReference<'a> = Return<GcReference<'a>>;
