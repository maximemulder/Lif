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

	pub fn node_str(&self, node: &Node) -> &str {
		return &self.text[node.left() .. node.right()];
	}

	pub fn node_line(&self, node: &Node) -> &str {
		let index = node.left();
		return &self.text[index - self.line_pos_left(index) .. index + self.line_pos_right(index)];
	}

	pub fn node_x(&self, node: &Node) -> usize {
		return self.line_pos_left(node.left()) + 1;
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

	pub fn node_shift_left(&self, node: &Node) -> usize {
		return self.line_shift(self.index_iterator_reverse(node.left()));
	}

	pub fn node_shift_right(&self, node: &Node) -> usize {
		return self.line_shift(self.index_iterator(node.left()));
	}

	fn line_pos_left(&self, index: usize) -> usize {
		return self.line_pos(self.index_iterator_reverse(index));
	}

	fn line_pos_right(&self, index: usize) -> usize {
		return self.line_pos(self.index_iterator(index));
	}

	fn line_pos(&self, chars: impl Iterator<Item = char>) -> usize {
		let mut counter = 0;
		for r#char in chars {
			match r#char {
				'\r' | '\n' => break,
				_ => counter += 1,
			}
		}

		return counter;
	}

	fn line_shift(&self, chars: impl Iterator<Item = char>) -> usize {
		let mut counter = 0;
		for r#char in chars {
			match r#char {
				'\r' | '\n' => break,
				'\t' => counter += 8,
				_ => counter += 1,
			}
		}

		return counter;
	}

	fn index_iterator<'a>(&'a self, index: usize) -> impl Iterator<Item = char> + 'a {
		return self.text[index ..].chars();
	}

	fn index_iterator_reverse<'a>(&'a self, index: usize) -> impl Iterator<Item = char> + 'a {
		return self.text[.. index].chars().rev();
	}
}
