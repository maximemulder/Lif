use crate::parser::{ CNode, Element, Parse };
use crate::parser::arena::ArenaRef;
use crate::parser::ascent::Ascent;

use std::iter;
use std::ops::Not;

pub trait Descent {
    fn descent(&self, parse: &mut Parse) -> Option<Vec<CNode>>;
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<CNode>> {
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<CNode>> {
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<CNode>> {
        self.descents.iter()
            .copied()
            .find_map(|descent| parse.descent(descent))
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<CNode>> {
        self.descents.iter()
            .copied()
            .map(|descent| parse.descent(descent))
            .flat_map(|option|
                option.map_or_else(|| vec![None], |nodes| nodes.into_iter().map(Some).collect())
            )
            .collect::<Option<Vec<CNode>>>()
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<CNode>> {
        Some(iter::from_fn(|| parse.descent(self.descent)).flatten().collect())
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<CNode>> {
        let nodes = iter::from_fn(|| parse.descent(self.descent)).flatten().collect::<Vec<CNode>>();
        nodes.is_empty().not().then_some(nodes)
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<CNode>> {
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<CNode>> {
        parse.descent_predicate(self.descent).then(Vec::new)
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<CNode>> {
        parse.descent_predicate(self.descent).not().then(Vec::new)
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<CNode>> {
        parse.descent(self.descent).map(|nodes| vec![CNode::new_production(parse.code, self.element, nodes.into_boxed_slice())])
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
    fn descent(&self, parse: &mut Parse) -> Option<Vec<CNode>> {
        let token = parse.next()?;
        (token.element == self.element).then(|| vec![token])
    }
}
