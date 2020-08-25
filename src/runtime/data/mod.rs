mod callable;
mod class;
mod instance;

pub use class::Class;
pub use callable::{ Callable, Function, Primitive };
pub use instance::Instance;

use crate::runtime::reference::Reference;

pub enum Data<'a> {
	Array(Vec<Reference<'a>>),
	Boolean(bool),
	Callable(Box<dyn Callable<'a> + 'a>),
	Class(Class<'a>),
	Instance(Instance<'a>),
	Integer(usize),
	String(String),
	Null,
}
