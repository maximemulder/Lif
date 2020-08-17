use crate::elements;
use super::{ Node, SyntaxNode };
use super::expression::Expression;
use super::program::Program;
use super::r#if::If;
use super::r#loop::Loop;
use super::r#while::While;
use super::do_while::DoWhile;
use super::for_in::ForIn;
use super::statement::Statement;
use super::statements::Statements;
use super::structure::Structure;
use super::operation::Operation;
use super::sequence::Sequence;
use super::declaration::Declaration;
use super::function::Function;
use super::block::Block;
use super::group::Group;
use super::integer::Integer;
use super::identifier::Identifier;
use super::string::String;
use super::chain::Chain;
use super::r#return::Return;
use super::r#break::Break;
use super::r#continue::Continue;

pub fn program(node: &SyntaxNode) -> Program {
	return Program::new(statements(&node.children()[0]));
}

fn statements(node: &SyntaxNode) -> Statements {
	let mut statements = Vec::new();
	for child in node.children() {
		statements.push(statement(child));
	}

	return Statements::new(statements);
}

fn statement(node: &SyntaxNode) -> Statement {
	return Statement::new(expression(&node.children()[0]));
}

fn expression(node: &SyntaxNode) -> Expression {
	let child = &node.children()[0];
	return Expression::new(match child.element {
		&elements::expressions::LITERAL     => literal(child),
		&elements::structures::STRUCTURE    => Box::new(structure(child)),
		&elements::expressions::DECLARATION => Box::new(declaration(child)),
		&elements::controls::CONTROL        => control(child),
		&elements::expressions::FUNCTION    => Box::new(function(child)),
		&elements::expressions::GROUP       => Box::new(group(child)),
		&elements::expressions::CHAIN       => Box::new(chain(child)),
		&elements::expressions::SEQUENCE    => Box::new(sequence(child)),
		&elements::expressions::OPERATION   => Box::new(operation(child)),
		_ => panic!(),
	});
}

fn literal(node: &SyntaxNode) -> Box<dyn Node> {
	let child = &node.children()[0];
	return match child.element {
		&elements::variables::NUMBER     => Box::new(integer(child)),
		&elements::variables::STRING     => Box::new(string(child)),
		&elements::variables::IDENTIFIER => Box::new(identifier(child)),
		_ => panic!(),
	};
}

fn integer(node: &SyntaxNode) -> Integer {
	return Integer::new(node.text().parse::<usize>().unwrap());
}

fn string(node: &SyntaxNode) -> String {
	let text = node.text();
	return String::new(Box::from(&text[1 .. text.len() - 1]));
}

fn identifier(node: &SyntaxNode) -> Identifier {
	return Identifier::new(Box::from(node.text()));
}

fn structure(node: &SyntaxNode) -> Structure {	let child = &node.children()[0];
	return Structure::new(match child.element {
		&elements::structures::BLOCK    => Box::new(block(child)),
		&elements::structures::IF       => Box::new(r#if(child)),
		&elements::structures::LOOP     => Box::new(r#loop(child)),
		&elements::structures::WHILE    => Box::new(r#while(child)),
		&elements::structures::DO_WHILE => Box::new(do_while(child)),
		&elements::structures::FOR_IN   => Box::new(for_in(child)),
		_ => panic!(),
	});
}

fn block(node: &SyntaxNode) -> Block {
	return Block::new(statements(&node.children()[1]), if node.children().len() == 4 {
		Some(expression(&node.children()[2]))
	} else {
		None
	});
}

fn r#if(node: &SyntaxNode) -> If {
	return If::new(expression(&node.children()[0]), then(&node.children()[1]), if let Some(child) = node.children().get(2) {
		Some(r#else(child))
	} else {
		None
	});
}

fn then(node: &SyntaxNode) -> Expression {
	return if node.children().len() == 1 {
		Expression::new(Box::new(block(&node.children()[0])))
	} else {
		expression(&node.children()[1])
	};
}

fn r#else(node: &SyntaxNode) -> Expression {
	return expression(&node.children()[1]);
}

fn r#do(node: &SyntaxNode) -> Expression {
	return if node.children().len() == 1 {
		Expression::new(Box::new(block(&node.children()[0])))
	} else {
		expression(&node.children()[1])
	};
}

fn r#loop(node: &SyntaxNode) -> Loop {
	return Loop::new(expression(&node.children()[1]));
}

fn r#while(node: &SyntaxNode) -> While {
	return While::new(expression(&node.children()[1]), r#do(&node.children()[2]));
}

fn do_while(node: &SyntaxNode) -> DoWhile {
	return DoWhile::new(expression(&node.children()[1]), expression(&node.children()[3]));
}

fn for_in(node: &SyntaxNode) -> ForIn {
	return ForIn::new(token(&node.children()[1]), expression(&node.children()[3]), r#do(&node.children()[4]));
}

fn declaration(node: &SyntaxNode) -> Declaration {
	return Declaration::new(token(&node.children()[1]));
}

fn control(node: &SyntaxNode) -> Box<dyn Node> {
	let child = &node.children()[0];
	return match child.element {
		&elements::controls::RETURN   => Box::new(r#return(child)),
		&elements::controls::BREAK    => Box::new(r#break(child)),
		&elements::controls::CONTINUE => Box::new(r#continue(child)),
		_ => panic!(),
	};
}

fn r#return(node: &SyntaxNode) -> Return {
	return Return::new(if let Some(child) = node.children().get(1) {
		Some(expression(child))
	} else {
		None
	});
}

fn r#break(node: &SyntaxNode) -> Break {
	return Break::new(if let Some(child) = node.children().get(1) {
		Some(expression(child))
	} else {
		None
	});
}

fn r#continue(node: &SyntaxNode) -> Continue {
	return Continue::new(if let Some(child) = node.children().get(1) {
		Some(expression(child))
	} else {
		None
	});
}

fn function(node: &SyntaxNode) -> Function {
	return Function::new(parameters(&node.children()[2]), block(&node.children()[4]));
}

fn parameters(node: &SyntaxNode) -> Vec<Box<str>> {
	let mut identifiers = Vec::new();
	for (i, child) in node.children().iter().enumerate()  {
		if i % 2 == 1 {
			continue;
		}

		identifiers.push(token(child));
	}

	return identifiers;
}

fn group(node: &SyntaxNode) -> Group {
	return Group::new(expression(&node.children()[node.children().len() - 1]));
}

fn chain(node: &SyntaxNode) -> Chain {
	return Chain::new(expression(&node.children()[0]), token(&node.children()[2]));
}

fn sequence(node: &SyntaxNode) -> Sequence {
	return Sequence::new(expression(&node.children()[0]), token(&node.children()[1]), expressions(&node.children()[2]), token(&node.children()[3]));
}

fn expressions(node: &SyntaxNode) -> Vec<Expression> {
	let mut expressions = Vec::new();
	for (i, child) in node.children().iter().enumerate()  {
		if i % 2 == 1 {
			continue;
		}

		expressions.push(expression(child));
	}

	return expressions;
}

fn operation(node: &SyntaxNode) -> Operation {
	return Operation::new(expression(&node.children()[0]), expression(&node.children()[2]), token(&node.children()[1]));
}

fn token(node: &SyntaxNode) -> Box<str> {
	return Box::from(node.text());
}
