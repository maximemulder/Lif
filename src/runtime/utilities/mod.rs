pub mod builder;
pub mod memoizes;
pub mod parameters;

use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::reference::GcReference;
use crate::runtime::value::GcValue;


pub type Return<T> = Result<T, Error>;

pub type ReturnReference<'a> = Return<GcReference<'a>>;

pub type Callable<'a> = dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> ReturnReference<'a>;
