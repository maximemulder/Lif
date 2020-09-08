use crate::node::Node;

pub struct Code {
	text: Box<str>,
}

impl Code {
	pub fn new(text: &str) -> Self {
		return Self {
			text: Box::from(text),
		};
	}

	pub fn node_x(&self, node: &Node) -> usize {
		return self.line_shift_left(node.left()) + 1;
	}

	pub fn node_y(&self, node: &Node) -> usize {
		let index = node.left();
		let mut counter = 0;
		let mut x = 1;
		for r#char in self.text.chars() {
			if counter == index {
				break;
			}

			counter += 1;
			if r#char == '\n' {
				x += 1;
			}
		}

		return x;
	}

	pub fn node_line(&self, node: &Node) -> &str {
		let index = node.left();
		return &self.text[index - self.line_shift_left(index) .. index + self.line_shift_right(index)];
	}

	pub fn node_str(&self, node: &Node) -> &str {
		return &self.text[node.left() .. node.right()];
	}

	fn line_shift(&self, chars: impl Iterator<Item = char>) -> usize {
		let mut counter = 0;
		for r#char in chars {
			if r#char == '\r' || r#char == '\n' {
				break;
			}

			counter += 1;
		}

		return counter;
	}

	fn line_shift_left(&self, index: usize) -> usize {
		return self.line_shift(self.text[.. index].chars().rev());
	}

	fn line_shift_right(&self, index: usize) -> usize {
		return self.line_shift(self.text[index ..].chars());
	}
}
