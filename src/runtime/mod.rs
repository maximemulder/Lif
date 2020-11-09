pub mod data;
pub mod engine;
pub mod error;
pub mod gc;
pub mod primitives;
pub mod reference;
pub mod scope;
pub mod value;

mod gc_2;

use error::Error;
use reference::GcReference;

pub type Return<'a, T> = Result<T, Error<'a>>;

pub type ReturnReference<'a, 'b> = Return<'a, GcReference<'a, 'b>>;
