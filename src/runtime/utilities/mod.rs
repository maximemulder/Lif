pub mod constructors;
pub mod parameters;
pub mod tag;
pub mod variable;

use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::value::Value;

pub type Callable<'a> = dyn Fn(&mut Engine<'a>, &mut [Value<'a>]) -> ReturnReference<'a>;
