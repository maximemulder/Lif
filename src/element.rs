use std::cmp::PartialEq;

pub struct Element {
	pub name: &'static str,
}

impl Element {
	pub const fn new(name: &'static str) -> Self {
		return Self {
			name,
		};
	}
}

impl PartialEq for Element {
	fn eq(&self, other: &Element) -> bool {
		return self.name == other.name;
	}
}
