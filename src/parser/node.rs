struct Node {
	pub element: Option<&'static Element>,
	pub execute: &'static dyn Fn(char) -> Option<&'static Node>,
}
