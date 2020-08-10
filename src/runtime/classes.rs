use crate::runtime::{ Engine, Value, Object };

pub struct Classes {
	pub array:    Value,
	pub boolean:  Value,
	pub class:    Value,
	pub function: Value,
	pub integer:  Value,
	pub string:   Value,
}

impl Classes {
	pub fn new() -> Self {
		return Classes {
			array:    Value::new_undefined(),
			boolean:  Value::new_undefined(),
			class:    Value::new_undefined(),
			function: Value::new_undefined(),
			integer:  Value::new_undefined(),
			string:   Value::new_undefined(),
		};
	}
}

impl Engine<'_> {
	pub fn build_classes(&mut self) {
		self.classes.class    = self.new_value(Object::new_class(self));

		self.classes.array    = self.new_value(Object::new_class(self));
		self.classes.boolean  = self.new_value(Object::new_class(self));
		self.classes.function = self.new_value(Object::new_class(self));
		self.classes.integer  = self.new_value(Object::new_class(self));
		self.classes.string   = self.new_value(Object::new_class(self));

		self.get_object(self.classes.class).class = self.classes.class;

		let primitive = self.new_value(Object::new_primitive(self, &|engine, _parameters| engine.new_undefined()));
		self.get_object(self.classes.integer).data.as_class().methods.insert("yo".to_string(), primitive);
	}
}
