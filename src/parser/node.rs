use crate::parser::Code;
use crate::parser::Element;
use crate::memory::Ref;

#[derive(Clone)]
pub struct SNode {
    pub code: Ref<Code>,
    pub element: &'static Element,
    pub content: SNodeContent,
}

#[derive(Clone)]
pub enum SNodeContent {
    Production(Box<[SNode]>),
    Token(usize, usize),
}

impl SNode {
    pub fn new_token(code: Ref<Code>, element: &'static Element, delimiters: (usize, usize)) -> Self {
        Self {
            code,
            element,
            content: SNodeContent::Token(delimiters.0, delimiters.1),
        }
    }

    pub fn new_production(code: Ref<Code>, element: &'static Element, children: Box<[SNode]>) -> Self {
        Self {
            code,
            element,
            content: SNodeContent::Production(children),
        }
    }

    pub fn children(&self) -> &[SNode] {
        if let SNodeContent::Production(children) = &self.content {
            return children;
        }

        panic!();
    }

    pub fn length(&self) -> usize {
        self.children().len()
    }

    pub fn front(&self, index: usize) -> Ref<SNode>{
        Ref::new(&self.children()[index])
    }

    pub fn back(&self, index: usize) -> Ref<SNode> {
        let children = self.children();
        Ref::new(&children[children.len() - index])
    }

    pub fn text(&self) -> Ref<str> {
        Ref::new(self.code.node_str(self))
    }

    pub fn left(&self) -> usize {
        match &self.content {
            SNodeContent::Production(children) => children.first().unwrap().left(),
            SNodeContent::Token(left, _) => *left,
        }
    }

    pub fn right(&self) -> usize {
        match &self.content {
            SNodeContent::Production(children) => children.last().unwrap().right(),
            SNodeContent::Token(_, right) => *right,
        }
    }
}
