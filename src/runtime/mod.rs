mod data;

use data::Data;

pub struct Engine {
	values: Vec<Value>,
	value: usize,
}

impl Engine {
	pub fn set_value(&mut self, index: usize) {

	}

	pub fn new_variable(&mut self, a: String) -> usize {
		return 0;
	}
}

struct Value {
	class: usize,
	data: Data,
}
