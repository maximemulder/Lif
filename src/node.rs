use crate::element::Element;

pub struct Node {
	pub element: Option<&'static Element>,
	pub execute: &'static dyn Fn(char) -> Option<&'static Node>,
}

impl Node {
	pub const fn new(element: &'static Element, execute: &'static dyn Fn(char) -> Option<&'static Node>) -> Self {
		return Self {
			element: Some(element),
			execute,
		}
	}

	pub const fn new_null(execute: &'static dyn Fn(char) -> Option<&'static Node>) -> Self {
		return Self {
			element: None,
			execute,
		};
	}

	pub const fn new_final(element: &'static Element) -> Self {
		return Self {
			element: Some(element),
			execute: &|_| None,
		};
	}
}
