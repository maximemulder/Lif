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

pub fn program<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Program<'a, 'b> {
	return Program::new(node, statements(text, &node.children()[0]));
}

fn statements<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Statements<'a, 'b> {
	let mut statements = Vec::new();
	for child in node.children() {
		statements.push(statement(text, child));
	}

	return Statements::new(node, statements);
}

fn statement<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Statement<'a, 'b> {
	let child = &node.children()[0];
	return Statement::new(node, match child.element {
		&elements::expressions::EXPRESSION => Box::new(expression(text, child)),
		&elements::structures::STRUCTURE   => Box::new(structure(text, child)),
		_ => panic!(),
	});
}

fn expression<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Expression<'a, 'b> {
	let child = &node.children()[0];
	return Expression::new(node, match child.element {
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

fn literal<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Box<dyn Node + 'b> {
	let child = &node.children()[0];
	return match child.element {
		&elements::variables::NUMBER     => Box::new(integer(text, child)),
		&elements::variables::STRING     => Box::new(string(text, child)),
		&elements::variables::IDENTIFIER => Box::new(identifier(text, child)),
		_ => panic!(),
	};
}

fn integer<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Integer<'a, 'b> {
	return Integer::new(node, text[node.left() .. node.right()].parse::<usize>().unwrap());
}

fn string<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> String<'a, 'b> {
	return String::new(node, Box::from(&text[node.left() + 1 .. node.right() - 1]));
}

fn identifier<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Identifier<'a, 'b> {
	return Identifier::new(node, Box::from(&text[node.left() .. node.right()]));
}

fn structure<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Structure<'a, 'b> {	let child = &node.children()[0];
	return Structure::new(node, match child.element {
		&elements::structures::BLOCK    => Box::new(block(text, child)),
		&elements::structures::IF       => Box::new(r#if(text, child)),
		&elements::structures::LOOP     => Box::new(r#loop(text, child)),
		&elements::structures::WHILE    => Box::new(r#while(text, child)),
		&elements::structures::DO_WHILE => Box::new(do_while(text, child)),
		&elements::structures::FOR_IN   => Box::new(for_in(text, child)),
		_ => panic!(),
	});
}

fn block<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Block<'a, 'b> {
	return Block::new(node, statements(text, &node.children()[1]), if node.children().len() == 4 {
		Some(expression(text, &node.children()[2]))
	} else {
		None
	});
}

fn r#if<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> If<'a, 'b> {
	return If::new(node, expression(text, &node.children()[1]), block(text, &node.children()[2]), node.children().get(4).map(|child| block(text, child)));
}

fn r#loop<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Loop<'a, 'b> {
	return Loop::new(node, block(text, &node.children()[1]));
}

fn r#while<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> While<'a, 'b> {
	return While::new(node, expression(text, &node.children()[1]), block(text, &node.children()[2]));
}

fn do_while<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> DoWhile<'a, 'b> {
	return DoWhile::new(node, block(text, &node.children()[1]), expression(text, &node.children()[3]));
}

fn for_in<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> ForIn<'a, 'b> {
	return ForIn::new(node, token(text, &node.children()[1]), expression(text, &node.children()[3]), block(text, &node.children()[4]));
}

fn r#let<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Declaration<'a, 'b> {
	return declaration(text, &node.children()[1]);
}

fn declaration<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Declaration<'a, 'b> {
	return Declaration::new(node, token(text, &node.children()[0]), node.children().get(2).map(|child| expression(text, child)));
}

fn control<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Box<dyn Node + 'b> {
	let child = &node.children()[0];
	return match child.element {
		&elements::controls::RETURN   => Box::new(r#return(text, child)),
		&elements::controls::BREAK    => Box::new(r#break(text, child)),
		&elements::controls::CONTINUE => Box::new(r#continue(text, child)),
		_ => panic!(),
	};
}

fn r#return<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Return<'a, 'b> {
	return Return::new(node, node.children().get(1).map(|child| expression(text, child)));
}

fn r#break<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Break<'a, 'b> {
	return Break::new(node, node.children().get(1).map(|child| expression(text, child)));
}

fn r#continue<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Continue<'a, 'b> {
	return Continue::new(node, node.children().get(1).map(|child| expression(text, child)));
}

fn function<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Function<'a, 'b> {
	return Function::new(node, parameters(text, &node.children()[2]), node.children().get(5).map(|child| expression(text, child)), block(text, &node.children().last().unwrap()));
}

fn parameters<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Vec<Declaration<'a, 'b>> {
	let mut identifiers = Vec::new();
	for (i, child) in node.children().iter().enumerate()  {
		if i % 2 == 1 {
			continue;
		}

		identifiers.push(declaration(text, child));
	}

	return identifiers;
}

fn group<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Group<'a, 'b> {
	return Group::new(node, expression(text, &node.children()[node.children().len() - 1]));
}

fn chain<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Chain<'a, 'b> {
	return Chain::new(node, expression(text, &node.children()[0]), token(text, &node.children()[2]));
}

fn sequence<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Sequence<'a, 'b> {
	return Sequence::new(node, expression(text, &node.children()[0]), token(text, &node.children()[1]), expressions(text, &node.children()[2]), token(text, &node.children()[3]));
}

fn expressions<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Vec<Expression<'a, 'b>> {
	let mut expressions = Vec::new();
	for (i, child) in node.children().iter().enumerate()  {
		if i % 2 == 1 {
			continue;
		}

		expressions.push(expression(text, child));
	}

	return expressions;
}

fn operation<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Operation<'a, 'b> {
	return Operation::new(node, expression(text, &node.children()[0]), expression(text, &node.children()[2]), token(text, &node.children()[1]));
}

fn token<'a, 'b>(text: &str, node: &'b SyntaxNode<'a>) -> Box<str> {
	return Box::from(&text[node.left() .. node.right()]);
}
