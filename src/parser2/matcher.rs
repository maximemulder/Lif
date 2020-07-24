use crate::node::Node;
use crate::element::Element;
use crate::parser2::Parser;

pub trait Matcher<'a> {
	fn go<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>>;
}

pub struct MatcherElement<'a, 'b> {
	matcher: &'a dyn Matcher<'b>,
	element: &'b Element,
}

pub struct MatcherChoice<'a, 'b> {
	matchers: &'a [&'a dyn Matcher<'b>],
}

pub struct MatcherSequence<'a, 'b> {
	matchers: &'a [&'a dyn Matcher<'b>],
}

pub struct MatcherList<'a, 'b> {
	matcher: &'a dyn Matcher<'b>,
}

pub struct MatcherOption<'a, 'b> {
	matcher: &'a dyn Matcher<'b>,
}

pub struct MatcherToken<'a> {
	element: &'a Element,
}

pub fn element<'a, 'b>(matcher: &'a dyn Matcher<'b>, element: &'b Element) -> MatcherElement<'a, 'b> {
	return MatcherElement {
		matcher,
		element,
	};
}

pub fn choice<'a, 'b>(matchers: &'a [&'a dyn Matcher<'b>]) -> MatcherChoice<'a, 'b> {
	return MatcherChoice {
		matchers,
	};
}

pub fn sequence<'a, 'b>(matchers: &'a [&'a dyn Matcher<'b>]) -> MatcherSequence<'a, 'b> {
	return MatcherSequence {
		matchers,
	};
}

pub fn list<'a, 'b>(matcher: &'a dyn Matcher<'b>) -> MatcherList<'a, 'b> {
	return MatcherList {
		matcher,
	};
}

pub fn option<'a, 'b>(matcher: &'a dyn Matcher<'b>) -> MatcherOption<'a, 'b> {
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

impl<'a> Matcher<'a> for MatcherElement<'_, 'a> {
	fn go<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		return if let Some(children) = self.matcher.go(parser) {
			Some(vec![Node::new_production(self.element, children)])
		} else {
			None
		};
	}
}

impl<'a> Matcher<'a> for MatcherChoice<'_, 'a> {
	fn go<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		let cursor = parser.save();
		for matcher in self.matchers {
			parser.restore(cursor);
			if let Some(nodes) = matcher.go(parser) {
				return Some(nodes);
			}
		}

		return None;
	}
}

impl<'a> Matcher<'a> for MatcherSequence<'_, 'a> {
	fn go<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		let mut nodes = Vec::new();
		for matcher in self.matchers {
			if let Some(children) = matcher.go(parser) {
				nodes.extend(children);
			} else {
				return None;
			}
		}

		return Some(nodes);
	}
}

impl<'a> Matcher<'a> for MatcherList<'_, 'a> {
	fn go<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		let mut nodes = Vec::new();
		while let Some(node) = self.matcher.go(parser) {
			nodes.extend(node);
		}

		return Some(nodes);
	}
}

impl<'a> Matcher<'a> for MatcherOption<'_, 'a> {
	fn go<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		return Some(vector(self.matcher.go(parser)));
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
