use crate::cheat;
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
	pub object:   Value,
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
			object:   Value::new_undefined(),
			string:   Value::new_undefined(),
		};
	}
}

impl<'a> Engine<'a> {
	fn create_class(&self) -> Value {
		return self.new_value(Object::new(self.primitives.class, Data::Class(Class::new(Some(self.primitives.object)))));
	}

	fn get_class(&self, value: Value) -> &mut Class {
		return self.get_object(value).data.as_class_mut();
	}

	fn new_method(&self, class: &mut Class, name: &str, callback: &'static dyn for<'b> Fn(&'b Engine, Vec<Reference>) -> Reference) {
		class.methods.insert(name.to_string(), self.new_primitive(callback));
	}

	pub fn populate(&mut self) {
		self.primitives.class  = self.create_class();
		self.primitives.object = self.create_class();

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

		self.new_variable("Array",    self.new_reference(self.primitives.array));
		self.new_variable("Boolean",  self.new_reference(self.primitives.array));
		self.new_variable("Class",    self.new_reference(self.primitives.array));
		self.new_variable("Function", self.new_reference(self.primitives.array));
		self.new_variable("Instance", self.new_reference(self.primitives.array));
		self.new_variable("Integer",  self.new_reference(self.primitives.array));
		self.new_variable("Object",   self.new_reference(self.primitives.array));
		self.new_variable("String",   self.new_reference(self.primitives.array));


		let array = self.get_class(self.primitives.array);
		self.new_method(array, "to_string", &array_to_string);
		self.new_method(array, "copy",      &array_copy);
		self.new_method(array, "append",    &array_append);
		self.new_method(array, "prepend",   &array_prepend);
		self.new_method(array, "insert",    &array_insert);
		self.new_method(array, "remove",    &array_remove);
		self.new_method(array, "[]",        &array_access);

		let boolean = self.get_class(self.primitives.boolean);
		self.new_method(boolean, "to_string", &boolean_to_string);
		self.new_method(boolean, "==",        &boolean_comparison);

		let class = self.get_class(self.primitives.class);
		self.new_method(class, "to_string", &class_to_string);
		self.new_method(class, ".",         &class_chain);

		let function = self.get_class(self.primitives.function);
		self.new_method(function, "to_string", &function_to_string);
		self.new_method(function, "()",        &function_call);

		let instance = self.get_class(self.primitives.instance);
		self.new_method(instance, "to_string", &instance_to_string);
		self.new_method(instance, ".",         &instance_chain);

		let integer = self.get_class(self.primitives.integer);
		self.new_method(integer, "to_string", &integer_to_string);
		self.new_method(integer, "==",        &integer_comparison);
		self.new_method(integer, "<",         &integer_lesser);
		self.new_method(integer, "+",         &integer_addition);
		self.new_method(integer, "-",         &integer_subtraction);
		self.new_method(integer, "*",         &integer_multiplication);
		self.new_method(integer, "/",         &integer_division);
		self.new_method(integer, "%",         &integer_remainder);

		let object = self.get_class(self.primitives.object);
		self.new_method(object, "==", &object_comparison);
		self.new_method(object, "!=", &object_difference);
		self.new_method(object, ">",  &object_greater);
		self.new_method(object, "<=", &object_lesser_equal);
		self.new_method(object, ">=", &object_greater_equal);
		self.new_method(object, ".",  &object_chain);

		let string = self.get_class(self.primitives.string);
		self.new_method(string, "to_string", &string_to_string);
		self.new_method(string, "==",        &string_comparison);
		self.new_method(string, "+",         &string_concatenation);
	}
}

fn primitive_assert(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	if !engine.get_object(engine.read(arguments[0])).data.as_boolean() {
		panic!();
	}

	return engine.new_undefined();
}

fn primitive_error(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	println!("{}", engine.get_object(engine.read(engine.call_method(arguments[0], "to_string", Vec::new()))).data.as_string());
	panic!();
}

fn primitive_exit(_: &Engine, _: Vec<Reference>) -> Reference {
	panic!();
}

fn primitive_new(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_instance(engine.read(arguments[0]));
}

fn primitive_print(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	println!("{}", engine.get_object(engine.read(engine.call_method(arguments[0], "to_string", Vec::new()))).data.as_string());
	return engine.new_undefined();
}

fn array_to_string(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	let mut string = String::from("[");
	let elements = engine.get_object(engine.read(arguments[0])).data.as_array();
	for element in elements {
		string.push_str(engine.get_object(engine.read(engine.call_method(*element, "to_string", Vec::new()))).data.as_string());
		string.push_str(", ");
	}

	if !elements.is_empty() {
		string.truncate(string.len() - 2);
	}

	string.push_str("]");
	return engine.new_string(string);
}

fn array_copy(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_array(engine.get_object(engine.read(arguments[0])).data.as_array().clone());
}

fn array_append(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	engine.get_object(engine.read(arguments[0])).data.as_array_mut().push(engine.new_reference(engine.read(arguments[1])));
	return engine.new_undefined();
}

fn array_prepend(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	engine.get_object(engine.read(arguments[0])).data.as_array_mut().insert(0, engine.new_reference(engine.read(arguments[1])));
	return engine.new_undefined();
}

fn array_insert(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	engine.get_object(engine.read(arguments[0])).data.as_array_mut().insert(
		*engine.get_object(engine.read(arguments[1])).data.as_integer(),
		engine.new_reference(engine.read(arguments[2]))
	);

	return engine.new_undefined();
}

fn array_remove(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	engine.get_object(engine.read(arguments[0])).data.as_array_mut().remove(*engine.get_object(engine.read(arguments[1])).data.as_integer());
	return engine.new_undefined();
}

fn array_access(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.get_object(engine.read(arguments[0])).data.as_array()[*engine.get_object(engine.read(arguments[1])).data.as_integer()];
}

fn boolean_to_string(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_string(engine.get_object(engine.read(arguments[0])).data.as_boolean().to_string());
}

fn boolean_comparison(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_boolean(engine.get_object(engine.read(arguments[0])).data.as_boolean() == engine.get_object(engine.read(arguments[1])).data.as_boolean());
}

fn class_to_string(engine: &Engine, _: Vec<Reference>) -> Reference {
	return engine.new_string("CLASS".to_string());
}

fn class_chain(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	let name = engine.get_object(engine.read(arguments[1])).data.as_string();
	let this = engine.read(arguments[0]);
	if let Some(method) = engine.get_object(engine.get_object(this).class).get_method(engine, name) {
		cheat(engine).this = Some(arguments[0]);
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

fn class_access(engine: &Engine, _: Vec<Reference>) -> Reference {
	return engine.new_reference(engine.primitives.array);
}


fn function_to_string(engine: &Engine, _: Vec<Reference>) -> Reference {
	return engine.new_string("FUNCTION".to_string());
}

fn function_call(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.get_object(engine.read(arguments[0])).data.as_callable().call(engine, arguments[1..].to_vec());
}

fn instance_to_string(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	let mut string = String::from("{");
	let attributes = &engine.get_object(engine.read(arguments[0])).data.as_instance().attributes;
	for (name, attribute) in attributes {
		string.push_str(name);
		string.push_str(": ");
		string.push_str(engine.get_object(engine.read(engine.call_method(*attribute, "to_string", Vec::new()))).data.as_string());
		string.push_str(", ");
	}

	if !attributes.is_empty() {
		string.truncate(string.len() - 2);
	}

	string.push_str("}");
	return engine.new_string(string);
}

fn instance_chain(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	let name = engine.get_object(engine.read(arguments[1])).data.as_string();
	let this = engine.read(arguments[0]);
	if let Some(method) = engine.get_object(engine.get_object(this).class).get_method(engine, name) {
		cheat(engine).this = Some(arguments[0]);
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

fn integer_to_string(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_string(engine.get_object(engine.read(arguments[0])).data.as_integer().to_string());
}

fn integer_comparison(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_boolean(
		*engine.get_object(engine.read(arguments[0])).data.as_integer() ==
		*engine.get_object(engine.read(arguments[1])).data.as_integer()
	);
}

fn integer_lesser(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_boolean(
		*engine.get_object(engine.read(arguments[0])).data.as_integer() <
		*engine.get_object(engine.read(arguments[1])).data.as_integer()
	);
}

fn integer_addition(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_integer(
		*engine.get_object(engine.read(arguments[0])).data.as_integer() +
		*engine.get_object(engine.read(arguments[1])).data.as_integer()
	);
}

fn integer_subtraction(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_integer(
		*engine.get_object(engine.read(arguments[0])).data.as_integer() +
		*engine.get_object(engine.read(arguments[1])).data.as_integer()
	);
}

fn integer_multiplication(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_integer(
		*engine.get_object(engine.read(arguments[0])).data.as_integer() +
		*engine.get_object(engine.read(arguments[1])).data.as_integer()
	);
}

fn integer_division(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_integer(
		*engine.get_object(engine.read(arguments[0])).data.as_integer() /
		*engine.get_object(engine.read(arguments[1])).data.as_integer()
	);
}

fn integer_remainder(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_integer(
		*engine.get_object(engine.read(arguments[0])).data.as_integer() %
		*engine.get_object(engine.read(arguments[1])).data.as_integer()
	);
}

fn object_assignment(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	engine.write(arguments[0], engine.read(arguments[1]));
	return engine.new_undefined();
}

fn object_comparison(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_boolean(engine.read(arguments[0]) == engine.read(arguments[1]));
}

fn object_difference(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_boolean(!engine.get_object(engine.read(engine.call_method_self(arguments[0], "==", arguments))).data.as_boolean());
}

fn object_greater(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_boolean(
		!engine.get_object(engine.read(engine.call_method_self(arguments[0], "<", arguments.clone()))).data.as_boolean() &&
		!engine.get_object(engine.read(engine.call_method_self(arguments[0], "==", arguments))).data.as_boolean()
	);
}

fn object_greater_equal(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_boolean(!engine.get_object(engine.read(engine.call_method_self(arguments[0], "<", arguments))).data.as_boolean());
}

fn object_lesser_equal(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_boolean(
		*engine.get_object(engine.read(engine.call_method_self(arguments[0], "<", arguments.clone()))).data.as_boolean() ||
		*engine.get_object(engine.read(engine.call_method_self(arguments[0], "==", arguments))).data.as_boolean()
	);
}

fn object_chain(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	let name = engine.get_object(engine.read(arguments[1])).data.as_string();
	let this = engine.read(arguments[0]);
	if let Some(method) = engine.get_object(engine.get_object(this).class).get_method(engine, name) {
		cheat(engine).this = Some(arguments[0]);
		return method;
	}

	panic!();
}

fn string_to_string(_: &Engine, arguments: Vec<Reference>) -> Reference {
	return arguments[0];
}

fn string_comparison(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_boolean(
		engine.get_object(engine.read(arguments[0])).data.as_string() ==
		engine.get_object(engine.read(arguments[1])).data.as_string()
	);
}

fn string_concatenation(engine: &Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_string(format!("{}{}",
		engine.get_object(engine.read(arguments[0])).data.as_string(),
		engine.get_object(engine.read(engine.call_method(arguments[1], "to_string", Vec::new()))).data.as_string()
	));
}