pub mod builder;
pub mod constructors;
pub mod parameters;
pub mod tag;
pub mod variable;

use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::reference::GcReference;
use crate::runtime::value::GcValue;

pub type Arguments<'a> = Box<[GcValue<'a>]>;

pub type Callable<'a> = dyn Fn(&mut Engine<'a>, Arguments<'a>) -> ReturnReference<'a>;

pub type Return<T> = Result<T, Error>;

pub type ReturnReference<'a> = Return<GcReference<'a>>;

pub type ReturnValue<'a> = Return<GcValue<'a>>;

mod flow;

pub use flow::{ Control, Jump, Flow, ReturnFlow };
