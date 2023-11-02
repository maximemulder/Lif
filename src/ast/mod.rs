pub mod build;
pub mod nodes;

use crate::memory::Ref;
use crate::parser::Code;

use std::cmp::min;

#[derive(Clone, Copy)]
pub struct Pos {
    pub source: Option<Ref<Code>>,
    start: usize,
    length: usize,
}

impl Pos {
    pub const DUMMY: Pos = Pos {
        source: None,
        start: 0,
        length: 0,
    };

    pub fn print_name(&self) -> String {
        if let Some(source) = self.source {
            source.name.as_ref()
                .map(|name| name.to_string())
                .unwrap_or(String::from("{anonymous}"))
        } else {
            String::from("{anonymous}")
        }
    }

    pub fn print_pos(&self) -> String {
        format!("{}:{}", self.start_y() + 1, self.start_x() + 1)
    }

    pub fn print_node(&self) -> String {
        let mut string = String::new();
        if let Some(_) = self.source {
            string += "|\n";
            string += "| ";
            string += self.code_line();
            string += "| ";
            string += &" ".repeat(self.start - self.start_line_left());
            string += &"^".repeat(min(self.length, self.start_line_right() - self.start));
            string += "\n";
        }

        string
    }

    fn start_x(&self) -> usize {
        let Some(source) = self.source else {
            return 0;
        };

        let mut line = 0;
        for (i, char) in source.text.chars().enumerate() {
            if self.start == i {
                break;
            }

            if char == '\n' {
                line = 0;
            } else {
                line += 1;
            }
        }

        line
    }

    fn start_y(&self) -> usize {
        let Some(source) = self.source else {
            return 0;
        };

        let mut col = 0;
        for (i, char) in source.text.chars().enumerate() {
            if self.start == i {
                break;
            }

            if char == '\n' {
                col += 1;
            }
        }

        col
    }

    fn start_line_left(&self) -> usize {
        let Some(source) = self.source else {
            return 0;
        };

        let mut count = 0;
        for (i, char) in source.text.chars().enumerate() {
            if i == self.start {
                break;
            }

            if char == '\n' {
                count = i + 1;
            }
        }

        count
    }

    fn start_line_right(&self) -> usize {
        let Some(source) = self.source else {
            return 0;
        };

        let mut count = 0;
        for (i, char) in source.text.chars().enumerate() {
            count += 1;
            if self.start < i && char == '\n' {
                break;
            }
        }

        count
    }

    fn code_line(&self) -> &str {
        &self.source.as_ref().unwrap().text[self.start_line_left() .. self.start_line_right()]
    }
}
