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
	fn create_class(&mut self) -> Value {
		return self.new_value(Object::new(self.primitives.class, Data::Class(Class::new(Some(self.primitives.object)))));
	}

	pub fn new_variable_primitive(&mut self, name: &str, callback: &'static dyn for<'b> Fn(&'b mut Engine, Vec<Reference>) -> Reference) {
		let primitive = self.new_primitive(callback);
		self.new_variable(name, primitive);
	}

	pub fn new_variable_value(&mut self, name: &str, value: Value) {
		let reference = self.new_reference(value);
		self.new_variable(name, reference);
	}

	fn new_method_primitive(&mut self, value: Value, name: &str, callback: &'static dyn for<'b> Fn(&'b mut Engine, Vec<Reference>) -> Reference) {
		let primitive = self.new_primitive(callback);
		self.get_object_mut(value).data_class_mut().methods.insert(name.to_string(), primitive);
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

		self.get_object_mut(self.primitives.class).class = self.primitives.class;
		self.get_object_mut(self.primitives.class).data_class_mut().parent = Some(self.primitives.object);
		self.get_object_mut(self.primitives.object).data_class_mut().parent = None;

		self.new_variable_primitive("assert", &primitive_assert);
		self.new_variable_primitive("error",  &primitive_error);
		self.new_variable_primitive("exit",   &primitive_exit);
		self.new_variable_primitive("new",    &primitive_new);
		self.new_variable_primitive("print",  &primitive_print);

		let array    = self.primitives.array;
		let boolean  = self.primitives.boolean;
		let class    = self.primitives.class;
		let function = self.primitives.function;
		let instance = self.primitives.instance;
		let integer  = self.primitives.integer;
		let object   = self.primitives.object;
		let string   = self.primitives.string;

		self.new_variable_value("Array",    array);
		self.new_variable_value("Boolean",  self.primitives.boolean);
		self.new_variable_value("Class",    self.primitives.class);
		self.new_variable_value("Function", self.primitives.function);
		self.new_variable_value("Instance", self.primitives.instance);
		self.new_variable_value("Integer",  self.primitives.integer);
		self.new_variable_value("Object",   self.primitives.object);
		self.new_variable_value("String",   self.primitives.string);

		self.new_method_primitive(array, "to_string", &array_to_string);
		self.new_method_primitive(array, "copy",      &array_copy);
		self.new_method_primitive(array, "append",    &array_append);
		self.new_method_primitive(array, "prepend",   &array_prepend);
		self.new_method_primitive(array, "insert",    &array_insert);
		self.new_method_primitive(array, "remove",    &array_remove);
		self.new_method_primitive(array, "[]",        &array_access);

		self.new_method_primitive(boolean, "to_string", &boolean_to_string);
		self.new_method_primitive(boolean, "==",        &boolean_comparison);

		self.new_method_primitive(class, "to_string", &class_to_string);
		self.new_method_primitive(class, ".",         &class_chain);

		self.new_method_primitive(function, "to_string", &function_to_string);
		self.new_method_primitive(function, "()",        &function_call);

		self.new_method_primitive(instance, "to_string", &instance_to_string);
		self.new_method_primitive(instance, ".",         &instance_chain);

		self.new_method_primitive(integer, "to_string", &integer_to_string);
		self.new_method_primitive(integer, "==",        &integer_comparison);
		self.new_method_primitive(integer, "<",         &integer_lesser);
		self.new_method_primitive(integer, "+",         &integer_addition);
		self.new_method_primitive(integer, "-",         &integer_subtraction);
		self.new_method_primitive(integer, "*",         &integer_multiplication);
		self.new_method_primitive(integer, "/",         &integer_division);
		self.new_method_primitive(integer, "%",         &integer_remainder);

		self.new_method_primitive(object, "==", &object_comparison);
		self.new_method_primitive(object, "!=", &object_difference);
		self.new_method_primitive(object, ">",  &object_greater);
		self.new_method_primitive(object, "<=", &object_lesser_equal);
		self.new_method_primitive(object, ">=", &object_greater_equal);
		self.new_method_primitive(object, ".",  &object_chain);

		self.new_method_primitive(string, "to_string", &string_to_string);
		self.new_method_primitive(string, "==",        &string_comparison);
		self.new_method_primitive(string, "+",         &string_concatenation);
	}
}

fn primitive_assert(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	if !engine.get_object(engine.read(arguments[0])).data_boolean() {
		panic!();
	}

	return engine.new_undefined();
}

fn primitive_error(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	let reference = engine.call_method(arguments[0], "to_string", Vec::new());
	println!("{}", engine.get_object(engine.read(reference)).data_string());
	panic!();
}

fn primitive_exit(_: &mut Engine, _: Vec<Reference>) -> Reference {
	panic!();
}

fn primitive_new(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_instance(engine.read(arguments[0]));
}

fn primitive_print(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	let reference = engine.call_method(arguments[0], "to_string", Vec::new());
	println!("{}", engine.get_object(engine.read(reference)).data_string());
	return engine.new_undefined();
}

fn array_to_string(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	let mut string = String::from("[");
	let elements = engine.get_object(engine.read(arguments[0])).data_array().clone();
	for element in elements.iter() {
		let reference = engine.call_method(*element, "to_string", Vec::new());
		string.push_str(engine.get_object(engine.read(reference)).data_string());
		string.push_str(", ");
	}

	if !elements.is_empty() {
		string.truncate(string.len() - 2);
	}

	string.push_str("]");
	return engine.new_string(string);
}

fn array_copy(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_array(engine.get_object(engine.read(arguments[0])).data_array().clone());
}

fn array_append(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	let reference = engine.new_reference(engine.read(arguments[1]));
	engine.get_object_mut(engine.read(arguments[0])).data_array_mut().push(reference);
	return engine.new_undefined();
}

fn array_prepend(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	let reference = engine.new_reference(engine.read(arguments[1]));
	engine.get_object_mut(engine.read(arguments[0])).data_array_mut().insert(0, reference);
	return engine.new_undefined();
}

fn array_insert(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	let index = *engine.get_object(engine.read(arguments[1])).data_integer();
	let element = engine.new_reference(engine.read(arguments[2]));
	engine.get_object_mut(engine.read(arguments[0])).data_array_mut().insert(index, element);

	return engine.new_undefined();
}

fn array_remove(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	let index = *engine.get_object(engine.read(arguments[1])).data_integer();
	engine.get_object_mut(engine.read(arguments[0])).data_array_mut().remove(index);
	return engine.new_undefined();
}

fn array_access(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	return engine.get_object(engine.read(arguments[0])).data_array()[*engine.get_object(engine.read(arguments[1])).data_integer()];
}

fn boolean_to_string(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_string(engine.get_object(engine.read(arguments[0])).data_boolean().to_string());
}

fn boolean_comparison(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_boolean(engine.get_object(engine.read(arguments[0])).data_boolean() == engine.get_object(engine.read(arguments[1])).data_boolean());
}

fn class_to_string(engine: &mut Engine, _: Vec<Reference>) -> Reference {
	return engine.new_string("CLASS".to_string());
}

fn class_chain(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	let name = engine.get_object(engine.read(arguments[1])).data_string().clone();
	let this = engine.read(arguments[0]);
	if let Some(method) = engine.get_object(this).get_method(engine, &name) {
		engine.this = Some(arguments[0]);
		return method;
	}

	let member = engine.new_undefined();
	let two = engine.get_object_mut(this);
	let class = two.data_class_mut();
	return if let Some(&member) = class.statics.get(&name) {
		member
	} else {
		class.statics.insert(name.clone(), member);
		member
	}
}

fn class_access(engine: &mut Engine, _: Vec<Reference>) -> Reference {
	return engine.new_reference(engine.primitives.array);
}

fn function_to_string(engine: &mut Engine, _: Vec<Reference>) -> Reference {
	return engine.new_string("FUNCTION".to_string());
}

fn function_call(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	return engine.get_object(engine.read(arguments[0])).data_callable().duplicate().call(engine, arguments[1..].to_vec());
}

fn instance_to_string(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	let mut string = String::from("{");
	let attributes = &engine.get_object(engine.read(arguments[0])).data_instance().attributes.clone();
	for (name, attribute) in attributes {
		string.push_str(&name);
		string.push_str(": ");
		let a = engine.call_method(*attribute, "to_string", Vec::new());
		string.push_str(engine.get_object(engine.read(a)).data_string());
		string.push_str(", ");
	}

	if !attributes.is_empty() {
		string.truncate(string.len() - 2);
	}

	string.push_str("}");
	return engine.new_string(string);
}

fn instance_chain(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	let name = engine.get_object(engine.read(arguments[1])).data_string().clone();
	let this = engine.read(arguments[0]);
	if let Some(method) = engine.get_object(this).get_method(engine, &name) {
		engine.this = Some(arguments[0]);
		return method;
	}

	let member = engine.new_undefined();
	let two = engine.get_object_mut(this);
	let instance = two.data_instance_mut();
	return if let Some(&member) = instance.attributes.get(&name) {
		member
	} else {
		instance.attributes.insert(name.clone(), member);
		member
	}
}

fn integer_to_string(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	let string = engine.get_object(engine.read(arguments[0])).data_integer().to_string();
	return engine.new_string(string);
}

fn integer_comparison(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_boolean(
		*engine.get_object(engine.read(arguments[0])).data_integer() ==
		*engine.get_object(engine.read(arguments[1])).data_integer()
	);
}

fn integer_lesser(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_boolean(
		*engine.get_object(engine.read(arguments[0])).data_integer() <
		*engine.get_object(engine.read(arguments[1])).data_integer()
	);
}

fn integer_addition(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_integer(
		*engine.get_object(engine.read(arguments[0])).data_integer() +
		*engine.get_object(engine.read(arguments[1])).data_integer()
	);
}

fn integer_subtraction(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_integer(
		*engine.get_object(engine.read(arguments[0])).data_integer() +
		*engine.get_object(engine.read(arguments[1])).data_integer()
	);
}

fn integer_multiplication(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_integer(
		*engine.get_object(engine.read(arguments[0])).data_integer() +
		*engine.get_object(engine.read(arguments[1])).data_integer()
	);
}

fn integer_division(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_integer(
		*engine.get_object(engine.read(arguments[0])).data_integer() /
		*engine.get_object(engine.read(arguments[1])).data_integer()
	);
}

fn integer_remainder(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_integer(
		*engine.get_object(engine.read(arguments[0])).data_integer() %
		*engine.get_object(engine.read(arguments[1])).data_integer()
	);
}

fn object_comparison(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_boolean(engine.read(arguments[0]) == engine.read(arguments[1]));
}

fn object_difference(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	let reference = engine.call_method_self(arguments[0], "==", arguments);
	return engine.new_boolean(!engine.get_object(engine.read(reference)).data_boolean());
}

fn object_greater(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	let left  = engine.call_method_self(arguments[0], "<", arguments.clone());
	let right = engine.call_method_self(arguments[0], "==", arguments.clone());
	return engine.new_boolean(
		!engine.get_object(engine.read(left)).data_boolean() &&
		!engine.get_object(engine.read(right)).data_boolean()
	);
}

fn object_greater_equal(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	let reference = engine.call_method_self(arguments[0], "<", arguments);
	return engine.new_boolean(!engine.get_object(engine.read(reference)).data_boolean());
}

fn object_lesser_equal(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	let left  = engine.call_method_self(arguments[0], "<", arguments.clone());
	let right = engine.call_method_self(arguments[0], "==", arguments.clone());
	return engine.new_boolean(
		*engine.get_object(engine.read(left)).data_boolean() ||
		*engine.get_object(engine.read(right)).data_boolean()
	);
}

fn object_chain(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	let one = engine.get_object(engine.read(arguments[1]));
	let name = one.data_string();
	let this = engine.read(arguments[0]);
	if let Some(method) = engine.get_object(this).get_method(engine, name) {
		engine.this = Some(arguments[0]);
		return method;
	}

	panic!();
}

fn string_to_string(_: &mut Engine, arguments: Vec<Reference>) -> Reference {
	return arguments[0];
}

fn string_comparison(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	return engine.new_boolean(
		engine.get_object(engine.read(arguments[0])).data_string() ==
		engine.get_object(engine.read(arguments[1])).data_string()
	);
}

fn string_concatenation(engine: &mut Engine, arguments: Vec<Reference>) -> Reference {
	let left  = arguments[0];
	let right = engine.call_method(arguments[1], "to_string", Vec::new());
	return engine.new_string(format!("{}{}",
		engine.get_object(engine.read(left)).data_string(),
		engine.get_object(engine.read(right)).data_string()
	));
}