use crate::node::Node;
use crate::element::Element;
use crate::parser2::Parser;

pub trait Rule<'a> {
	fn rule<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>>;
}

pub struct RuleAlias {
	rule: usize,
}

impl RuleAlias {
	pub fn new(rule: usize) -> Self {
		return Self {
			rule,
		};
	}
}

impl<'a> Rule<'a> for RuleAlias {
	fn rule<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		return parser.rule(self.rule);
	}
}

pub struct RuleFilter {
	rule: usize,
	filter: usize,
}

impl RuleFilter {
	pub fn new(rule: usize, filter: usize) -> Self {
		return Self {
			rule,
			filter,
		};
	}
}

impl<'a> Rule<'a> for RuleFilter {
	fn rule<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		if let Some(nodes) = parser.rule(self.rule) {
			return parser.filter(self.filter, nodes);
		}

		return None;
	}
}

pub struct RuleChoice {
	rules: Vec<usize>,
}

impl RuleChoice {
	pub fn new(rules: Vec<usize>) -> Self {
		return Self {
			rules,
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

pub struct RuleSequence {
	rules: Vec<usize>,
}

impl RuleSequence {
	pub fn new(rules: Vec<usize>) -> Self {
		return Self {
			rules,
		};
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

pub struct RuleList {
	rule:   usize,
}

impl RuleList {
	pub fn new(rule: usize) -> Self {
		return Self {
			rule,
		};
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

pub struct RuleOption {
	rule: usize,
}

impl RuleOption {
	pub fn new(rule: usize) -> Self {
		return Self {
			rule,
		};
	}
}

impl<'a> Rule<'a> for RuleOption {
	fn rule<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		let nodes = parser.rule(self.rule);
		if nodes.is_some() {
			return nodes;
		}

		return Some(Vec::new());
	}
}

pub struct RulePredicateAnd {
	rule: usize,
}

impl RulePredicateAnd {
	pub fn new(rule: usize) -> Self {
		return Self {
			rule,
		};
	}
}

impl<'a> Rule<'a> for RulePredicateAnd {
	fn rule<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		return if parser.rule_predicate(self.rule) {
			Some(Vec::new())
		} else {
			None
		};
	}
}

pub struct RulePredicateNot {
	rule: usize,
}

impl RulePredicateNot {
	pub fn new(rule: usize) -> Self {
		return Self {
			rule,
		};
	}
}

impl<'a> Rule<'a> for RulePredicateNot {
	fn rule<'b>(&self, parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
		return if parser.rule_predicate(self.rule) {
			None
		} else {
			Some(Vec::new())
		};
	}
}

pub struct RuleToken<'a> {
	element: &'a Element,
}

impl<'a> RuleToken<'a> {
	pub fn new(element: &'a Element) -> Self {
		return Self {
			element,
		};
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
