use crate::runtime::ReturnReference;
use crate::runtime::data::{ Class, Data };
use crate::runtime::engine::Engine;
use crate::runtime::gc::{ GcRef, GcTraceable };
use crate::runtime::value::GcValue;

pub struct Environment<'a, 'b> {
	pub array:    GcValue<'a, 'b>,
	pub boolean:  GcValue<'a, 'b>,
	pub class:    GcValue<'a, 'b>,
	pub function: GcValue<'a, 'b>,
	pub instance: GcValue<'a, 'b>,
	pub integer:  GcValue<'a, 'b>,
	pub object:   GcValue<'a, 'b>,
	pub string:   GcValue<'a, 'b>,
}

impl<'a, 'b> Environment<'a, 'b> {
	pub fn new() -> Self {
		return Self {
			array:    GcRef::null(),
			boolean:  GcRef::null(),
			class:    GcRef::null(),
			function: GcRef::null(),
			instance: GcRef::null(),
			integer:  GcRef::null(),
			object:   GcRef::null(),
			string:   GcRef::null(),
		};
	}
}

impl GcTraceable for Environment<'_, '_> {
	fn trace(&mut self) {
		for class in [self.array, self.boolean, self.class, self.function, self.instance, self.integer, self.object, self.string].iter_mut() {
			class.trace();
		}
	}
}

impl<'a, 'b> Engine<'a, 'b> {
	fn create_class(&mut self) -> GcValue<'a, 'b> {
		return self.new_value(self.environment.class, Data::Class(Class::new(Some(self.environment.object))));
	}

	pub fn add_constant_value(&mut self, name: &str, value: GcValue<'a, 'b>) {
		let reference = self.new_constant(value);
		self.add_variable(name, reference);
	}

	fn add_constant_primitive(&mut self, name: &str, callback: &'b dyn Fn(&mut Engine<'a, 'b>, Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b>) {
		let primitive = self.new_primitive(callback);
		self.add_variable(name, primitive);
	}

	fn add_method_primitive(&mut self, mut value: GcValue<'a, 'b>, name: &str, callback: &'b dyn Fn(&mut Engine<'a, 'b>, Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b>) {
		let primitive = self.new_primitive(callback).get_value();
		value.data_class_mut().methods.insert(name.to_string(), primitive);
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

		self.environment.class.class = self.environment.class;
		self.environment.class.data_class_mut().parent = Some(self.environment.object);
		self.environment.object.data_class_mut().parent = None;

		self.add_constant_primitive("assert", &primitive_assert);
		self.add_constant_primitive("error",  &primitive_error);
		self.add_constant_primitive("exit",   &primitive_exit);
		self.add_constant_primitive("new",    &primitive_new);
		self.add_constant_primitive("print",  &primitive_print);

		let array    = self.environment.array;
		let boolean  = self.environment.boolean;
		let class    = self.environment.class;
		let function = self.environment.function;
		let instance = self.environment.instance;
		let integer  = self.environment.integer;
		let object   = self.environment.object;
		let string   = self.environment.string;

		self.add_constant_value("Array",    array);
		self.add_constant_value("Boolean",  boolean);
		self.add_constant_value("Class",    class);
		self.add_constant_value("Function", function);
		self.add_constant_value("Instance", instance);
		self.add_constant_value("Integer",  integer);
		self.add_constant_value("Object",   object);
		self.add_constant_value("String",   string);

		self.add_method_primitive(array, "to_string", &array_to_string);
		self.add_method_primitive(array, "copy",      &array_copy);
		self.add_method_primitive(array, "append",    &array_append);
		self.add_method_primitive(array, "prepend",   &array_prepend);
		self.add_method_primitive(array, "insert",    &array_insert);
		self.add_method_primitive(array, "remove",    &array_remove);
		self.add_method_primitive(array, "[]",        &array_access);

		self.add_method_primitive(boolean, "to_string", &boolean_to_string);
		self.add_method_primitive(boolean, "==",        &boolean_comparison);

		self.add_method_primitive(class, "to_string", &class_to_string);
		self.add_method_primitive(class, ".",         &class_chain);

		self.add_method_primitive(function, "to_string", &function_to_string);
		self.add_method_primitive(function, "()",        &function_call);

		self.add_method_primitive(instance, "to_string", &instance_to_string);
		self.add_method_primitive(instance, ".",         &instance_chain);

		self.add_method_primitive(integer, "to_string", &integer_to_string);
		self.add_method_primitive(integer, "==",        &integer_comparison);
		self.add_method_primitive(integer, "<",         &integer_lesser);
		self.add_method_primitive(integer, "+",         &integer_addition);
		self.add_method_primitive(integer, "-",         &integer_subtraction);
		self.add_method_primitive(integer, "*",         &integer_multiplication);
		self.add_method_primitive(integer, "/",         &integer_division);
		self.add_method_primitive(integer, "%",         &integer_remainder);

		self.add_method_primitive(object, "==", &object_comparison);
		self.add_method_primitive(object, "!=", &object_difference);
		self.add_method_primitive(object, ">",  &object_greater);
		self.add_method_primitive(object, "<=", &object_lesser_equal);
		self.add_method_primitive(object, ">=", &object_greater_equal);

		self.add_method_primitive(string, "to_string", &string_to_string);
		self.add_method_primitive(string, "==",        &string_comparison);
		self.add_method_primitive(string, "+",         &string_concatenation);
	}
}

fn primitive_assert<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	if !arguments[0].data_boolean() {
		panic!();
	}

	return Ok(engine.undefined());
}

fn primitive_error<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let reference = engine.call_method(arguments[0], "to_string", Vec::new())?;
	println!("{}", reference.read()?.data_string());
	panic!();
}

fn primitive_exit<'a, 'b>(_: &mut Engine<'a, 'b>, _: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	panic!();
}

fn primitive_new<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_instance(arguments[0]));
}

fn primitive_print<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let reference = engine.call_method(arguments[0], "to_string", Vec::new())?;
	println!("{}", reference.read()?.data_string());
	return Ok(engine.undefined());
}

fn array_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let mut string = String::from("[");
	let elements = arguments[0].data_array().clone();
	for element in elements.iter() {
		let reference = engine.call_method(element.read()?, "to_string", Vec::new())?;
		string.push_str(reference.read()?.data_string());
		string.push_str(", ");
	}

	if !elements.is_empty() {
		string.truncate(string.len() - 2);
	}

	string.push_str("]");
	return Ok(engine.new_string(string));
}

fn array_copy<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_array(arguments[0].data_array().clone()));
}

fn array_append<'a, 'b>(engine: &mut Engine<'a, 'b>, mut arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let reference = engine.new_reference(arguments[1]);
	arguments[0].data_array_mut().push(reference);
	return Ok(engine.undefined());
}

fn array_prepend<'a, 'b>(engine: &mut Engine<'a, 'b>, mut arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let reference = engine.new_reference(arguments[1]);
	arguments[0].data_array_mut().insert(0, reference);
	return Ok(engine.undefined());
}

fn array_insert<'a, 'b>(engine: &mut Engine<'a, 'b>, mut arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let reference = engine.new_reference(arguments[1]);
	let index = *arguments[1].data_integer();
	arguments[0].data_array_mut().insert(index, reference);
	return Ok(engine.undefined());
}

fn array_remove<'a, 'b>(engine: &mut Engine<'a, 'b>, mut arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let index = *arguments[1].data_integer();
	arguments[0].data_array_mut().remove(index);
	return Ok(engine.undefined());
}

fn array_access<'a, 'b>(_: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(arguments[0].data_array()[*arguments[1].data_integer()]);
}

fn boolean_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_string(arguments[0].data_boolean().to_string()));
}

fn boolean_comparison<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_boolean(arguments[0].data_boolean() == arguments[1].data_boolean()));
}

fn class_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, _: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_string("CLASS".to_string()));
}

fn class_chain<'a, 'b>(engine: &mut Engine<'a, 'b>, mut arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let name = arguments[1].data_string().clone();
	let member = engine.undefined();
	let class = arguments[0].data_class_mut();
	return Ok(if let Some(&member) = class.statics.get(&name) {
		member
	} else {
		class.statics.insert(name.clone(), member);
		member
	});
}

fn class_access<'a, 'b>(engine: &mut Engine<'a, 'b>, _: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_constant(engine.environment.array));
}

fn function_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, _: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_string("FUNCTION".to_string()));
}

fn function_call<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return arguments[0].data_callable().duplicate().execute(engine, arguments[1 ..].to_vec());
}

fn instance_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let mut string = String::from("{");
	let attributes = &arguments[0].data_instance().attributes.clone();
	for (name, attribute) in attributes {
		string.push_str(&name);
		string.push_str(": ");
		let reference = engine.call_method(attribute.read()?, "to_string", Vec::new())?;
		string.push_str(reference.read()?.data_string());
		string.push_str(", ");
	}

	if !attributes.is_empty() {
		string.truncate(string.len() - 2);
	}

	string.push_str("}");
	return Ok(engine.new_string(string));
}

fn instance_chain<'a, 'b>(engine: &mut Engine<'a, 'b>, mut arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let name = arguments[1].data_string().clone();
	let member = engine.undefined();
	let instance = arguments[0].data_instance_mut();
	return Ok(if let Some(&member) = instance.attributes.get(&name) {
		member
	} else {
		instance.attributes.insert(name.clone(), member);
		member
	});
}

fn integer_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_string(arguments[0].data_integer().to_string()));
}

fn integer_comparison<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_boolean(*arguments[0].data_integer() == *arguments[1].data_integer()));
}

fn integer_lesser<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_boolean(*arguments[0].data_integer() < *arguments[1].data_integer()));
}

fn integer_addition<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_integer(*arguments[0].data_integer() + *arguments[1].data_integer()));
}

fn integer_subtraction<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_integer(*arguments[0].data_integer() - *arguments[1].data_integer()));
}

fn integer_multiplication<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_integer(*arguments[0].data_integer() * *arguments[1].data_integer()));
}

fn integer_division<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_integer(*arguments[0].data_integer() / *arguments[1].data_integer()));
}

fn integer_remainder<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_integer(*arguments[0].data_integer() % *arguments[1].data_integer()));
}

fn object_comparison<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_boolean(arguments[0] == arguments[1]));
}

fn object_difference<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let reference = engine.call_method_self(arguments[0], "==", arguments)?;
	return Ok(engine.new_boolean(!reference.read()?.data_boolean()));
}

fn object_greater<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let left  = engine.call_method_self(arguments[0], "<", arguments.clone())?;
	let right = engine.call_method_self(arguments[0], "==", arguments.clone())?;
	return Ok(engine.new_boolean(!left.read()?.data_boolean() && !right.read()?.data_boolean()));
}

fn object_greater_equal<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let reference = engine.call_method_self(arguments[0], "<", arguments)?;
	return Ok(engine.new_boolean(!reference.read()?.data_boolean()));
}

fn object_lesser_equal<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let left  = engine.call_method_self(arguments[0], "<", arguments.clone())?;
	let right = engine.call_method_self(arguments[0], "==", arguments.clone())?;
	return Ok(engine.new_boolean(*left.read()?.data_boolean() || *right.read()?.data_boolean()));
}

fn string_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_constant(arguments[0]));
}

fn string_comparison<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_boolean(arguments[0].data_string() == arguments[1].data_string()));
}

fn string_concatenation<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let right = engine.call_method(arguments[1], "to_string", Vec::new())?;
	return Ok(engine.new_string(format!("{}{}", arguments[0].data_string(), right.read()?.data_string())));
}
