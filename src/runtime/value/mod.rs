mod data;

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

	pub fn new_boolean(boolean: bool) -> Self {
		return Self::new(0, Data::Boolean(Boolean::new(boolean)));
	}

	pub fn new_class() -> Self {
		return Self::new(0, Data::Class(Class::new()));
	}

	pub fn new_integer(integer: usize) -> Self {
		return Self::new(0, Data::Integer(Integer::new(integer)));
	}

	pub fn new_string(string: &str) -> Self {
		return Self::new(0, Data::String(Text::new(string)));
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
