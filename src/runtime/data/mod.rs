mod callable;
mod class;
mod generic;
mod object;

pub use class::Class;
pub use callable::{ Callable, Function, Primitive };
pub use object::Object;
pub use generic::Generic;

use crate::runtime::gc::GcTraceable;
use crate::runtime::reference::GcReference;

pub enum Data<'a, 'b> {
	Array(Vec<GcReference<'a, 'b>>),
	Boolean(bool),
	Callable(Box<dyn Callable<'a, 'b> + 'b>),
	Class(Class<'a, 'b>),
	Generic(Generic<'a, 'b>),
	Object(Object<'a, 'b>),
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
			Data::Object(object) => object.trace(),
			_ => (),
		}
	}
}
