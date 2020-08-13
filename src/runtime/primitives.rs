use crate::runtime::{ Engine, Object, Reference, Value };
use crate::runtime::object::data::Data;
use crate::runtime::object::class::Class;

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
	pub fn create_class(&self) -> Value {
		return self.new_value(Object::new(self.primitives.class, Data::Class(Class::new())));
	}

	pub fn build_primitives(&mut self) {
		self.primitives.class = self.create_class();

		self.primitives.array    = self.create_class();
		self.primitives.boolean  = self.create_class();
		self.primitives.function = self.create_class();
		self.primitives.instance = self.create_class();
		self.primitives.integer  = self.create_class();
		self.primitives.string   = self.create_class();

		self.get_object(self.primitives.class).class = self.primitives.class;

		self.new_variable("assert", self.new_primitive(&primitive_assert));
		self.new_variable("error",  self.new_primitive(&primitive_error));
		self.new_variable("exit",   self.new_primitive(&primitive_exit));
		self.new_variable("new",    self.new_primitive(&primitive_new));
		self.new_variable("print",  self.new_primitive(&primitive_print));
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
	return engine.new_instance(engine.read(parameters[0]));
}

fn primitive_print(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	println!("{}", engine.get_cast_string(engine.read(parameters[0])));
	return engine.new_undefined();
}

fn array_to_string(engine: &Engine, _: Vec<Reference>) -> Reference {
	return engine.new_string("ARRAY".to_string());
}

fn array_copy(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_array(engine.get_object(engine.read(parameters[0])).data.as_array().clone());
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

fn array_access(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.get_object(engine.read(parameters[0])).data.as_array()[*engine.get_object(engine.read(parameters[1])).data.as_integer()];
}

fn function_to_string(engine: &Engine, _: Vec<Reference>) -> Reference {
	return engine.new_string("FUNCTION".to_string());
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
	return engine.new_string(engine.get_object(engine.read(parameters[0])).data.as_integer().to_string());
}

fn integer_comparison(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_boolean(
		*engine.get_object(engine.read(parameters[0])).data.as_integer() ==
		*engine.get_object(engine.read(parameters[1])).data.as_integer()
	);
}

fn integer_order_lesser(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_boolean(
		*engine.get_object(engine.read(parameters[0])).data.as_integer() <
		*engine.get_object(engine.read(parameters[1])).data.as_integer()
	);
}

fn integer_addition(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_integer(
		*engine.get_object(engine.read(parameters[0])).data.as_integer() +
		*engine.get_object(engine.read(parameters[1])).data.as_integer()
	);
}

fn integer_substraction(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_integer(
		*engine.get_object(engine.read(parameters[0])).data.as_integer() +
		*engine.get_object(engine.read(parameters[1])).data.as_integer()
	);
}

fn integer_multiplication(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_integer(
		*engine.get_object(engine.read(parameters[0])).data.as_integer() +
		*engine.get_object(engine.read(parameters[1])).data.as_integer()
	);
}

fn integer_division(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_integer(
		*engine.get_object(engine.read(parameters[0])).data.as_integer() /
		*engine.get_object(engine.read(parameters[1])).data.as_integer()
	);
}

fn integer_remainder(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_integer(
		*engine.get_object(engine.read(parameters[0])).data.as_integer() %
		*engine.get_object(engine.read(parameters[1])).data.as_integer()
	);
}

fn object_to_string(engine: &Engine, _: Vec<Reference>) -> Reference {
	return engine.new_string("OBJECT".to_string());
}

fn object_comparison(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_boolean(engine.read(parameters[0]) == engine.read(parameters[1]));
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
	return engine.new_boolean(
		engine.get_object(engine.read(parameters[0])).data.as_string() ==
		engine.get_object(engine.read(parameters[1])).data.as_string()
	);
}

fn string_concatenation(engine: &Engine, parameters: Vec<Reference>) -> Reference {
	return engine.new_string(format!("{}{}",
		engine.get_object(engine.read(parameters[0])).data.as_string(),
		engine.get_object(engine.read(parameters[1])).data.as_string()
	));
}

fn type_to_string(engine: &Engine, _: Vec<Reference>) -> Reference {
	return engine.new_string("TYPE".to_string());
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
