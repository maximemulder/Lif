mod data;

use crate::runtime::Engine;
use data::*;

pub struct Value {
	pub class: usize,
	pub data: Data,
}

impl Value {
	pub fn new(class: usize, data: Data) -> Self {
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

	pub fn new_string(engine: &Engine, string: &str) -> Self {
		return Self::new(engine.classes.string, Data::String(string.to_string()));
	}

	pub fn new_undefined() -> Self {
		return Self::new(0, Data::Undefined(()));
	}

	pub fn cast(&self, class: usize) {
		if self.class != class {
			panic!();
		}
	}
}
