use crate::node::Node;
use crate::element::Element;
use crate::parser::Parser;

pub trait Descent {
    fn descent<'a>(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>>;
}

pub struct DescentAlias {
    descent: usize,
}

impl DescentAlias {
    pub fn new(descent: usize) -> Self {
        Self {
            descent,
        }
    }
}

impl Descent for DescentAlias {
    fn descent<'a>(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        parser.descent(self.descent)
    }
}

pub struct DescentAscent {
    descent: usize,
    ascent: usize,
}

impl DescentAscent {
    pub fn new(descent: usize, ascent: usize) -> Self {
        Self {
            descent,
            ascent,
        }
    }
}

impl Descent for DescentAscent {
    fn descent<'a>(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        if let Some(nodes) = parser.descent(self.descent) {
            return parser.ascent(self.ascent, nodes);
        }

        None
    }
}

pub struct DescentChoice {
    descents: Box<[usize]>,
}

impl DescentChoice {
    pub fn new<const N: usize>(descents: [usize; N]) -> Self {
        Self {
            descents: Box::new(descents),
        }
    }
}

impl Descent for DescentChoice {
    fn descent<'a>(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        for descent in self.descents.iter() {
            if let Some(nodes) = parser.descent(*descent) {
                return Some(nodes);
            }
        }

        None
    }
}

pub struct DescentSequence {
    descents: Box<[usize]>,
}

impl DescentSequence {
    pub fn new<const N: usize>(descents: [usize; N]) -> Self {
        Self {
            descents: Box::from(descents),
        }
    }
}

impl Descent for DescentSequence {
    fn descent<'a>(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        let mut nodes = Vec::new();
        for descent in self.descents.iter() {
            if let Some(children) = parser.descent(*descent) {
                nodes.extend(children);
            } else {
                return None;
            }
        }

        Some(nodes)
    }
}

pub struct DescentZeroOrMore {
    descent: usize,
}

impl DescentZeroOrMore {
    pub fn new(descent: usize) -> Self {
        Self {
            descent,
        }
    }
}

impl Descent for DescentZeroOrMore {
    fn descent<'a>(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        let mut nodes = Vec::new();
        while let Some(children) = parser.descent(self.descent) {
            nodes.extend(children);
        }

        Some(nodes)
    }
}

pub struct DescentOneOrMore {
    descent:   usize,
}

impl DescentOneOrMore {
    pub fn new(descent: usize) -> Self {
        Self {
            descent,
        }
    }
}

impl Descent for DescentOneOrMore {
    fn descent<'a>(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        let mut nodes = Vec::new();
        while let Some(children) = parser.descent(self.descent) {
            nodes.extend(children);
        }

        if !nodes.is_empty() {
            Some(nodes)
        } else {
            None
        }
    }
}

pub struct DescentOption {
    descent: usize,
}

impl DescentOption {
    pub fn new(descent: usize) -> Self {
        Self {
            descent,
        }
    }
}

impl Descent for DescentOption {
    fn descent<'a>(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        let nodes = parser.descent(self.descent);
        if nodes.is_some() {
            return nodes;
        }

        Some(Vec::new())
    }
}

pub struct DescentPredicateAnd {
    descent: usize,
}

impl DescentPredicateAnd {
    pub fn new(descent: usize) -> Self {
        Self {
            descent,
        }
    }
}

impl Descent for DescentPredicateAnd {
    fn descent<'a>(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        if parser.descent_predicate(self.descent) {
            Some(Vec::new())
        } else {
            None
        }
    }
}

pub struct DescentPredicateNot {
    descent: usize,
}

impl DescentPredicateNot {
    pub fn new(descent: usize) -> Self {
        Self {
            descent,
        }
    }
}

impl Descent for DescentPredicateNot {
    fn descent<'a>(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        if parser.descent_predicate(self.descent) {
            None
        } else {
            Some(Vec::new())
        }
    }
}

pub struct DescentElement {
    descent: usize,
    element: &'static Element,
}

impl DescentElement {
    pub fn new(descent: usize, element: &'static Element) -> Self {
        Self {
            descent,
            element,
        }
    }
}

impl Descent for DescentElement {
    fn descent<'a>(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        if let Some(nodes) = parser.descent(self.descent) {
            Some(vec![Node::new_production(parser.code, self.element, nodes)])
        } else {
            None
        }
    }
}

pub struct DescentToken {
    element: &'static Element,
}

impl DescentToken {
    pub fn new(element: &'static Element) -> Self {
        Self {
            element,
        }
    }
}

impl Descent for DescentToken {
    fn descent<'a>(&self, parser: &mut Parser<'a, '_>) -> Option<Vec<Node<'a>>> {
        if let Some(token) = parser.next() {
            if token.element == self.element {
                return Some(vec![token]);
            }
        }

        None
    }
}
