use crate::node::Node;

pub struct Error<'a> {
	pub message: String,
	pub node: Option<&'a Node<'a>>,
}

impl Error<'_> {
	pub fn new_runtime(error: &str) -> Self {
		let mut message = String::from("RUNTIME ERROR: ");
		message.push_str(error);
		return Self {
			message,
			node: None,
		};
	}
}
