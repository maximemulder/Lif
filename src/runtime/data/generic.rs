use crate::nodes::Node;

pub struct Generic<'a, 'b> {
	pub generics: &'b Vec<&'a str>,
	pub node: &'b dyn Node<'a>,
}

impl<'a, 'b> Generic<'a, 'b> {
	pub fn new(generics: &'b Vec<&'a str>, node: &'b dyn Node<'a>) -> Self {
		return Self {
			generics,
			node,
		};
	}
}
