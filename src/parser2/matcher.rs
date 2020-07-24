use crate::node::Node;
use crate::element::Element;
use crate::parser2::Parser;

pub trait Matcher<'a> {
	fn go<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>>;
}

pub struct MatcherElement<'a> {
	matcher: usize,
	element: &'a Element,
}

pub struct MatcherChoice<'a> {
	matchers: &'a [usize],
}

pub struct MatcherSequence<'a> {
	matchers: &'a [usize],
}

pub struct MatcherList {
	matcher: usize,
}

pub struct MatcherOption {
	matcher: usize,
}

pub struct MatcherToken<'a> {
	element: &'a Element,
}

pub fn element<'a>(matcher: usize, element: &'a Element) -> MatcherElement<'a> {
	return MatcherElement {
		matcher,
		element,
	};
}

pub fn choice<'a>(matchers: &'a [usize]) -> MatcherChoice<'a> {
	return MatcherChoice {
		matchers,
	};
}

pub fn sequence<'a>(matchers: &'a [usize]) -> MatcherSequence<'a> {
	return MatcherSequence {
		matchers,
	};
}

pub fn list(matcher: usize) -> MatcherList {
	return MatcherList {
		matcher,
	};
}

pub fn option(matcher: usize) -> MatcherOption {
	return MatcherOption {
		matcher,
	};
}

pub fn token<'a>(element: &'a Element) -> MatcherToken<'a> {
	return MatcherToken {
		element,
	};
}

fn vector<T>(option: Option<Vec<T>>) -> Vec<T> {
	return if let Some(vector) = option {
		vector
	} else {
		Vec::new()
	};
}

impl<'a> Matcher<'a> for MatcherElement<'a> {
	fn go<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		return if let Some(children) = parser.go(self.matcher) {
			Some(vec![Node::new_production(self.element, children)])
		} else {
			None
		};
	}
}

impl<'a> Matcher<'a> for MatcherChoice<'a> {
	fn go<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		let cursor = parser.save();
		for matcher in self.matchers {
			parser.restore(cursor);
			if let Some(nodes) = parser.go(*matcher) {
				return Some(nodes);
			}
		}

		return None;
	}
}

impl<'a> Matcher<'a> for MatcherSequence<'a> {
	fn go<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		let mut nodes = Vec::new();
		for matcher in self.matchers {
			if let Some(children) = parser.go(*matcher) {
				nodes.extend(children);
			} else {
				return None;
			}
		}

		return Some(nodes);
	}
}

impl<'a> Matcher<'a> for MatcherList {
	fn go<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		let mut nodes = Vec::new();
		while let Some(node) = parser.go(self.matcher) {
			nodes.extend(node);
		}

		return Some(nodes);
	}
}

impl<'a> Matcher<'a> for MatcherOption {
	fn go<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		return Some(vector(parser.go(self.matcher)));
	}
}

impl<'a> Matcher<'a> for MatcherToken<'a> {
	fn go<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		if let Some(token) = parser.token() {
			if token.element == self.element {
				parser.advance();
				return Some(vec![token]);
			}
		}

		return None;
	}
}
