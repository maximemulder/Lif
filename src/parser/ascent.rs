use crate::element::Element;
use crate::parser::Parser;
use crate::parser::arena2::ArenaRef;
use crate::node::Node;

pub trait Ascent<'a> {
    fn ascent(&self, parser: &mut Parser<'a, '_>, nodes: Vec<Node<'a>>) -> Option<Vec<Node<'a>>>;
}

pub struct AscentList<'a> {
    ascents: Box<[ArenaRef<dyn Ascent<'a>>]>,
}

impl<'a> AscentList<'a> {
    pub fn new<const N: usize>(ascents: [ArenaRef<dyn Ascent<'a>>; N]) -> Self {
        Self {
            ascents: Box::new(ascents),
        }
    }
}

impl<'a> Ascent<'a> for AscentList<'a> {
    fn ascent(&self, parser: &mut Parser<'a, '_>, mut nodes: Vec<Node<'a>>) -> Option<Vec<Node<'a>>> {
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

impl<'a> Ascent<'a> for AscentExtension {
    fn ascent(&self, parser: &mut Parser<'a, '_>, mut nodes: Vec<Node<'a>>) -> Option<Vec<Node<'a>>> {
        if let Some(children) = parser.descent(self.descent) {
            nodes.extend(children);
            return parser.ascent(self.ascent, nodes);
        }

        Some(nodes)
    }
}

pub struct AscentElement<'a> {
    element: &'a Element,
}

impl<'a> AscentElement<'a> {
    pub fn new(element: &'a Element) -> Self {
        Self {
            element,
        }
    }
}

impl<'a> Ascent<'a> for AscentElement<'a> {
    fn ascent(&self, parser: &mut Parser<'a, '_>, nodes: Vec<Node<'a>>) -> Option<Vec<Node<'a>>> {
        Some(vec![Node::new_production(self.element, nodes)])
    }
}
