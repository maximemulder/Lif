use crate::runtime::value::{ Value, ValueObject };

#[derive(Clone,Copy)]
pub struct ReferenceObject<'a> {
	value: Value<'a>,
}

impl<'a> ReferenceObject<'a> {
	pub fn new(value: Value<'a>) -> Self {
		return Self {
			value,
		}
	}

	pub fn value_ref(&self) -> &Value<'a> {
		return &self.value;
	}

	pub fn value_mut(&mut self) -> &mut Value<'a> {
		return &mut self.value;
	}

	pub fn object_ref(&self) -> &ValueObject<'a> {
		return self.value_ref().object_ref();
	}

	pub fn object_mut(&mut self) -> &mut ValueObject<'a> {
		return self.value_mut().object_mut();
	}
}
