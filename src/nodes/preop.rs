use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Preop<'a> {
    expression: Node<'a>,
    operator:   &'a str,
}

impl<'a> Preop<'a> {
    pub fn new(operator: &'a str, expression: Node<'a>) -> Self {
        Self {
            expression,
            operator: match operator {
                "~" => "__bnot__",
                "+" => "__pos__",
                "-" => "__neg__",
                "!" => "__not__",
                _   => panic!(),
            },
        }
    }
}

impl<'a> Executable<'a> for Preop<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        let expression = execute!(engine, &self.expression).read()?;
        expression.get_method(&self.operator).unwrap().call(engine, vec![expression])
    }
}
