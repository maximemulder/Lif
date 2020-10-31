use crate::node::Node;
use std::fs::read_to_string;

pub struct Code {
    pub name: Box<str>,
    pub text: Box<str>,
}

impl Code {
    pub fn new(name: &str) -> Option<Self> {
        let text = read_to_string(name).ok()?;
        Some(Self {
            name: Box::from(name),
            text: Box::from(text),
        })
    }

    pub fn node_str(&self, node: &Node) -> &str {
        &self.text[node.left() .. node.right()]
    }

    pub fn node_line(&self, node: &Node) -> &str {
        let index = node.left();
        &self.text[index - self.line_pos_left(index) .. index + self.line_pos_right(index)]
    }

    pub fn node_x(&self, node: &Node) -> usize {
        self.line_pos_left(node.left()) + 1
    }

    pub fn node_y(&self, node: &Node) -> usize {
        let index = node.left();
        let mut x = 1;
        for (counter, r#char) in self.text.chars().enumerate() {
            if counter == index {
                break;
            }

            if r#char == '\n' {
                x += 1;
            }
        }

        x
    }

    pub fn node_shift_left(&self, node: &Node) -> usize {
        self.line_shift(self.index_iterator_reverse(node.left()))
    }

    pub fn node_shift_right(&self, node: &Node) -> usize {
        self.line_shift(self.index_iterator(node.left()))
    }

    fn line_pos_left(&self, index: usize) -> usize {
        self.line_pos(self.index_iterator_reverse(index))
    }

    fn line_pos_right(&self, index: usize) -> usize {
        self.line_pos(self.index_iterator(index))
    }

    fn line_pos(&self, chars: impl Iterator<Item = char>) -> usize {
        let mut counter = 0;
        for r#char in chars {
            match r#char {
                '\r' | '\n' => break,
                _ => counter += 1,
            }
        }

        counter
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

        counter
    }

    fn index_iterator(&self, index: usize) -> impl Iterator<Item = char> + '_ {
        self.text[index ..].chars()
    }

    fn index_iterator_reverse(&self, index: usize) -> impl Iterator<Item = char> + '_ {
        self.text[.. index].chars().rev()
    }
}
