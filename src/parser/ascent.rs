use crate::element::Element;
use crate::parser::Parse;
use crate::node::Node;

pub trait Ascent {
    fn ascent<'a>(&self, parser: &mut Parse<'_, 'a>, nodes: Vec<Node<'a>>) -> Option<Vec<Node<'a>>>;
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
    fn ascent<'a>(&self, parser: &mut Parse<'_, 'a>, mut nodes: Vec<Node<'a>>) -> Option<Vec<Node<'a>>> {
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
    fn ascent<'a>(&self, parser: &mut Parse<'_, 'a>, mut nodes: Vec<Node<'a>>) -> Option<Vec<Node<'a>>> {
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
    fn ascent<'a>(&self, parser: &mut Parse<'_, 'a>, nodes: Vec<Node<'a>>) -> Option<Vec<Node<'a>>> {
        Some(vec![Node::new_production(parser.code, self.element, nodes)])
    }
}
