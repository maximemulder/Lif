use crate::runtime::Engine;
use crate::runtime::Value;

pub struct Classes {
	pub boolean: usize,
}

impl Classes {
	pub fn new() -> Self {
		return Classes {
			boolean: 0,
		};
	}
}

impl Engine {
	pub fn build_classes(&mut self) {
		self.classes.boolean = self.new_value(Value::new_class());
	}
}
