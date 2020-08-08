pub struct Text {
	string: String,
}

impl Text {
	pub fn new(string: &str) -> Self {
		return Self {
			string: string.to_string(),
		}
	}
}
