use crate::element::Element;

pub struct Token<'a> {
	pub element: &'static Element,
	pub string: &'a str,
}

impl<'a> Token<'a> {
	pub const fn new(element: &'static Element, string: &'a str) -> Self {
		return Self {
			element,
			string,
		};
	}
}
