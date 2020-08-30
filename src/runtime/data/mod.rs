mod callable;
mod class;
mod instance;

pub use class::Class;
pub use callable::{ Callable, Function, Primitive };
pub use instance::Instance;

use crate::runtime::gc::{ GcRef, GcTraceable };
use crate::runtime::reference::Reference;

pub enum Data<'a> {
	Array(Vec<GcRef<Reference<'a>>>),
	Boolean(bool),
	Callable(Box<dyn Callable<'a> + 'a>),
	Class(Class<'a>),
	Instance(Instance<'a>),
	Integer(usize),
	String(String),
	Null,
}

impl GcTraceable for Data<'_> {
	fn trace(&mut self) {
		match self {
			Data::Array(references)  => for reference in references.iter_mut() {
				reference.trace();
			},
			Data::Callable(callable) => callable.trace(),
			Data::Class(class)       => class.trace(),
			Data::Instance(instance) => instance.trace(),
			_ => (),
		}
	}
}
