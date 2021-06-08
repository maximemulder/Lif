use crate::memory::Ref;
use crate::node::Node;
use crate::runtime::value::GcValue;

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

    pub fn new_undefined_method(method: &str, class: GcValue) -> Self {
        let mut message = String::new();
        message += "Method \"";
        message += method;
        message += "\" is undefined in the type ";
        message += &class.data_class().tag().to_string();
        message += ".";
        Self::new_runtime(&message)
    }

    pub fn new_undeclared_variable(variable: &str) -> Self {
        let mut message = String::new();
        message += "Variable \"";
        message += variable;
        message += "\" is not declared.";
        Self::new_runtime(&message)
    }

    pub fn new_jump() -> Self {
        Self::new_runtime("Incorrect jump use.")
    }

    pub fn new_undefined() -> Self {
        Self::new_runtime("Cannot read an undefined reference.")
    }

    pub fn new_constant_write() -> Self {
        Self::new_runtime("Cannot write data into a constant.")
    }

    pub fn new_arguments(parameters: usize, arguments: usize) -> Self {
        let mut message = String::new();
        message += "Provided ";
        message += &arguments.to_string();
        message += " arguments while the function expects ";
        message += &parameters.to_string();
        message += " parameters.";
        Self::new_runtime(&message)
    }

    pub fn new_cast(value: GcValue, r#type: GcValue) -> Self {
        let mut message = String::new();
        message += "Cannot cast a value of the type ";
        message += &value.class.data_class().tag().to_string();
        message += " to the type ";
        message += &r#type.data_class().tag().to_string();
        message += ".";
        Self::new_runtime(&message)
    }

    pub fn new_nullable() -> Self {
        Self::new_runtime("Cannot get the content of a null value.")
    }

    pub fn new_rest() -> Self {
        Self::new_runtime("Rest parameter type must be an array.")
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
