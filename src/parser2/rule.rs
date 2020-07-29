use crate::node::Node;
use crate::element::Element;
use crate::parser2::Parser;

pub trait Rule<'a> {
	fn rule<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>>;
}

pub struct RuleAlias {
	rule: usize,
}

pub struct RuleElement<'a> {
	rule: usize,
	element: &'a Element,
}

pub struct RuleChoice {
	rules: Vec<usize>,
}

pub struct RuleSequence {
	rules: Vec<usize>,
}

pub struct RuleList {
	rule: usize,
}

pub struct RuleRecurse<'a> {
	left: usize,
	right: usize,
	mapper: &'a dyn for<'b, 'c> Fn(Vec<Node<'b, 'c>>) -> Vec<Node<'b, 'c>>,
}

pub struct RuleOption {
	rule: usize,
}

pub struct RuleToken<'a> {
	element: &'a Element,
}

pub fn alias(rule: usize) -> RuleAlias {
	return RuleAlias {
		rule,
	};
}

pub fn element<'a>(rule: usize, element: &'a Element) -> RuleElement<'a> {
	return RuleElement {
		rule,
		element,
	};
}

pub fn choice(rules: Vec<usize>) -> RuleChoice {
	return RuleChoice {
		rules,
	};
}

pub fn sequence(rules: Vec<usize>) -> RuleSequence {
	return RuleSequence {
		rules,
	};
}

pub fn list(rule: usize) -> RuleList {
	return RuleList {
		rule,
	};
}

pub fn recurse<'c>(left: usize, right: usize, mapper: &'c dyn for<'a, 'b> Fn(Vec<Node<'a, 'b>>) -> Vec<Node<'a, 'b>>) -> RuleRecurse<'c> {
	return RuleRecurse {
		left,
		right,
		mapper,
	};
}

pub fn option(rule: usize) -> RuleOption {
	return RuleOption {
		rule,
	};
}

pub fn token<'a>(element: &'a Element) -> RuleToken<'a> {
	return RuleToken {
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

impl<'a> Rule<'a> for RuleAlias {
	fn rule<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		return parser.rule(self.rule);
	}
}

impl<'a> Rule<'a> for RuleElement<'a> {
	fn rule<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		return if let Some(children) = parser.rule(self.rule) {
			Some(vec![Node::new_production(self.element, children)])
		} else {
			None
		};
	}
}

impl<'a> Rule<'a> for RuleChoice {
	fn rule<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		for rule in self.rules.iter() {
			if let Some(nodes) = parser.rule(*rule) {
				return Some(nodes);
			}
		}

		return None;
	}
}

impl<'a> Rule<'a> for RuleSequence {
	fn rule<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		let mut nodes = Vec::new();
		for rule in self.rules.iter() {
			if let Some(children) = parser.rule(*rule) {
				nodes.extend(children);
			} else {
				return None;
			}
		}

		return Some(nodes);
	}
}

impl<'a> Rule<'a> for RuleList {
	fn rule<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		let mut nodes = Vec::new();
		while let Some(children) = parser.rule(self.rule) {
			nodes.extend(children);
		}

		return Some(nodes);
	}
}

impl<'a> Rule<'a> for RuleRecurse<'_> {
	fn rule<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		if let Some(mut nodes) = parser.rule(self.left) {
			while let Some(children) = parser.rule(self.right) {
				nodes.extend(children);
				nodes = (self.mapper)(nodes);
			}

			return Some(nodes);
		}

		return None;
	}
}

impl<'a> Rule<'a> for RuleOption {
	fn rule<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		return Some(vector(parser.rule(self.rule)));
	}
}

impl<'a> Rule<'a> for RuleToken<'a> {
	fn rule<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		if let Some(token) = parser.next() {
			if token.element == self.element {
				return Some(vec![token]);
			}
		}

		return None;
	}
}
