use crate::memory::Ref;
use crate::node::Node;

use std::cmp::min;

pub struct Error {
    pub message: Box<str>,
    pub node: Option<Ref<Node>>,
}

impl Error {
    fn new(message: String, node: Option<Ref<Node>>) -> Self {
        Self {
            message: message.into_boxed_str(),
            node,
        }
    }

    pub fn new_runtime(error: &str) -> Self {
        let mut message = String::new();
        message += "RUNTIME ERROR: ";
        message += error;
        Self::new(message, None)
    }

    pub fn get_message(&self) -> String {
        let mut message = String::new();
        message += &self.message;
        if let Some(node) = self.node {
            let code = node.code;
            if let Some(name) = code.name.as_ref() {
                message += " ";
                message += name;
            }

            message += "\n\n";
            message += code.node_line(&node);
            message += "\n";
            message += &" ".repeat(code.node_shift_left(&node));
            message += &"^".repeat(min(code.node_str(&node).len(), code.node_shift_right(&node)));
        }

        message
    }
}
