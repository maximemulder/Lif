use crate::runtime::Engine;
use crate::runtime::Value;

pub struct Classes {
	pub array: usize,
	pub boolean: usize,
	pub class: usize,
	pub function: usize,
	pub integer: usize,
	pub string: usize,
}

impl Classes {
	pub fn new() -> Self {
		return Classes {
			array: 0,
			boolean: 0,
			class: 0,
			function: 0,
			integer: 0,
			string: 0,
		};
	}
}

impl Engine {
	pub fn build_classes(&mut self) {
		self.classes.class    = self.new_value(Value::new_class(self));

		self.classes.array    = self.new_value(Value::new_class(self));
		self.classes.boolean  = self.new_value(Value::new_class(self));
		self.classes.function = self.new_value(Value::new_class(self));
		self.classes.integer  = self.new_value(Value::new_class(self));
		self.classes.string   = self.new_value(Value::new_class(self));

		self.get_value(self.classes.class).class = self.classes.class;
	}
}
