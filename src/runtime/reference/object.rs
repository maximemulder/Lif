use crate::runtime::proxy::Visitable;
use crate::runtime::value::Value;

pub struct ReferenceObject<'a> {
	value: Value<'a>,
}

impl<'a> ReferenceObject<'a> {
	pub fn new(value: Value<'a>) -> Self {
		return Self {
			value,
		}
	}

	pub fn value(&self) -> Value<'a> {
		return self.value;
	}

	pub fn value_ref(&self) -> &Value<'a> {
		return &self.value;
	}

	pub fn value_mut(&mut self) -> &mut Value<'a> {
		return &mut self.value;
	}
}

impl Visitable for ReferenceObject<'_> {
	fn visit(&mut self) {
		self.value.visit();
	}
}
