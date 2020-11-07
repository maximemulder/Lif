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
                "=="  => "__eq__",
                "!="  => "__ne__",
                "<"   => "__lt__",
                ">"   => "__gt__",
                "<="  => "__le__",
                ">="  => "__ge__",
                "&&"  => "__and__",
                "||"  => "__or__",
                "+"   => "__add__",
                "-"   => "__sub__",
                "*"   => "__mul__",
                "/"   => "__div__",
                "%"   => "__rem__",
                "&"   => "__band__",
                "|"   => "__bor__",
                "^"   => "__bxor__",
                "<<"  => "__bls__",
                ">>"  => "__brs__",
                "<<<" => "__bcls__",
                ">>>" => "__bcrs__",
                _     => panic!(),
            },
        }
    }
}

impl<'a> Executable<'a> for Binop<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        let left = execute!(engine, &self.left).read()?;
        match self.operator {
            "__and__" => {
                left.cast(engine.primitives.boolean)?;
                let boolean = if *left.data_boolean() {
                    let right = execute!(engine, &self.right).read()?;
                    right.cast(engine.primitives.boolean)?;
                    *right.data_boolean()
                } else {
                    false
                };

                Ok(engine.new_boolean(boolean))
            },
            "__or__" => {
                left.cast(engine.primitives.boolean)?;
                let boolean = if *left.data_boolean() {
                    true
                } else {
                    let right = execute!(engine, &self.right).read()?;
                    right.cast(engine.primitives.boolean)?;
                    *right.data_boolean()
                };

                Ok(engine.new_boolean(boolean))
            },
            _ => {
                let right = execute!(engine, &self.right).read()?;
                left.get_method(&self.operator).unwrap().call(engine, vec![left, right])
            },
        }
    }
}
