use super::data::*;

pub struct Value {
	class: usize,
	data: Data,
}

impl Value {
	pub fn new(class: usize, data: Data) -> Self {
		return Self {
			class,
			data,
		};
	}

	pub fn new_integer(integer: usize) -> Self {
		return Self::new(0, Data::Integer(Integer::new(integer)));
	}

	pub fn new_string(string: &str) -> Self {
		return Self::new(0, Data::String(Text::new(string)));
	}
}
