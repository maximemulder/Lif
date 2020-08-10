pub mod callable;
pub mod class;
pub mod data;
pub mod instance;

use crate::runtime::{ Engine, Reference, Value };
use crate::nodes::block::Block;
use data::Data;
use class::Class;
use callable::{ Function, Primitive };

pub struct Object<'a> {
	pub class: Value,
	pub data: Data<'a>,
}

impl<'a> Object<'a> {
	pub fn new(class: Value, data: Data<'a>) -> Self {
		return Self {
			class,
			data,
		};
	}

	pub fn new_boolean(engine: &Engine, boolean: bool) -> Self {
		return Self::new(engine.classes.boolean, Data::Boolean(boolean));
	}

	pub fn new_class(engine: &Engine) -> Self {
		return Self::new(engine.classes.class, Data::Class(Class::new()));
	}

	pub fn new_integer(engine: &Engine, integer: usize) -> Self {
		return Self::new(engine.classes.integer, Data::Integer(integer));
	}

	pub fn new_function(engine: &Engine, parameters: &'a Vec<Box<str>>, block: &'a Block) -> Self {
		return Self::new(engine.classes.function, Data::Callable(Box::new(Function::new(engine.scope, parameters, block))));
	}

	pub fn new_primitive(engine: &Engine, callback: &'static dyn for<'b> Fn(&'b mut Engine, Vec<Reference>) -> Reference) -> Self {
		return Self::new(engine.classes.function, Data::Callable(Box::new(Primitive::new(callback))));
	}

	pub fn new_string(engine: &Engine, string: &str) -> Self {
		return Self::new(engine.classes.string, Data::String(string.to_string()));
	}

	pub fn cast(&self, class: Value) {
		if self.class != class {
			panic!();
		}
	}
}
