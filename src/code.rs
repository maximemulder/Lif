use crate::memory::{ Own, Ref };
use crate::node::Node as ANode;
use crate::walker::Node as CNode;
use crate::parser::Parser;
use std::fs::read_to_string;

pub struct Code {
    pub name: Option<Box<str>>,
    pub text: Box<str>,
    pub ast: Option<ANode>,
    pub cst: Option<CNode>,
}

impl Code {
    fn new(parser: &Parser, production: usize, program: &dyn Fn(Ref<ANode>) -> CNode, name: Option<&str>, text: Box<str>) -> Own<Self> {
        let mut code = Own::new(Self {
            text,
            name: name.map(Box::from),
            ast: None,
            cst: None,
        });

        let ast = parser.parse(production, code.get_ref()).unwrap();
        code.ast = Some(ast);
        code.cst = Some(program(Ref::new(code.ast.as_ref().unwrap())));
        code

    }

    pub fn from_file(parser: &Parser, production: usize, program: &dyn Fn(Ref<ANode>) -> CNode, name: &str) -> Option<Own<Self>> {
        Some(Code::new(parser, production, program, Some(name), read_to_string(name).ok()?.into_boxed_str()))
    }

    pub fn from_string(parser: &Parser, production: usize, program: &dyn Fn(Ref<ANode>) -> CNode, text: &str) -> Own<Self> {
        Code::new(parser, production, program, None, Box::from(text))
    }

    pub fn node_str(&self, node: &ANode) -> &str {
        &self.text[node.left() .. node.right()]
    }

    pub fn node_line(&self, node: &ANode) -> &str {
        let index = node.left();
        &self.text[index - self.line_pos_left(index) .. index + self.line_pos_right(index)]
    }

    pub fn node_x(&self, node: &ANode) -> usize {
        self.line_pos_left(node.left()) + 1
    }

    pub fn node_y(&self, node: &ANode) -> usize {
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

    pub fn node_shift_left(&self, node: &ANode) -> usize {
        self.line_shift(self.index_iterator_reverse(node.left()))
    }

    pub fn node_shift_right(&self, node: &ANode) -> usize {
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
