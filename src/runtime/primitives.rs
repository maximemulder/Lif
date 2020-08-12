use crate::runtime::{ Engine, Object, Reference, Value };

pub struct Primitives {
	pub array:    Value,
	pub boolean:  Value,
	pub class:    Value,
	pub function: Value,
	pub instance: Value,
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
			instance: Value::new_undefined(),
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
		self.primitives.instance = self.new_value(Object::new_class(self));
		self.primitives.integer  = self.new_value(Object::new_class(self));
		self.primitives.string   = self.new_value(Object::new_class(self));

		self.get_object(self.primitives.class).class = self.primitives.class;

		self.new_primitive("assert", Object::new_primitive(self, &primitive_assert));
		self.new_primitive("error",  Object::new_primitive(self, &primitive_error));
		self.new_primitive("exit",   Object::new_primitive(self, &primitive_exit));
		self.new_primitive("new",    Object::new_primitive(self, &primitive_new));
		self.new_primitive("print",  Object::new_primitive(self, &primitive_print));
	}
}

fn primitive_assert(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	if !engine.get_object(engine.read(parameters[0])).data.as_boolean() {
		panic!();
	}

	return engine.new_undefined();
}
fn primitive_error(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	println!("{}", engine.get_cast_string(engine.read(parameters[0])));
	panic!();
}

fn primitive_exit(_: &Engine, _: Vec<Reference>) -> Reference {
	panic!();
}

fn primitive_new(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_object(Object::new_instance(engine, engine.read(parameters[0])));
}

fn primitive_print(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	println!("{}", engine.get_cast_string(engine.read(parameters[0])));
	return engine.new_undefined();
}

fn array_to_string(engine: &Engine, _: Vec<Reference>) -> Reference {
	return engine.new_object(Object::new_string(engine, "ARRAY".to_string()));
}

fn array_copy(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_object(Object::new_array(engine, engine.get_object(engine.read(parameters[0])).data.as_array().clone()));
}

fn array_append(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	engine.get_object(engine.read(parameters[0])).data.as_array_mut().push(engine.new_reference(engine.read(parameters[1])));
	return engine.new_undefined();
}

fn array_prepend(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	engine.get_object(engine.read(parameters[0])).data.as_array_mut().insert(0, engine.new_reference(engine.read(parameters[1])));
	return engine.new_undefined();
}

fn array_insert(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	engine.get_object(engine.read(parameters[0])).data.as_array_mut().insert(
		*engine.get_object(engine.read(parameters[1])).data.as_integer(),
		engine.new_reference(engine.read(parameters[2]))
	);

	return engine.new_undefined();
}

fn array_remove(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	engine.get_object(engine.read(parameters[0])).data.as_array_mut().remove(*engine.get_object(engine.read(parameters[1])).data.as_integer());
	return engine.new_undefined();
}

fn array_access(engine: &Engine, _: Vec<Reference>) -> Reference {
	return engine.new_object(Object::new_string(engine, "FUNCTION".to_string()));
}

fn function_to_string(engine: &Engine, _: Vec<Reference>) -> Reference {
	return engine.new_object(Object::new_string(engine, "FUNCTION".to_string()));
}

fn function_call(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.get_object(engine.read(parameters[0])).data.as_callable().call(engine, parameters[1..].to_vec());
}

fn instance_chain(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	let name = engine.get_object(engine.read(parameters[1])).data.as_string();
	let this = engine.read(parameters[0]);
	if let Some(method) = engine.get_object(engine.get_object(this).class).get_method(engine, name) {
		engine.cheat().this = Some(this);
		return method;
	}

	let instance = engine.get_object(this).data.as_instance_mut();
	return if let Some(&member) = instance.attributes.get(name) {
		member
	} else {
		let member = engine.new_undefined();
		instance.attributes.insert(name.clone(), member);
		member
	}
}

fn integer_to_string(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_object(Object::new_string(engine,
		engine.get_object(engine.read(parameters[0])).data.as_integer().to_string()
	));
}

fn integer_comparison(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_object(Object::new_boolean(engine,
		*engine.get_object(engine.read(parameters[0])).data.as_integer() ==
		*engine.get_object(engine.read(parameters[1])).data.as_integer()
	));
}

fn integer_order_lesser(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_object(Object::new_boolean(engine,
		*engine.get_object(engine.read(parameters[0])).data.as_integer() <
		*engine.get_object(engine.read(parameters[1])).data.as_integer()
	));
}

fn integer_addition(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_object(Object::new_integer(engine,
		*engine.get_object(engine.read(parameters[0])).data.as_integer() +
		*engine.get_object(engine.read(parameters[1])).data.as_integer()
	));
}

fn integer_substraction(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_object(Object::new_integer(engine,
		*engine.get_object(engine.read(parameters[0])).data.as_integer() +
		*engine.get_object(engine.read(parameters[1])).data.as_integer()
	));
}

fn integer_multiplication(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_object(Object::new_integer(engine,
		*engine.get_object(engine.read(parameters[0])).data.as_integer() +
		*engine.get_object(engine.read(parameters[1])).data.as_integer()
	));
}

fn integer_division(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_object(Object::new_integer(engine,
		*engine.get_object(engine.read(parameters[0])).data.as_integer() /
		*engine.get_object(engine.read(parameters[1])).data.as_integer()
	));
}

fn integer_remainder(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_object(Object::new_integer(engine,
		*engine.get_object(engine.read(parameters[0])).data.as_integer() %
		*engine.get_object(engine.read(parameters[1])).data.as_integer()
	));
}

fn object_to_string(engine: &Engine, _: Vec<Reference>) -> Reference {
	return engine.new_object(Object::new_string(engine, "OBJECT".to_string()));
}

fn object_comparison(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_object(Object::new_boolean(engine, engine.read(parameters[0]) == engine.read(parameters[1])));
}

fn object_chain(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	let name = engine.get_object(engine.read(parameters[1])).data.as_string();
	let this = engine.read(parameters[0]);
	if let Some(method) = engine.get_object(engine.get_object(this).class).get_method(engine, name) {
		engine.cheat().this = Some(this);
		return method;
	}

	panic!();
}

fn string_to_string(_: &Engine, parameters: Vec<Reference>) -> Reference {
	return parameters[0];
}

fn string_comparison(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_object(Object::new_boolean(engine,
		engine.get_object(engine.read(parameters[0])).data.as_string() ==
		engine.get_object(engine.read(parameters[1])).data.as_string()
	));
}

fn string_concatenation(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_object(Object::new_string(engine, format!("{}{}",
		engine.get_object(engine.read(parameters[0])).data.as_string(),
		engine.get_object(engine.read(parameters[1])).data.as_string()
	)));
}

fn type_to_string(engine: &Engine, _: Vec<Reference>) -> Reference {
	return engine.new_object(Object::new_string(engine, "TYPE".to_string()));
}

fn type_chain(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	let name = engine.get_object(engine.read(parameters[1])).data.as_string();
	let this = engine.read(parameters[0]);
	if let Some(method) = engine.get_object(engine.get_object(this).class).get_method(engine, name) {
		engine.cheat().this = Some(this);
		return method;
	}

	let class = engine.get_object(this).data.as_class_mut();
	return if let Some(&member) = class.statics.get(name) {
		member
	} else {
		let member = engine.new_undefined();
		class.statics.insert(name.clone(), member);
		member
	}
}

fn type_access(engine: &Engine, _: Vec<Reference>) -> Reference {
	return engine.new_reference(engine.primitives.array);
}
