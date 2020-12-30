use crate::element::Element;
use crate::parser::Parse;
use crate::node::Node;

pub trait Ascent {
    fn ascent(&self, parser: &mut Parse, nodes: Vec<Node>) -> Option<Vec<Node>>;
}

pub struct AscentList {
    ascents: Box<[usize]>,
}

impl AscentList {
    pub fn new<const N: usize>(ascents: [usize; N]) -> Self {
        Self {
            ascents: Box::new(ascents),
        }
    }
}

impl Ascent for AscentList {
    fn ascent(&self, parser: &mut Parse, mut nodes: Vec<Node>) -> Option<Vec<Node>> {
        for ascent in self.ascents.iter().rev() {
            if let Some(others) = parser.ascent(*ascent, nodes) {
                nodes = others;
            } else {
                return None;
            }
        }

        Some(nodes)
    }
}

pub struct AscentExtension {
    descent: usize,
    ascent: usize,
}

impl AscentExtension {
    pub fn new(descent: usize, ascent: usize) -> Self {
        Self {
            descent,
            ascent,
        }
    }
}

impl Ascent for AscentExtension {
    fn ascent(&self, parser: &mut Parse, mut nodes: Vec<Node>) -> Option<Vec<Node>> {
        if let Some(children) = parser.descent(self.descent) {
            nodes.extend(children);
            return parser.ascent(self.ascent, nodes);
        }

        Some(nodes)
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
    fn ascent(&self, parser: &mut Parse, nodes: Vec<Node>) -> Option<Vec<Node>> {
        Some(vec![Node::new_production(parser.code, self.element, nodes)])
    }
}
