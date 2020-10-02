use crate::element::Element;
use crate::parser::Parser;
use crate::node::Node;

pub trait Ascent<'a> {
	fn ascent(&self, parser: &mut Parser<'a, '_>, nodes: Vec<Node<'a>>) -> Option<Vec<Node<'a>>>;
}

pub struct AscentList {
	ascents: Vec<usize>,
}

impl AscentList {
	pub fn new(ascents: Vec<usize>) -> Self {
		return Self {
			ascents,
		};
	}
}

impl<'a> Ascent<'a> for AscentList {
	fn ascent(&self, parser: &mut Parser<'a, '_>, mut nodes: Vec<Node<'a>>) -> Option<Vec<Node<'a>>> {
		for ascent in self.ascents.iter().rev() {
			if let Some(others) = parser.ascent(*ascent, nodes) {
				nodes = others;
			} else {
				return None;
			}
		}

		return Some(nodes);
	}
}

pub struct AscentExtension {
	descent: usize,
	ascent: usize,
}

impl AscentExtension {
	pub fn new(descent: usize, ascent: usize) -> Self {
		return Self {
			descent,
			ascent,
		};
	}
}

impl<'a> Ascent<'a> for AscentExtension {
	fn ascent(&self, parser: &mut Parser<'a, '_>, mut nodes: Vec<Node<'a>>) -> Option<Vec<Node<'a>>> {
		if let Some(children) = parser.descent(self.descent) {
			nodes.extend(children);
			return parser.ascent(self.ascent, nodes);
		}

		return Some(nodes);
	}
}

pub struct AscentElement<'a> {
	element: &'a Element,
}

impl<'a> AscentElement<'a> {
	pub fn new(element: &'a Element) -> Self {
		return Self {
			element,
		};
	}
}

impl<'a> Ascent<'a> for AscentElement<'a> {
	fn ascent(&self, parser: &mut Parser<'a, '_>, nodes: Vec<Node<'a>>) -> Option<Vec<Node<'a>>> {
		return Some(vec![Node::new_production(self.element, nodes)]);
	}
}
