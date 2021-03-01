use crate::node::Node;
use crate::element::Element;
use crate::parser::Parse;

pub trait Descent {
    fn descent(&self, parse: &mut Parse) -> Option<Vec<Node>>;
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<Node>> {
        parse.descent(self.descent)
    }
}

pub struct DescentAscent {
    ascent: usize,
}

impl DescentAscent {
    pub fn new(ascent: usize) -> Self {
        Self {
            ascent,
        }
    }
}

impl Descent for DescentAscent {
    fn descent(&self, parse: &mut Parse) -> Option<Vec<Node>> {
        parse.ascent(self.ascent, Vec::new())
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<Node>> {
        for descent in self.descents.iter() {
            if let Some(nodes) = parse.descent(*descent) {
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<Node>> {
        let mut nodes = Vec::new();
        for descent in self.descents.iter() {
            nodes.extend(parse.descent(*descent)?);
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<Node>> {
        let mut nodes = Vec::new();
        while let Some(children) = parse.descent(self.descent) {
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<Node>> {
        let mut nodes = Vec::new();
        while let Some(children) = parse.descent(self.descent) {
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<Node>> {
        parse.descent(self.descent).or_else(|| Some(Vec::new()))
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<Node>> {
        if parse.descent_predicate(self.descent) {
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<Node>> {
        if parse.descent_predicate(self.descent) {
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<Node>> {
        parse.descent(self.descent).map(|nodes| vec![Node::new_production(parse.code, self.element, nodes.into_boxed_slice())])
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<Node>> {
        if let Some(token) = parse.next() {
            if token.element == self.element {
                return Some(vec![token]);
            }
        }

        None
    }
}
