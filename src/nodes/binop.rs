use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Binop<'a> {
    left:     Node<'a>,
    right:    Node<'a>,
    operator: &'a str,
}

impl<'a> Binop<'a> {
    pub fn new(left: Node<'a>, operator: &'a str, right: Node<'a>) -> Self {
        Self {
            left,
            right,
            operator: match operator {
                "==" => "__eq__",
                "!=" => "__ne__",
                "<"  => "__lt__",
                ">"  => "__gt__",
                "<=" => "__le__",
                ">=" => "__ge__",
                "+" => "__add__",
                "-" => "__sub__",
                "*" => "__mul__",
                "/" => "__div__",
                "%" => "__rem__",
                _    => panic!(),
            },
        }
    }
}

impl<'a> Executable<'a> for Binop<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        let left  = execute!(engine, &self.left).read()?;
        let right = execute!(engine, &self.right).read()?;
        left.get_method(&self.operator).unwrap().call(engine, vec![left, right])
    }
}
