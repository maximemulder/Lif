use crate::ast::Pos;

use crate::runtime::bis::flow::Res;

pub struct Error {
    pos: Pos,
    message: Box<str>,
}

impl Error {
    pub fn new<T>(pos: Pos, message: String) -> Res<T> {
        Err(Self { pos, message: Box::from(message) })
    }

    pub fn get_message(&self) -> Box<str> {
        format!("RUNTIME ERROR: {}\n--> `{}` {}\n{}",
            self.message,
            self.pos.print_name(),
            self.pos.print_pos(),
            self.pos.print_node(),
        ).into_boxed_str()
    }
}
