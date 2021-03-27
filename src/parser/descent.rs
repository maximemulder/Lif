use crate::node::Node;
use crate::element::Element;
use crate::parser::Parse;
use crate::parser::arena::ArenaRef;
use crate::parser::ascent::Ascent;

pub trait Descent {
    fn descent(&self, parse: &mut Parse) -> Option<Vec<Node>>;
}

pub struct DescentAlias {
    descent: ArenaRef<dyn Descent>,
}

impl DescentAlias {
    pub fn new(descent: ArenaRef<dyn Descent>) -> Self {
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
    ascent: ArenaRef<dyn Ascent>,
}

impl DescentAscent {
    pub fn new(ascent: ArenaRef<dyn Ascent>) -> Self {
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
    descents: Box<[ArenaRef<dyn Descent>]>,
}

impl DescentChoice {
    pub fn new(descents: Box<[ArenaRef<dyn Descent>]>) -> Self {
        Self {
            descents,
        }
    }
}

impl Descent for DescentChoice {
    fn descent(&self, parse: &mut Parse) -> Option<Vec<Node>> {
        for descent in self.descents.iter().copied() {
            if let Some(nodes) = parse.descent(descent) {
                return Some(nodes);
            }
        }

        None
    }
}

pub struct DescentSequence {
    descents: Box<[ArenaRef<dyn Descent>]>,
}

impl DescentSequence {
    pub fn new(descents: Box<[ArenaRef<dyn Descent>]>) -> Self {
        Self {
            descents,
        }
    }
}

impl Descent for DescentSequence {
    fn descent(&self, parse: &mut Parse) -> Option<Vec<Node>> {
        let mut nodes = Vec::new();
        for descent in self.descents.iter().copied() {
            nodes.extend(parse.descent(descent)?);
        }

        Some(nodes)
    }
}

pub struct DescentZeroOrMore {
    descent: ArenaRef<dyn Descent>,
}

impl DescentZeroOrMore {
    pub fn new(descent: ArenaRef<dyn Descent>) -> Self {
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
    descent: ArenaRef<dyn Descent>,
}

impl DescentOneOrMore {
    pub fn new(descent: ArenaRef<dyn Descent>) -> Self {
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
    descent: ArenaRef<dyn Descent>,
}

impl DescentOption {
    pub fn new(descent: ArenaRef<dyn Descent>) -> Self {
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
    descent: ArenaRef<dyn Descent>,
}

impl DescentPredicateAnd {
    pub fn new(descent: ArenaRef<dyn Descent>) -> Self {
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
    descent: ArenaRef<dyn Descent>,
}

impl DescentPredicateNot {
    pub fn new(descent: ArenaRef<dyn Descent>) -> Self {
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
    descent: ArenaRef<dyn Descent>,
    element: &'static Element,
}

impl DescentElement {
    pub fn new(descent: ArenaRef<dyn Descent>, element: &'static Element) -> Self {
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
