pub struct Error {
	pub message: String,
	pub delimiters: Option<(usize, usize)>,
}

impl Error {
	pub fn new_runtime(error: &str) -> Self {
		let mut message = String::from("RUNTIME ERROR: ");
		message.push_str(error);
		return Self {
			message,
			delimiters: None,
		};
	}
}
