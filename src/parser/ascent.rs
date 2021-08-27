use crate::parser::{ CNode, Element, Parse };
use crate::parser::arena::ArenaRef;
use crate::parser::descent::Descent;

pub trait Ascent {
    fn ascent(&self, parse: &mut Parse, nodes: Vec<CNode>) -> Option<Vec<CNode>>;
}

pub struct AscentDescent {
    descent: ArenaRef<dyn Descent>,
}

impl AscentDescent {
    pub fn new(descent: ArenaRef<dyn Descent>) -> Self {
        Self {
            descent,
        }
    }
}

impl Ascent for AscentDescent {
    fn ascent(&self, parse: &mut Parse, nodes: Vec<CNode>) -> Option<Vec<CNode>> {
        parse.descent(self.descent).map(|others| nodes.into_iter().chain(others).collect())
    }
}

pub struct AscentChoice {
    ascents: Box<[ArenaRef<dyn Ascent>]>,
}

impl AscentChoice {
    pub fn new(ascents: Box<[ArenaRef<dyn Ascent>]>) -> Self {
        Self {
            ascents,
        }
    }
}

impl Ascent for AscentChoice {
    fn ascent(&self, parse: &mut Parse, nodes: Vec<CNode>) -> Option<Vec<CNode>> {
        self.ascents.iter()
            .copied()
            .find_map(|ascent| parse.ascent(ascent, nodes.clone()))
    }
}

pub struct AscentSequence {
    ascents: Box<[ArenaRef<dyn Ascent>]>,
}

impl AscentSequence {
    pub fn new(ascents: Box<[ArenaRef<dyn Ascent>]>) -> Self {
        Self {
            ascents,
        }
    }
}

impl Ascent for AscentSequence {
    fn ascent(&self, parse: &mut Parse, nodes: Vec<CNode>) -> Option<Vec<CNode>> {
        self.ascents.iter()
            .copied()
            .fold(Some(nodes), |nodes, ascent| parse.ascent(ascent, nodes?))
    }
}

pub struct AscentOption {
    ascent: ArenaRef<dyn Ascent>,
}

impl AscentOption {
    pub fn new(ascent: ArenaRef<dyn Ascent>) -> Self {
        Self {
            ascent,
        }
    }
}

impl Ascent for AscentOption {
    fn ascent(&self, parse: &mut Parse, nodes: Vec<CNode>) -> Option<Vec<CNode>> {
        parse.ascent(self.ascent, nodes.clone()).or(Some(nodes))
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
    fn ascent(&self, parse: &mut Parse, nodes: Vec<CNode>) -> Option<Vec<CNode>> {
        Some(vec![CNode::new_production(parse.code, self.element, nodes.into_boxed_slice())])
    }
}
