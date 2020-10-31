use crate::code::Code;
use crate::element::Element;

#[derive(Clone)]
pub enum Content<'a> {
    Production(Vec<Node<'a>>),
    Token(usize, usize),
}

#[derive(Clone)]
pub struct Node<'a> {
    pub code: &'a Code,
    pub element: &'static Element,
    pub content: Content<'a>,
}

impl<'a> Node<'a> {
    pub fn new_token(code: &'a Code, element: &'static Element, delimiters: (usize, usize)) -> Self {
        Self {
            code,
            element,
            content: Content::Token(delimiters.0, delimiters.1),
        }
    }

    pub fn new_production(code: &'a Code, element: &'static Element, children: Vec<Node<'a>>) -> Self {
        Self {
            code,
            element,
            content: Content::Production(children),
        }
    }

    pub fn children(&self) -> &Vec<Node<'a>> {
        if let Content::Production(children) = &self.content {
            return children;
        }

        panic!();
    }

    pub fn text(&self) -> &str {
        self.code.node_str(self)
    }

    pub fn left(&self) -> usize {
        match &self.content {
            Content::Production(children) => children.first().unwrap().left(),
            Content::Token(left, _) => *left,
        }
    }

    pub fn right(&self) -> usize {
        match &self.content {
            Content::Production(children) => children.last().unwrap().right(),
            Content::Token(_, right) => *right,
        }
    }
}
