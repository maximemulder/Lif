use crate::elements;
use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::expression::Expression;
use crate::nodes::program::Program;
use crate::nodes::r#if::If;
use crate::nodes::r#loop::Loop;
use crate::nodes::r#while::While;
use crate::nodes::do_while::DoWhile;
use crate::nodes::for_in::ForIn;
use crate::nodes::statement::Statement;
use crate::nodes::statements::Statements;
use crate::nodes::structure::Structure;
use crate::nodes::operation::Operation;
use crate::nodes::sequence::Sequence;
use crate::nodes::declaration::Declaration;
use crate::nodes::function::Function;
use crate::nodes::block::Block;
use crate::nodes::group::Group;
use crate::nodes::integer::Integer;
use crate::nodes::identifier::Identifier;
use crate::nodes::string::String;
use crate::nodes::chain::Chain;
use crate::nodes::r#return::Return;
use crate::nodes::r#break::Break;
use crate::nodes::r#continue::Continue;

pub fn program(text: &str, node: &SyntaxNode) -> Program {
	return Program::new(statements(text, &node.children()[0]));
}

fn statements(text: &str, node: &SyntaxNode) -> Statements {
	let mut statements = Vec::new();
	for child in node.children() {
		statements.push(statement(text, child));
	}

	return Statements::new(statements);
}

fn statement(text: &str, node: &SyntaxNode) -> Statement {
	let child = &node.children()[0];
	return Statement::new(match child.element {
		&elements::expressions::EXPRESSION => Box::new(expression(text, child)),
		&elements::structures::STRUCTURE   => Box::new(structure(text, child)),
		_ => panic!(),
	});
}

fn expression(text: &str, node: &SyntaxNode) -> Expression {
	let child = &node.children()[0];
	return Expression::new(match child.element {
		&elements::expressions::LITERAL     => literal(text, child),
		&elements::structures::STRUCTURE    => Box::new(structure(text, child)),
		&elements::expressions::LET         => Box::new(r#let(text, child)),
		&elements::controls::CONTROL        => control(text, child),
		&elements::expressions::FUNCTION    => Box::new(function(text, child)),
		&elements::expressions::GROUP       => Box::new(group(text, child)),
		&elements::expressions::CHAIN       => Box::new(chain(text, child)),
		&elements::expressions::SEQUENCE    => Box::new(sequence(text, child)),
		&elements::expressions::OPERATION   => Box::new(operation(text, child)),
		_ => panic!(),
	});
}

fn literal(text: &str, node: &SyntaxNode) -> Box<dyn Node> {
	let child = &node.children()[0];
	return match child.element {
		&elements::variables::NUMBER     => Box::new(integer(text, child)),
		&elements::variables::STRING     => Box::new(string(text, child)),
		&elements::variables::IDENTIFIER => Box::new(identifier(text, child)),
		_ => panic!(),
	};
}

fn integer(text: &str, node: &SyntaxNode) -> Integer {
	return Integer::new(text[node.left()..node.right()].parse::<usize>().unwrap());
}

fn string(text: &str, node: &SyntaxNode) -> String {
	return String::new(Box::from(&text[node.left() + 1 .. node.right() - 1]));
}

fn identifier(text: &str, node: &SyntaxNode) -> Identifier {
	return Identifier::new(Box::from(&text[node.left()..node.right()]));
}

fn structure(text: &str, node: &SyntaxNode) -> Structure {	let child = &node.children()[0];
	return Structure::new(match child.element {
		&elements::structures::BLOCK    => Box::new(block(text, child)),
		&elements::structures::IF       => Box::new(r#if(text, child)),
		&elements::structures::LOOP     => Box::new(r#loop(text, child)),
		&elements::structures::WHILE    => Box::new(r#while(text, child)),
		&elements::structures::DO_WHILE => Box::new(do_while(text, child)),
		&elements::structures::FOR_IN   => Box::new(for_in(text, child)),
		_ => panic!(),
	});
}

fn block(text: &str, node: &SyntaxNode) -> Block {
	return Block::new(statements(text, &node.children()[1]), if node.children().len() == 4 {
		Some(expression(text, &node.children()[2]))
	} else {
		None
	});
}

fn r#if(text: &str, node: &SyntaxNode) -> If {
	return If::new(expression(text, &node.children()[1]), block(text, &node.children()[2]), node.children().get(4).map(|child| block(text, child)));
}

fn r#loop(text: &str, node: &SyntaxNode) -> Loop {
	return Loop::new(block(text, &node.children()[1]));
}

fn r#while(text: &str, node: &SyntaxNode) -> While {
	return While::new(expression(text, &node.children()[1]), block(text, &node.children()[2]));
}

fn do_while(text: &str, node: &SyntaxNode) -> DoWhile {
	return DoWhile::new(block(text, &node.children()[1]), expression(text, &node.children()[3]));
}

fn for_in(text: &str, node: &SyntaxNode) -> ForIn {
	return ForIn::new(token(text, &node.children()[1]), expression(text, &node.children()[3]), block(text, &node.children()[4]));
}

fn r#let(text: &str, node: &SyntaxNode) -> Declaration {
	return declaration(text, &node.children()[1]);
}

fn declaration(text: &str, node: &SyntaxNode) -> Declaration {
	return Declaration::new(token(text, &node.children()[0]), node.children().get(2).map(|child| expression(text, child)));
}

fn control(text: &str, node: &SyntaxNode) -> Box<dyn Node> {
	let child = &node.children()[0];
	return match child.element {
		&elements::controls::RETURN   => Box::new(r#return(text, child)),
		&elements::controls::BREAK    => Box::new(r#break(text, child)),
		&elements::controls::CONTINUE => Box::new(r#continue(text, child)),
		_ => panic!(),
	};
}

fn r#return(text: &str, node: &SyntaxNode) -> Return {
	return Return::new(node.children().get(1).map(|child| expression(text, child)));
}

fn r#break(text: &str, node: &SyntaxNode) -> Break {
	return Break::new(node.children().get(1).map(|child| expression(text, child)));
}

fn r#continue(text: &str, node: &SyntaxNode) -> Continue {
	return Continue::new(node.children().get(1).map(|child| expression(text, child)));
}

fn function(text: &str, node: &SyntaxNode) -> Function {
	return Function::new(parameters(text, &node.children()[2]), node.children().get(5).map(|child| expression(text, child)), block(text, &node.children().last().unwrap()));
}

fn parameters(text: &str, node: &SyntaxNode) -> Vec<Declaration> {
	let mut identifiers = Vec::new();
	for (i, child) in node.children().iter().enumerate()  {
		if i % 2 == 1 {
			continue;
		}

		identifiers.push(declaration(text, child));
	}

	return identifiers;
}

fn group(text: &str, node: &SyntaxNode) -> Group {
	return Group::new(expression(text, &node.children()[node.children().len() - 1]));
}

fn chain(text: &str, node: &SyntaxNode) -> Chain {
	return Chain::new(expression(text, &node.children()[0]), token(text, &node.children()[2]));
}

fn sequence(text: &str, node: &SyntaxNode) -> Sequence {
	return Sequence::new(expression(text, &node.children()[0]), token(text, &node.children()[1]), expressions(text, &node.children()[2]), token(text, &node.children()[3]));
}

fn expressions(text: &str, node: &SyntaxNode) -> Vec<Expression> {
	let mut expressions = Vec::new();
	for (i, child) in node.children().iter().enumerate()  {
		if i % 2 == 1 {
			continue;
		}

		expressions.push(expression(text, child));
	}

	return expressions;
}

fn operation(text: &str, node: &SyntaxNode) -> Operation {
	return Operation::new(expression(text, &node.children()[0]), expression(text, &node.children()[2]), token(text, &node.children()[1]));
}

fn token(text: &str, node: &SyntaxNode) -> Box<str> {
	return Box::from(&text[node.left()..node.right()]);
}
