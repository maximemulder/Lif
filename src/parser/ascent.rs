use crate::element::Element;
use crate::parser::Parse;
use crate::node::Node;

pub trait Ascent {
    fn ascent(&self, parse: &mut Parse, nodes: Vec<Node>) -> Option<Vec<Node>>;
}

pub struct AscentDescent {
    descent: usize,
}

impl AscentDescent {
    pub fn new(descent: usize) -> Self {
        Self {
            descent,
        }
    }
}

impl Ascent for AscentDescent {
    fn ascent(&self, parse: &mut Parse, mut nodes: Vec<Node>) -> Option<Vec<Node>> {
        parse.descent(self.descent).map(|others| {
            nodes.extend(others);
            nodes
        })
    }
}

pub struct AscentChoice {
    ascents: Box<[usize]>,
}

impl AscentChoice {
    pub fn new<const N: usize>(ascents: [usize; N]) -> Self {
        Self {
            ascents: Box::new(ascents),
        }
    }
}

impl Ascent for AscentChoice {
    fn ascent(&self, parse: &mut Parse, nodes: Vec<Node>) -> Option<Vec<Node>> {
        for ascent in self.ascents.iter() {
            if let Some(others) = parse.ascent(*ascent, nodes.clone()) {
                return Some(others);
            }
        }

        Some(nodes)
    }
}

pub struct AscentSequence {
    ascents: Box<[usize]>,
}

impl AscentSequence {
    pub fn new<const N: usize>(ascents: [usize; N]) -> Self {
        Self {
            ascents: Box::new(ascents),
        }
    }
}

impl Ascent for AscentSequence {
    fn ascent(&self, parse: &mut Parse, mut nodes: Vec<Node>) -> Option<Vec<Node>> {
        for ascent in self.ascents.iter() {
            nodes = parse.ascent(*ascent, nodes)?;
        }

        Some(nodes)
    }
}

pub struct AscentOption {
    ascent: usize,
}

impl AscentOption {
    pub fn new(ascent: usize) -> Self {
        Self {
            ascent,
        }
    }
}

impl Ascent for AscentOption {
    fn ascent(&self, parse: &mut Parse, nodes: Vec<Node>) -> Option<Vec<Node>> {
        parse.ascent(self.ascent, nodes.clone()).or_else(|| Some(nodes))
    }
}

pub struct AscentElement {
    element: &'static Element,
}

impl AscentElement {
    pub fn new(element: &'static Element) -> Self {
        Self {
            element,
        }
    }
}

impl Ascent for AscentElement {
    fn ascent(&self, parse: &mut Parse, nodes: Vec<Node>) -> Option<Vec<Node>> {
        Some(vec![Node::new_production(parse.code, self.element, nodes.into_boxed_slice())])
    }
}
