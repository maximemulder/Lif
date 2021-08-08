use crate::parser::Code;
use crate::parser::Element;
use crate::memory::Ref;

#[derive(Clone)]
pub struct SNode {
    pub code: Ref<Code>,
    pub element: &'static Element,
    children: Box<[SNode]>,
    left: usize,
    right: usize,
}

impl SNode {
    pub fn new(code: Ref<Code>,  element: &'static Element, children: Box<[SNode]>, left: usize, right: usize) -> Self {
        debug_assert!(right >= left);
        Self {
            code,
            element,
            children,
            left,
            right,
        }
    }

    pub fn new_token(code: Ref<Code>, element: &'static Element, left: usize, right: usize) -> Self {
        Self::new(code, element,  Box::new([]), left, right)
    }

    pub fn new_production(code: Ref<Code>, element: &'static Element, children: Box<[SNode]>) -> Self {
        let (left, right) = if !children.is_empty() {
            (children.first().unwrap().left(), children.first().unwrap().right())
        } else {
            (0, 0)
        };

        Self::new(code, element, children, left, right)
    }

    pub fn children(&self) -> &[SNode] {
        &self.children
    }

    pub fn left(&self) -> usize {
        self.left
    }

    pub fn right(&self) -> usize {
        self.right
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
}
