use crate::runtime::{ Object, Value };

#[derive(Clone,Copy)]
pub struct Reference<'a> {
	value: Value<'a>,
}

impl<'a> Reference<'a> {
	pub fn new(value: Value<'a>) -> Self {
		return Self {
			value,
		}
	}

	pub fn new_undefined() -> Self {
		return Self {
			value: Value::new_undefined(),
		};
	}

	pub fn value_ref(&self) -> &Value<'a> {
		return &self.value;
	}

	pub fn value_mut(&mut self) -> &mut Value<'a> {
		return &mut self.value;
	}

	pub fn object_ref(&self) -> &Object<'a> {
		return self.value_ref().object_ref();
	}

	pub fn object_mut(&mut self) -> &mut Object<'a> {
		return self.value_mut().object_mut();
	}
}
