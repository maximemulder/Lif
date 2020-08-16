use crate::elements;
use crate::runtime::{ Engine, Reference };
use super::{ Node, SyntaxNode };
use super::literal::literal;
use super::sequence::Sequence;
use super::structure::Structure;
use super::operation::Operation;
use super::function::Function;
use super::group::Group;
use super::declaration::Declaration;
use super::chain::Chain;

pub struct Expression {
	node: Box<dyn Node>,
}

impl Expression {
	pub fn build(node: &SyntaxNode) -> Expression {
		let child = &node.children()[0];
		return Expression {
			node: match child.element {
				&elements::expressions::LITERAL     => literal(child),
				&elements::structures::STRUCTURE    => Box::new(Structure::build(child)),
				&elements::expressions::FUNCTION    => Box::new(Function::build(child)),
				&elements::expressions::OPERATION   => Box::new(Operation::build(child)),
				&elements::expressions::SEQUENCE    => Box::new(Sequence::build(child)),
				&elements::expressions::GROUP       => Box::new(Group::build(child)),
				&elements::expressions::DECLARATION => Box::new(Declaration::build(child)),
				&elements::expressions::CHAIN       => Box::new(Chain::build(child)),
				_ => panic!(),
			},
		};
	}
}

impl Node for Expression {
	fn execute<'a>(&'a self, engine: &mut Engine<'a>) -> Reference {
		return self.node.execute(engine);
	}
}
