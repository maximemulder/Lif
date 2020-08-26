mod callable;
mod class;
mod instance;

pub use class::Class;
pub use callable::{ Callable, Function, Primitive };
pub use instance::Instance;

use crate::runtime::proxy::Visitable;
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

impl Visitable for Data<'_> {
	fn visit(&mut self) {
		match self {
			Data::Array(references)  => for reference in references.iter_mut() {
				reference.visit();
			},
			Data::Callable(callable) => callable.visit(),
			Data::Class(class)       => class.visit(),
			Data::Instance(instance) => instance.visit(),
			_ => (),
		}
	}
}
