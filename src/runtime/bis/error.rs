use crate::ast::Pos;

pub struct Error {
    pos: Pos,
    message: Box<str>,
}

impl Error {
    pub fn new(pos: Pos, message: &str) -> Self {
        Self { pos, message: Box::from(message) }
    }

    pub fn get_message(&self) -> Box<str> {
        format!("RUNTIME ERROR: {}", self.message).into_boxed_str()
    }
}
