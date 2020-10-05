use crate::node::Node;
use crate::runtime::value::{ GcValue, Value };

pub struct Error<'a> {
	pub message: Box<str>,
	pub node: Option<&'a Node<'a>>,
}

impl Error<'_> {

	fn new(message: String) -> Self {
		return Self {
			message: Box::from(message),
			node: None
		};
	}

	pub fn new_runtime(error: &str) -> Self {
		let mut message = String::new();
		message += "RUNTIME ERROR: ";
		message += error;
		return Self::new(message);
	}

	pub fn new_undefined_method(method: &str, class: GcValue) -> Self {
		let mut message = String::new();
		message += "RUNTIME ERROR: Method \"";
		message += method;
		message += "\" is undefined";
		if let Some(name) = &class.data_class().name {
			message += " in type \"";
			message += name;
			message += "\"";
		}

		message += ".";
		return Self::new(message);
	}

	pub fn new_undeclared_variable(variable: &str) -> Self {
		let mut message = String::new();
		message += "RUNTIME ERROR: Variable \"";
		message += variable;
		message += "\" is not declared.";
		return Self::new(message);
	}

	pub fn new_control() -> Self {
		return Self::new(String::from("RUNTIME ERROR: Cannot loop control out of a function."));
	}

	pub fn new_undefined() -> Self {
		return Self::new(String::from("RUNTIME ERROR: Cannot read an undefined reference."));
	}

	pub fn new_constant_write() -> Self {
		return Self::new(String::from("RUNTIME ERROR: Cannot write data into a constant."));
	}

	pub fn new_cast(value: &Value, r#type: GcValue) -> Self {
		let mut message = String::new();
		message += "RUNTIME ERROR: Cannot cast ";
		if let Some(name) = &value.class.data_class().name {
			message += "a value of the type \"";
			message += name;
			message += "\"";
		} else {
			message += "value ";
		}

		message += " to the ";
		if let Some(name) = &r#type.data_class().name {
			message += "type \"";
			message += name;
			message += "\"";
		} else {
			message += "required type";
		}

		message += ".";
		return Self::new(message);
	}
}
