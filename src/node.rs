use crate::element::Element;

#[derive(Clone)]
pub enum Type<'a> {
    Production(Vec<Node<'a>>),
    Token(usize, usize),
}

#[derive(Clone)]
pub struct Node<'a> {
    pub element: &'a Element,
    pub r#type: Type<'a>,
}

impl<'a> Node<'a> {
    pub fn new_token(element: &'a Element, delimiters: (usize, usize)) -> Self {
        Self {
            element,
            r#type: Type::Token(delimiters.0, delimiters.1),
        }
    }

    pub fn new_production(element: &'a Element, children: Vec<Node<'a>>) -> Self {
        Self {
            element,
            r#type: Type::Production(children),
        }
    }

    pub fn children(&self) -> &Vec<Node<'a>> {
        match &self.r#type {
            Type::Production(children) => children,
            Type::Token(_, _) => panic!(),
        }
    }

    pub fn left(&self) -> usize {
        match &self.r#type {
            Type::Production(children) => children.first().unwrap().left(),
            Type::Token(left, _) => *left,
        }
    }

    pub fn right(&self) -> usize {
        match &self.r#type {
            Type::Production(children) => children.last().unwrap().right(),
            Type::Token(_, right) => *right,
        }
    }
}
