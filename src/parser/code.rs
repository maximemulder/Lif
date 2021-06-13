use crate::memory::{ Own, Ref };
use crate::parser::Grammar;
use crate::parser::SNode;
use crate::walker::WNode;
use std::fs::read_to_string;

pub struct Code {
    pub name:        Option<Box<str>>,
    pub text:        Box<str>,
    pub syntax_tree: Option<SNode>,
    pub walk_tree:   Option<WNode>,
}

impl Code {
    fn new(grammar: &Grammar, production: usize, program: &dyn Fn(Ref<SNode>) -> WNode, name: Option<&str>, text: Box<str>) -> Own<Self> {
        let mut code = Own::new(Self {
            text,
            name: name.map(Box::from),
            syntax_tree: None,
            walk_tree: None,
        });

        let syntax_tree = grammar.parse(production, code.get_ref()).unwrap();
        code.syntax_tree = Some(syntax_tree);
        code.walk_tree = Some(program(Ref::new(code.syntax_tree.as_ref().unwrap())));
        code

    }

    pub fn from_file(grammar: &Grammar, production: usize, program: &dyn Fn(Ref<SNode>) -> WNode, name: &str) -> Option<Own<Self>> {
        Some(Code::new(grammar, production, program, Some(name), read_to_string(name).ok()?.into_boxed_str()))
    }

    pub fn from_string(grammar: &Grammar, production: usize, program: &dyn Fn(Ref<SNode>) -> WNode, text: &str) -> Own<Self> {
        Code::new(grammar, production, program, None, Box::from(text))
    }

    pub fn node_str(&self, node: &SNode) -> &str {
        &self.text[node.left() .. node.right()]
    }

    pub fn node_line(&self, node: &SNode) -> &str {
        let index = node.left();
        &self.text[index - self.line_pos_left(index) .. index + self.line_pos_right(index)]
    }

    pub fn node_x(&self, node: &SNode) -> usize {
        self.line_pos_left(node.left()) + 1
    }

    pub fn node_y(&self, node: &SNode) -> usize {
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

    pub fn node_shift_left(&self, node: &SNode) -> usize {
        self.line_shift(self.index_iterator_reverse(node.left()))
    }

    pub fn node_shift_right(&self, node: &SNode) -> usize {
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
