use crate::runtime::{ Engine, Object, Reference, Value };

pub struct Primitives {
	pub array:    Value,
	pub boolean:  Value,
	pub class:    Value,
	pub function: Value,
	pub integer:  Value,
	pub string:   Value,
}

impl Primitives {
	pub fn new() -> Self {
		return Self {
			array:    Value::new_undefined(),
			boolean:  Value::new_undefined(),
			class:    Value::new_undefined(),
			function: Value::new_undefined(),
			integer:  Value::new_undefined(),
			string:   Value::new_undefined(),
		};
	}
}

impl<'a> Engine<'a> {
	pub fn new_primitive(&self, name: &str, object: Object<'a>) {
		self.new_variable(name, self.new_reference(self.new_value(object)));
	}

	pub fn build_primitives(&mut self) {
		self.primitives.class    = self.new_value(Object::new_class(self));

		self.primitives.array    = self.new_value(Object::new_class(self));
		self.primitives.boolean  = self.new_value(Object::new_class(self));
		self.primitives.function = self.new_value(Object::new_class(self));
		self.primitives.integer  = self.new_value(Object::new_class(self));
		self.primitives.string   = self.new_value(Object::new_class(self));

		self.get_object(self.primitives.class).class = self.primitives.class;

		self.new_primitive("print", Object::new_primitive(self, &print));
	}
}

fn print(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	println!("{}", engine.get_cast_string(engine.read(parameters[0])));
	return engine.new_undefined();
}
