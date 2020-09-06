use crate::node::Node;
use crate::element::Element;
use crate::parser2::Parser;

pub trait Descent<'a> {
	fn descent<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>>;
}

pub struct DescentAlias {
	descent: usize,
}

impl DescentAlias {
	pub fn new(descent: usize) -> Self {
		return Self {
			descent,
		};
	}
}

impl<'a> Descent<'a> for DescentAlias {
	fn descent<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		return parser.descent(self.descent);
	}
}

pub struct DescentAscent {
	descent: usize,
	ascent: usize,
}

impl DescentAscent {
	pub fn new(descent: usize, ascent: usize) -> Self {
		return Self {
			descent,
			ascent,
		};
	}
}

impl<'a> Descent<'a> for DescentAscent {
	fn descent<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		if let Some(nodes) = parser.descent(self.descent) {
			return parser.ascent(self.ascent, nodes);
		}

		return None;
	}
}

pub struct DescentChoice {
	descents: Vec<usize>,
}

impl DescentChoice {
	pub fn new(descents: Vec<usize>) -> Self {
		return Self {
			descents,
		};
	}
}

impl<'a> Descent<'a> for DescentChoice {
	fn descent<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		for descent in self.descents.iter() {
			if let Some(nodes) = parser.descent(*descent) {
				return Some(nodes);
			}
		}

		return None;
	}
}

pub struct DescentSequence {
	descents: Vec<usize>,
}

impl DescentSequence {
	pub fn new(descents: Vec<usize>) -> Self {
		return Self {
			descents,
		};
	}
}

impl<'a> Descent<'a> for DescentSequence {
	fn descent<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		let mut nodes = Vec::new();
		for descent in self.descents.iter() {
			if let Some(children) = parser.descent(*descent) {
				nodes.extend(children);
			} else {
				return None;
			}
		}

		return Some(nodes);
	}
}

pub struct DescentZeroOrMore {
	descent:   usize,
}

impl DescentZeroOrMore {
	pub fn new(descent: usize) -> Self {
		return Self {
			descent,
		};
	}
}

impl<'a> Descent<'a> for DescentZeroOrMore {
	fn descent<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		let mut nodes = Vec::new();
		while let Some(children) = parser.descent(self.descent) {
			nodes.extend(children);
		}

		return Some(nodes);
	}
}

pub struct DescentOneOrMore {
	descent:   usize,
}

impl DescentOneOrMore {
	pub fn new(descent: usize) -> Self {
		return Self {
			descent,
		};
	}
}

impl<'a> Descent<'a> for DescentOneOrMore {
	fn descent<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		let mut nodes = Vec::new();
		while let Some(children) = parser.descent(self.descent) {
			nodes.extend(children);
		}

		return if !nodes.is_empty() {
			Some(nodes)
		} else {
			None
		};
	}
}

pub struct DescentOption {
	descent: usize,
}

impl DescentOption {
	pub fn new(descent: usize) -> Self {
		return Self {
			descent,
		};
	}
}

impl<'a> Descent<'a> for DescentOption {
	fn descent<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		let nodes = parser.descent(self.descent);
		if nodes.is_some() {
			return nodes;
		}

		return Some(Vec::new());
	}
}

pub struct DescentPredicateAnd {
	descent: usize,
}

impl DescentPredicateAnd {
	pub fn new(descent: usize) -> Self {
		return Self {
			descent,
		};
	}
}

impl<'a> Descent<'a> for DescentPredicateAnd {
	fn descent<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		return if parser.descent_predicate(self.descent) {
			Some(Vec::new())
		} else {
			None
		};
	}
}

pub struct DescentPredicateNot {
	descent: usize,
}

impl DescentPredicateNot {
	pub fn new(descent: usize) -> Self {
		return Self {
			descent,
		};
	}
}

impl<'a> Descent<'a> for DescentPredicateNot {
	fn descent<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		return if parser.descent_predicate(self.descent) {
			None
		} else {
			Some(Vec::new())
		};
	}
}

pub struct DescentElement<'a> {
	descent: usize,
	element: &'a Element,
}

impl<'a> DescentElement<'a> {
	pub fn new(descent: usize, element: &'a Element) -> Self {
		return Self {
			descent,
			element,
		};
	}
}

impl<'a> Descent<'a> for DescentElement<'a> {
	fn descent<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		return if let Some(nodes) = parser.descent(self.descent) {
			Some(vec![Node::new_production(self.element, nodes)])
		} else {
			None
		};
	}
}

pub struct DescentToken<'a> {
	element: &'a Element,
}

impl<'a> DescentToken<'a> {
	pub fn new(element: &'a Element) -> Self {
		return Self {
			element,
		};
	}
}

impl<'a> Descent<'a> for DescentToken<'a> {
	fn descent<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		if let Some(token) = parser.next() {
			if token.element == self.element {
				return Some(vec![token]);
			}
		}

		return None;
	}
}
