use crate::element::Element;
use crate::parser::Parse;
use crate::parser::arena::ArenaRef;
use crate::parser::descent::Descent;
use crate::node::Node;

pub trait Ascent {
    fn ascent(&self, parse: &mut Parse, nodes: Vec<Node>) -> Option<Vec<Node>>;
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
    fn ascent(&self, parse: &mut Parse, mut nodes: Vec<Node>) -> Option<Vec<Node>> {
        parse.descent(self.descent).map(|others| {
            nodes.extend(others);
            nodes
        })
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
    fn ascent(&self, parse: &mut Parse, nodes: Vec<Node>) -> Option<Vec<Node>> {
        for ascent in self.ascents.iter().copied() {
            if let Some(others) = parse.ascent(ascent, nodes.clone()) {
                return Some(others);
            }
        }

        None
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
    fn ascent(&self, parse: &mut Parse, mut nodes: Vec<Node>) -> Option<Vec<Node>> {
        for ascent in self.ascents.iter().copied() {
            nodes = parse.ascent(ascent, nodes)?;
        }

        Some(nodes)
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
