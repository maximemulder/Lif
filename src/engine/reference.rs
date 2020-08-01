use crate::engine::value::Value;

struct Reference {
	value: Option<Value>,
}

impl Reference {
	fn new() -> Self {
		return Self {
			value: None,
		}
	}

	fn read(&self) -> Value {
		return Value {};
	}

	fn write(&mut self, value: Value) {
		self.value = Some(value);
	}
}
