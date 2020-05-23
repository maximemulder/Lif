use crate::element::Element;
use crate::elements;

#[derive(Clone)]
pub enum Content<'a, 'b> {
	Token(&'b str),
	Production(Vec<Node<'a, 'b>>)
}

#[derive(Clone)]
pub struct Node<'a, 'b> {
	pub element: &'a Element,
	pub content: Content<'a, 'b>,
}

impl<'a, 'b> Node<'a, 'b> {
	fn new(element: &'a Element, content: Content<'a, 'b>) -> Self {
		return Self {
			element,
			content,
		};
	}

	pub fn new_token(element: &'a Element, string: &'b str) -> Self {
		return Self::new(element, Content::Token(string));
	}

	pub fn new_production(element: &'a Element, children: Vec<Node<'a, 'b>>) -> Self {
		return Self::new(element, Content::Production(children));
	}

	pub fn new_expression(element: &'a Element, children: Vec<Node<'a, 'b>>) -> Self {
		return Self::new_production(&elements::PRODUCTION_EXPRESSION, vec![Node::new_production(element, children)]);
	}
}
