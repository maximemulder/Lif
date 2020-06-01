struct Reference {
	value: Option<Value>,
	type: Option<Value>,
	mutable: bool,
}

impl Reference {
	fn undefined() -> Self {
		return Self {
			type: None,
			value: None,
			mutable: false,
		}
	}

	fn constant(type: Option<Value>, value: Option<Value>) -> Self {
		return Self {
			type,
			value,
			mutable: false,
		};
	}

	fn variable(type: Option<Value>, value: Option<Value>) -> Self {
		return Self {
			type,
			value,
			mutable: true,
		};
	}

	fn read(&self) -> Value {
		match self.value {
			Some(value) => return value,
			None => panic!(),
		}
	}

	fn write(&mut self, value: Value) {
		if !self.mutable {
			panic!();
		}

		self.value = Some(value);
	}
}
