use crate::code::Code;
use crate::element::Element;
use crate::memory::Ref;

#[derive(Clone)]
pub enum Content {
    Production(Box<[Node]>),
    Token(usize, usize),
}

#[derive(Clone)]
pub struct Node {
    pub code: Ref<Code>,
    pub element: &'static Element,
    pub content: Content,
}

impl Node {
    pub fn new_token(code: Ref<Code>, element: &'static Element, delimiters: (usize, usize)) -> Self {
        Self {
            code,
            element,
            content: Content::Token(delimiters.0, delimiters.1),
        }
    }

    pub fn new_production(code: Ref<Code>, element: &'static Element, children: Box<[Node]>) -> Self {
        Self {
            code,
            element,
            content: Content::Production(children),
        }
    }

    pub fn children(&self) -> &[Node] {
        if let Content::Production(children) = &self.content {
            return children;
        }

        panic!();
    }

    pub fn length(&self) -> usize {
        self.children().len()
    }

    pub fn front(&self, index: usize) -> Ref<Node>{
        Ref::new(&self.children()[index])
    }

    pub fn back(&self, index: usize) -> Ref<Node> {
        let children = self.children();
        Ref::new(&children[children.len() - index])
    }

    pub fn text(&self) -> Ref<str> {
        Ref::new(self.code.node_str(self))
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
