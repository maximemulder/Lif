use crate::runtime::{ Engine, Reference, Value };
use crate::runtime::object::data::Data;
use crate::runtime::object::class::Class;

pub struct Environment<'a> {
	pub array:    Value<'a>,
	pub boolean:  Value<'a>,
	pub class:    Value<'a>,
	pub function: Value<'a>,
	pub instance: Value<'a>,
	pub integer:  Value<'a>,
	pub object:   Value<'a>,
	pub string:   Value<'a>,
}

impl<'a> Environment<'a> {
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
	fn create_class(&mut self) -> Value<'a> {
		return Value::create(self.environment.class, Data::Class(Class::new(Some(self.environment.object))));
	}

	pub fn new_variable_primitive(&mut self, name: &str, callback: &'a dyn Fn(&mut Engine<'a>, Vec<Reference<'a>>) -> Reference<'a>) {
		let primitive = self.new_primitive(callback);
		self.new_variable(name, primitive);
	}

	pub fn new_variable_value(&mut self, name: &str, value: Value<'a>) {
		self.new_variable(name, Reference::new(value));
	}

	fn new_method_primitive(&mut self, mut value: Value<'a>, name: &str, callback: &'a dyn Fn(&mut Engine<'a>, Vec<Reference<'a>>) -> Reference<'a>) {
		let primitive = self.new_primitive(callback);
		value.object_mut().data_class_mut().methods.insert(name.to_string(), primitive);
	}

	pub fn populate(&mut self) {
		self.environment.class  = self.create_class();
		self.environment.object = self.create_class();

		self.environment.array    = self.create_class();
		self.environment.boolean  = self.create_class();
		self.environment.function = self.create_class();
		self.environment.instance = self.create_class();
		self.environment.integer  = self.create_class();
		self.environment.string   = self.create_class();

		self.environment.class.object_mut().class = self.environment.class;
		self.environment.class.object_mut().data_class_mut().parent = Some(self.environment.object);
		self.environment.object.object_mut().data_class_mut().parent = None;

		self.new_variable_primitive("assert", &primitive_assert);
		self.new_variable_primitive("error",  &primitive_error);
		self.new_variable_primitive("exit",   &primitive_exit);
		self.new_variable_primitive("new",    &primitive_new);
		self.new_variable_primitive("print",  &primitive_print);

		let array    = self.environment.array;
		let boolean  = self.environment.boolean;
		let class    = self.environment.class;
		let function = self.environment.function;
		let instance = self.environment.instance;
		let integer  = self.environment.integer;
		let object   = self.environment.object;
		let string   = self.environment.string;

		self.new_variable_value("Array",    array);
		self.new_variable_value("Boolean",  boolean);
		self.new_variable_value("Class",    class);
		self.new_variable_value("Function", function);
		self.new_variable_value("Instance", instance);
		self.new_variable_value("Integer",  integer);
		self.new_variable_value("Object",   object);
		self.new_variable_value("String",   string);

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

fn primitive_assert<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	if !arguments[0].object_ref().data_boolean() {
		panic!();
	}

	return Reference::new_undefined();
}

fn primitive_error<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	let reference = engine.call_method(arguments[0], "to_string", Vec::new());
	println!("{}", reference.object_ref().data_string());
	panic!();
}

fn primitive_exit<'a>(_: &mut Engine<'a>, _: Vec<Reference<'a>>) -> Reference<'a> {
	panic!();
}

fn primitive_new<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	return engine.new_instance(*arguments[0].value_ref());
}

fn primitive_print<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	let reference = engine.call_method(arguments[0], "to_string", Vec::new());
	println!("{}", reference.object_ref().data_string());
	return Reference::new_undefined();
}

fn array_to_string<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	let mut string = String::from("[");
	let elements = arguments[0].object_ref().data_array().clone();
	for element in elements.iter() {
		let reference = engine.call_method(*element, "to_string", Vec::new());
		string.push_str(reference.object_ref().data_string());
		string.push_str(", ");
	}

	if !elements.is_empty() {
		string.truncate(string.len() - 2);
	}

	string.push_str("]");
	return engine.new_string(string);
}

fn array_copy<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	return engine.new_array(arguments[0].object_ref().data_array().clone());
}

fn array_append<'a>(engine: &mut Engine<'a>, mut arguments: Vec<Reference<'a>>) -> Reference<'a> {
	let reference = arguments[1].clone();
	arguments[0].object_mut().data_array_mut().push(reference);
	return Reference::new_undefined();
}

fn array_prepend<'a>(engine: &mut Engine<'a>, mut arguments: Vec<Reference<'a>>) -> Reference<'a> {
	let reference = arguments[1].clone();
	arguments[0].object_mut().data_array_mut().insert(0, reference);
	return Reference::new_undefined();
}

fn array_insert<'a>(engine: &mut Engine<'a>, mut arguments: Vec<Reference<'a>>) -> Reference<'a> {
	let index = *arguments[1].object_ref().data_integer();
	let element = arguments[2].clone();
	arguments[0].object_mut().data_array_mut().insert(index, element);

	return Reference::new_undefined();
}

fn array_remove<'a>(engine: &mut Engine<'a>, mut arguments: Vec<Reference<'a>>) -> Reference<'a> {
	let index = *arguments[1].object_ref().data_integer();
	arguments[0].object_mut().data_array_mut().remove(index);
	return Reference::new_undefined();
}

fn array_access<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	return arguments[0].object_ref().data_array()[*arguments[1].object_ref().data_integer()];
}

fn boolean_to_string<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	return engine.new_string(arguments[0].object_ref().data_boolean().to_string());
}

fn boolean_comparison<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	return engine.new_boolean(arguments[0].object_ref().data_boolean() == arguments[1].object_ref().data_boolean());
}

fn class_to_string<'a>(engine: &mut Engine<'a>, _: Vec<Reference<'a>>) -> Reference<'a> {
	return engine.new_string("CLASS".to_string());
}

fn class_chain<'a>(engine: &mut Engine<'a>, mut arguments: Vec<Reference<'a>>) -> Reference<'a> {
	let name = arguments[1].object_ref().data_string().clone();
	let this = arguments[0].value_mut();
	if let Some(method) = this.object_ref().get_method(engine, &name) {
		engine.this = Some(arguments[0]);
		return method;
	}

	let member = Reference::new_undefined();
	let two = this.object_mut();
	let class = two.data_class_mut();
	return if let Some(&member) = class.statics.get(&name) {
		member
	} else {
		class.statics.insert(name.clone(), member);
		member
	}
}

fn class_access<'a>(engine: &mut Engine<'a>, _: Vec<Reference<'a>>) -> Reference<'a> {
	return Reference::new(engine.environment.array);
}

fn function_to_string<'a>(engine: &mut Engine<'a>, _: Vec<Reference<'a>>) -> Reference<'a> {
	return engine.new_string("FUNCTION".to_string());
}

fn function_call<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	return arguments[0].object_ref().data_callable().duplicate().call(engine, arguments[1..].to_vec());
}

fn instance_to_string<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	let mut string = String::from("{");
	let attributes = &arguments[0].object_ref().data_instance().attributes.clone();
	for (name, attribute) in attributes {
		string.push_str(&name);
		string.push_str(": ");
		let a = engine.call_method(*attribute, "to_string", Vec::new());
		string.push_str(a.object_ref().data_string());
		string.push_str(", ");
	}

	if !attributes.is_empty() {
		string.truncate(string.len() - 2);
	}

	string.push_str("}");
	return engine.new_string(string);
}

fn instance_chain<'a>(engine: &mut Engine<'a>, mut arguments: Vec<Reference<'a>>) -> Reference<'a> {
	let name = arguments[1].object_ref().data_string().clone();
	let this = arguments[0].value_mut();
	if let Some(method) = this.object_ref().get_method(engine, &name) {
		engine.this = Some(arguments[0]);
		return method;
	}

	let member = Reference::new_undefined();
	let two = this.object_mut();
	let instance = two.data_instance_mut();
	return if let Some(&member) = instance.attributes.get(&name) {
		member
	} else {
		instance.attributes.insert(name.clone(), member);
		member
	}
}

fn integer_to_string<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	let string = arguments[0].object_ref().data_integer().to_string();
	return engine.new_string(string);
}

fn integer_comparison<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	return engine.new_boolean(
		*arguments[0].object_ref().data_integer() ==
		*arguments[1].object_ref().data_integer()
	);
}

fn integer_lesser<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	return engine.new_boolean(
		*arguments[0].object_ref().data_integer() <
		*arguments[1].object_ref().data_integer()
	);
}

fn integer_addition<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	return engine.new_integer(
		*arguments[0].object_ref().data_integer() +
		*arguments[1].object_ref().data_integer()
	);
}

fn integer_subtraction<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	return engine.new_integer(
		*arguments[0].object_ref().data_integer() +
		*arguments[1].object_ref().data_integer()
	);
}

fn integer_multiplication<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	return engine.new_integer(
		*arguments[0].object_ref().data_integer() +
		*arguments[1].object_ref().data_integer()
	);
}

fn integer_division<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	return engine.new_integer(
		*arguments[0].object_ref().data_integer() /
		*arguments[1].object_ref().data_integer()
	);
}

fn integer_remainder<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	return engine.new_integer(
		*arguments[0].object_ref().data_integer() %
		*arguments[1].object_ref().data_integer()
	);
}

fn object_comparison<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	return engine.new_boolean(arguments[0].value_ref() == arguments[1].value_ref());
}

fn object_difference<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	let reference = engine.call_method_self(arguments[0], "==", arguments);
	return engine.new_boolean(!reference.object_ref().data_boolean());
}

fn object_greater<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	let left  = engine.call_method_self(arguments[0], "<", arguments.clone());
	let right = engine.call_method_self(arguments[0], "==", arguments.clone());
	return engine.new_boolean(
		!left.object_ref().data_boolean() &&
		!right.object_ref().data_boolean()
	);
}

fn object_greater_equal<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	let reference = engine.call_method_self(arguments[0], "<", arguments);
	return engine.new_boolean(!reference.object_ref().data_boolean());
}

fn object_lesser_equal<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	let left  = engine.call_method_self(arguments[0], "<", arguments.clone());
	let right = engine.call_method_self(arguments[0], "==", arguments.clone());
	return engine.new_boolean(
		*left.object_ref().data_boolean() ||
		*right.object_ref().data_boolean()
	);
}

fn object_chain<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	let one = arguments[1].object_ref();
	let name = one.data_string();
	let this = arguments[0].value_ref();
	if let Some(method) = this.object_ref().get_method(engine, name) {
		engine.this = Some(arguments[0]);
		return method;
	}

	panic!();
}

fn string_to_string<'a>(_: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	return arguments[0];
}

fn string_comparison<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	return engine.new_boolean(
		arguments[0].object_ref().data_string() ==
		arguments[1].object_ref().data_string()
	);
}

fn string_concatenation<'a>(engine: &mut Engine<'a>, arguments: Vec<Reference<'a>>) -> Reference<'a> {
	let left  = arguments[0];
	let right = engine.call_method(arguments[1], "to_string", Vec::new());
	return engine.new_string(format!("{}{}",
		left.object_ref().data_string(),
		right.object_ref().data_string()
	));
}