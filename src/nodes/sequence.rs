use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Sequence<'a> {
    expression:  Node<'a>,
    expressions: Box<[Node<'a>]>,
    operator:    &'a str,
}

impl<'a> Sequence<'a> {
    pub fn new(expression: Node<'a>, open: &'a str, expressions: Box<[Node<'a>]>, close: &'a str) -> Self {
        Self {
            expression,
            expressions,
            operator: match format!("{}{}", open, close).as_str() {
                "()" => "__cl__",
                "[]" => "__id__",
                "<>" => "__gn__",
                _ => panic!(),
            }
        }
    }
}

impl<'a> Executable<'a> for Sequence<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        let value = execute!(engine, &self.expression).read()?;
        let mut arguments = Vec::new();
        for argument in self.expressions.iter() {
            arguments.push(execute!(engine, argument));
        }

        let array = engine.new_array(arguments).read()?;
        value.call_method(engine, &self.operator, vec![array])
    }
}
