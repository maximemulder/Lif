use crate::runtime::ReturnReference;
use crate::runtime::data::{ Class, Data };
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTraceable;
use crate::runtime::reference::GcReference;
use crate::runtime::value::GcValue;

pub struct Environment<'a, 'b> {
	pub any:      GcValue<'a, 'b>,
	pub array:    GcValue<'a, 'b>,
	pub boolean:  GcValue<'a, 'b>,
	pub class:    GcValue<'a, 'b>,
	pub function: GcValue<'a, 'b>,
	pub generic:  GcValue<'a, 'b>,
	pub object:   GcValue<'a, 'b>,
	pub integer:  GcValue<'a, 'b>,
	pub string:   GcValue<'a, 'b>,
}

impl<'a, 'b> Environment<'a, 'b> {
	pub fn new() -> Self {
		return Self {
			any:      GcValue::null(),
			array:    GcValue::null(),
			boolean:  GcValue::null(),
			class:    GcValue::null(),
			function: GcValue::null(),
			generic:  GcValue::null(),
			object:   GcValue::null(),
			integer:  GcValue::null(),
			string:   GcValue::null(),
		};
	}
}

impl GcTraceable for Environment<'_, '_> {
	fn trace(&mut self) {
		for class in [self.array, self.boolean, self.class, self.function, self.object, self.integer, self.generic, self.any, self.string].iter_mut() {
			class.trace();
		}
	}
}

impl<'a, 'b> Engine<'a, 'b> {
	fn create_class(&mut self, name: &str) -> GcValue<'a, 'b> {
		return self.new_value(self.environment.class, Data::Class(Class::new(Some(name), Some(self.environment.any))));
	}

	pub fn add_constant_value(&mut self, name: &str, value: GcValue<'a, 'b>) {
		let reference = self.new_constant(value);
		self.add_variable(name, reference);
	}

	fn add_constant_primitive<const N: usize>(&mut self, name: &str, parameters: [GcValue<'a, 'b>; N], callback: &'b dyn Fn(&mut Engine<'a, 'b>, Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b>) {
		let primitive = self.new_primitive(Box::new(parameters), callback);
		self.add_variable(name, primitive);
	}

	fn add_method_primitive<const N: usize>(&mut self, mut value: GcValue<'a, 'b>, name: &str, parameters: [GcValue<'a, 'b>; N], callback: &'b dyn Fn(&mut Engine<'a, 'b>, Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b>) {
		let primitive = self.new_primitive(Box::new(parameters), callback).get_value();
		value.data_class_mut().methods.insert(name.to_string(), primitive);
	}

	pub fn populate(&mut self) {
		self.environment.class = self.create_class("Class");
		self.environment.any   = self.create_class("Any");

		self.environment.array    = self.create_class("Array");
		self.environment.boolean  = self.create_class("Boolean");
		self.environment.function = self.create_class("Function");
		self.environment.generic  = self.create_class("Generic");
		self.environment.object   = self.create_class("Object");
		self.environment.integer  = self.create_class("Integer");
		self.environment.string   = self.create_class("String");

		self.environment.class.class = self.environment.class;
		self.environment.class.data_class_mut().parent = Some(self.environment.any);
		self.environment.any.data_class_mut().parent = None;

		let any      = self.environment.any;
		let array    = self.environment.array;
		let boolean  = self.environment.boolean;
		let class    = self.environment.class;
		let function = self.environment.function;
		let generic  = self.environment.generic;
		let object   = self.environment.object;
		let integer  = self.environment.integer;
		let string   = self.environment.string;

		self.add_constant_primitive("assert", [any],     &primitive_assert);
		self.add_constant_primitive("error",  [any],     &primitive_error);
		self.add_constant_primitive("exit",   [integer], &primitive_exit);
		self.add_constant_primitive("new",    [class],   &primitive_new);
		self.add_constant_primitive("print",  [any],     &primitive_print);

		self.add_constant_value("Any",      any);
		self.add_constant_value("Array",    array);
		self.add_constant_value("Boolean",  boolean);
		self.add_constant_value("Class",    class);
		self.add_constant_value("Function", function);
		self.add_constant_value("Object",   object);
		self.add_constant_value("Integer",  integer);
		self.add_constant_value("String",   string);

		self.add_method_primitive(array, "to_string", [array],               &array_to_string);
		self.add_method_primitive(array, "copy",      [array],               &array_copy);
		self.add_method_primitive(array, "append",    [array, any],          &array_append);
		self.add_method_primitive(array, "prepend",   [array, any],          &array_prepend);
		self.add_method_primitive(array, "insert",    [array, integer, any], &array_insert);
		self.add_method_primitive(array, "remove",    [array, integer],      &array_remove);
		self.add_method_primitive(array, "[]",        [array, array],        &array_access);

		self.add_method_primitive(boolean, "to_string", [boolean],      &boolean_to_string);
		self.add_method_primitive(boolean, "==",        [boolean, any], &boolean_comparison);

		self.add_method_primitive(class, "to_string", [class],         &class_to_string);
		self.add_method_primitive(class, ".",         [class, string], &class_chain);

		self.add_method_primitive(function, "to_string", [function],        &function_to_string);
		self.add_method_primitive(function, "()",        [function, array], &function_call);

		self.add_method_primitive(generic, "to_string", [generic],        &generic_to_string);
		self.add_method_primitive(generic, "<>",        [generic, array], &generic_apply);

		self.add_method_primitive(object, "to_string", [object],         &object_to_string);
		self.add_method_primitive(object, ".",         [object, string], &object_chain);

		self.add_method_primitive(integer, "to_string", [integer],          &integer_to_string);
		self.add_method_primitive(integer, "==",        [integer, any],     &integer_comparison);
		self.add_method_primitive(integer, "<",         [integer, integer], &integer_lesser);
		self.add_method_primitive(integer, "+",         [integer, integer], &integer_addition);
		self.add_method_primitive(integer, "-",         [integer, integer], &integer_subtraction);
		self.add_method_primitive(integer, "*",         [integer, integer], &integer_multiplication);
		self.add_method_primitive(integer, "/",         [integer, integer], &integer_division);
		self.add_method_primitive(integer, "%",         [integer, integer], &integer_remainder);

		self.add_method_primitive(any, "==", [any, any], &any_comparison);
		self.add_method_primitive(any, "!=", [any, any], &any_difference);
		self.add_method_primitive(any, ">",  [any, any], &any_greater);
		self.add_method_primitive(any, "<=", [any, any], &any_lesser_equal);
		self.add_method_primitive(any, ">=", [any, any], &any_greater_equal);

		self.add_method_primitive(string, "to_string", [string],      &string_to_string);
		self.add_method_primitive(string, "==",        [string, any], &string_comparison);
		self.add_method_primitive(string, "+",         [string, any], &string_concatenation);
	}
}

fn primitive_assert<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	if !arguments[0].read()?.data_boolean() {
		panic!();
	}

	return Ok(engine.undefined());
}

fn primitive_error<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let reference = arguments[0].read()?.call_method(engine, "to_string", Vec::new())?;
	println!("{}", reference.read()?.data_string());
	panic!();
}

fn primitive_exit<'a, 'b>(_: &mut Engine<'a, 'b>, _: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	panic!();
}

fn primitive_new<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_object(arguments[0].read()?));
}

fn primitive_print<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let reference = arguments[0].read()?.call_method(engine, "to_string", Vec::new())?;
	println!("{}", reference.read()?.data_string());
	return Ok(engine.undefined());
}

fn any_comparison<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_boolean(arguments[0] == arguments[1]));
}

fn any_difference<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let reference = arguments[0].read()?.call_method_self(engine, "==", arguments)?;
	return Ok(engine.new_boolean(!reference.read()?.data_boolean()));
}

fn any_greater<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let left  = arguments[0].read()?.call_method_self(engine, "<", arguments.clone())?;
	let right = arguments[0].read()?.call_method_self(engine, "==", arguments.clone())?;
	return Ok(engine.new_boolean(!left.read()?.data_boolean() && !right.read()?.data_boolean()));
}

fn any_greater_equal<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let reference = arguments[0].read()?.call_method_self(engine, "<", arguments)?;
	return Ok(engine.new_boolean(!reference.read()?.data_boolean()));
}

fn any_lesser_equal<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let left  = arguments[0].read()?.call_method_self(engine, "<", arguments.clone())?;
	let right = arguments[0].read()?.call_method_self(engine, "==", arguments.clone())?;
	return Ok(engine.new_boolean(*left.read()?.data_boolean() || *right.read()?.data_boolean()));
}

fn array_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let mut string = String::from("[");
	let elements = arguments[0].read()?.data_array().clone();
	for element in elements.iter() {
		let reference = element.read()?.call_method(engine, "to_string", Vec::new())?;
		string.push_str(reference.read()?.data_string());
		string.push_str(", ");
	}

	if !elements.is_empty() {
		string.truncate(string.len() - 2);
	}

	string.push_str("]");
	return Ok(engine.new_string(string));
}

fn array_copy<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_array(arguments[0].read()?.data_array().clone()));
}

fn array_append<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let reference = engine.new_reference(arguments[1].read()?);
	arguments[0].read()?.data_array_mut().push(reference);
	return Ok(engine.undefined());
}

fn array_prepend<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let reference = engine.new_reference(arguments[1].read()?);
	arguments[0].read()?.data_array_mut().insert(0, reference);
	return Ok(engine.undefined());
}

fn array_insert<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let reference = engine.new_reference(arguments[1].read()?);
	let index = *arguments[1].read()?.data_integer();
	arguments[0].read()?.data_array_mut().insert(index, reference);
	return Ok(engine.undefined());
}

fn array_remove<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let index = *arguments[1].read()?.data_integer();
	arguments[0].read()?.data_array_mut().remove(index);
	return Ok(engine.undefined());
}

fn array_access<'a, 'b>(_: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(arguments[0].read()?.data_array()[*arguments[1].read()?.data_integer()]);
}

fn boolean_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_string(arguments[0].read()?.data_boolean().to_string()));
}

fn boolean_comparison<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_boolean(arguments[0].read()?.data_boolean() == arguments[1].read()?.data_boolean()));
}

fn class_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let mut string = String::new();
	string += "Class";
	if let Some(name) = &arguments[0].read()?.data_class().name {
		string += "(";
		string += name;
		string += ")";
	}

	return Ok(engine.new_string(string));
}

fn class_chain<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let name = arguments[1].read()?.data_string().clone();
	let member = engine.undefined();
	let mut value = arguments[0].read()?;
	let class = value.data_class_mut();
	return Ok(if let Some(&member) = class.statics.get(&name) {
		member
	} else {
		class.statics.insert(name.clone(), member);
		member
	});
}

fn class_access<'a, 'b>(engine: &mut Engine<'a, 'b>, _: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_constant(engine.environment.array));
}

fn function_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, _: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_string("FUNCTION".to_string()));
}

fn function_call<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return arguments[0].read()?.data_callable().duplicate().execute(engine, arguments[1].read()?.data_array().clone());
}

fn generic_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, _: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_string("GENERIC".to_string()));
}

fn generic_apply<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	engine.push_scope();
	let value = arguments[0].read()?;
	let generic = value.data_generic();
	for (parameter, argument) in generic.generics.iter().zip(arguments[1].read()?.data_array()) {
		let reference = engine.new_reference(argument.read()?);
		engine.add_variable(parameter, reference);
	}

	let reference = generic.node.execute(engine)?;
	engine.pop_scope();
	return Ok(reference);
}

fn object_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let mut string = String::from("{");
	let attributes = &arguments[0].read()?.data_object().attributes.clone();
	for (name, attribute) in attributes {
		string.push_str(&name);
		string.push_str(": ");
		let reference = attribute.read()?.call_method(engine, "to_string", Vec::new())?;
		string.push_str(reference.read()?.data_string());
		string.push_str(", ");
	}

	if !attributes.is_empty() {
		string.truncate(string.len() - 2);
	}

	string.push_str("}");
	return Ok(engine.new_string(string));
}

fn object_chain<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let name = arguments[1].read()?.data_string().clone();
	let member = engine.undefined();
	let mut value = arguments[0].read()?;
	let object = value.data_object_mut();
	return Ok(if let Some(&member) = object.attributes.get(&name) {
		member
	} else {
		object.attributes.insert(name.clone(), member);
		member
	});
}

fn integer_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_string(arguments[0].read()?.data_integer().to_string()));
}

fn integer_comparison<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_boolean(*arguments[0].read()?.data_integer() == *arguments[1].read()?.data_integer()));
}

fn integer_lesser<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_boolean(*arguments[0].read()?.data_integer() < *arguments[1].read()?.data_integer()));
}

fn integer_addition<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_integer(*arguments[0].read()?.data_integer() + *arguments[1].read()?.data_integer()));
}

fn integer_subtraction<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_integer(*arguments[0].read()?.data_integer() - *arguments[1].read()?.data_integer()));
}

fn integer_multiplication<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_integer(*arguments[0].read()?.data_integer() * *arguments[1].read()?.data_integer()));
}

fn integer_division<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_integer(*arguments[0].read()?.data_integer() / *arguments[1].read()?.data_integer()));
}

fn integer_remainder<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_integer(*arguments[0].read()?.data_integer() % *arguments[1].read()?.data_integer()));
}

fn string_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_constant(arguments[0].read()?));
}

fn string_comparison<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	return Ok(engine.new_boolean(arguments[0].read()?.data_string() == arguments[1].read()?.data_string()));
}

fn string_concatenation<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b> {
	let right = arguments[1].read()?.call_method(engine, "to_string", Vec::new())?;
	return Ok(engine.new_string(format!("{}{}", arguments[0].read()?.data_string(), right.read()?.data_string())));
}
