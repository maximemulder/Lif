pub struct Error {
	pub message: String,
}

impl Error {
	pub fn new_runtime(error: &str) -> Self {
		let mut message = String::from("RUNTIME ERROR: ");
		message.push_str(error);
		return Self {
			message,
		};
	}
}
