mod callable;
mod class;
mod instance;

pub use class::Class;
pub use callable::{ Callable, Function, Primitive };
pub use instance::Instance;

use crate::runtime::gc::GcTraceable;
use crate::runtime::reference::GcReference;

pub enum Data<'a, 'b> {
	Array(Vec<GcReference<'a, 'b>>),
	Boolean(bool),
	Callable(Box<dyn Callable<'a, 'b> + 'b>),
	Class(Class<'a, 'b>),
	Instance(Instance<'a, 'b>),
	Integer(usize),
	String(String),
	Null,
}

impl GcTraceable for Data<'_, '_> {
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
